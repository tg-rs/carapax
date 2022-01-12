use crate::core::{Handler, HandlerResult, TryFromInput};
use futures_util::future::BoxFuture;
use std::{error::Error, marker::PhantomData};

/// A predicate decorator
///
/// Wraps a handler with a predicate which allows to decide should handler run or not.
///
/// Predicate must return [PredicateResult](enum.PredicateResult.html)
#[derive(Clone)]
pub struct Predicate<P, PI, H, HI> {
    predicate: P,
    predicate_input: PhantomData<PI>,
    handler: H,
    handler_input: PhantomData<HI>,
}

impl<P, PI, H, HI> Predicate<P, PI, H, HI> {
    /// Creates a new decorator
    ///
    /// # Arguments
    ///
    /// * predicate - A predicate handler
    /// * handler - A handler to wrap
    pub fn new(predicate: P, handler: H) -> Self {
        Self {
            predicate,
            predicate_input: PhantomData,
            handler,
            handler_input: PhantomData,
        }
    }
}

impl<P, PI, H, HI> Handler<(PI, HI)> for Predicate<P, PI, H, HI>
where
    P: Handler<PI> + Clone + 'static,
    P::Output: Into<PredicateResult>,
    PI: TryFromInput + 'static,
    PI::Error: 'static,
    H: Handler<HI> + Clone + 'static,
    H::Output: Into<HandlerResult>,
    HI: TryFromInput + 'static,
    HI::Error: 'static,
{
    type Output = HandlerResult;
    type Future = BoxFuture<'static, Self::Output>;

    fn handle(&self, (predicate_input, handler_input): (PI, HI)) -> Self::Future {
        let predicate = self.predicate.clone();
        let handler = self.handler.clone();
        Box::pin(async move {
            let predicate_future = predicate.handle(predicate_input);
            match predicate_future.await.into() {
                PredicateResult::True => {
                    let handler_future = handler.handle(handler_input);
                    handler_future.await.into()
                }
                PredicateResult::False(result) => result,
            }
        })
    }
}

/// A predicate result
#[derive(Debug)]
pub enum PredicateResult {
    /// Decorated handler will run
    True,
    /// Decorated handler will not run
    ///
    /// `HandlerResult` allows to decide, will next handler run or not.
    False(HandlerResult),
}

impl From<bool> for PredicateResult {
    fn from(value: bool) -> Self {
        if value {
            PredicateResult::True
        } else {
            PredicateResult::False(HandlerResult::Continue)
        }
    }
}

impl<T, E> From<Result<T, E>> for PredicateResult
where
    T: Into<PredicateResult>,
    E: Error + Send + 'static,
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(value) => value.into(),
            Err(err) => PredicateResult::False(HandlerResult::Error(Box::new(err))),
        }
    }
}

/// Predicate shortcuts
pub trait PredicateExt<P, PI, HI>: Sized {
    /// Shortcut to create a new predicate decorator (`handler.predicate(predicate)`)
    ///
    /// # Arguments
    ///
    /// * predicate - A predicate handler
    fn predicate(self, predicate: P) -> Predicate<P, PI, Self, HI> {
        Predicate::new(predicate, self)
    }
}

impl<P, PI, H, HI> PredicateExt<P, PI, HI> for H
where
    H: Handler<HI>,
    HI: TryFromInput,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Integer, User};
    use std::{error::Error, fmt};

    #[tokio::test]
    async fn decorator() {
        let handler = Predicate::new(has_access, process_user);
        let user_1 = create_user(1);
        let user_2 = create_user(2);
        let user_3 = create_user(3);
        assert!(matches!(
            handler.handle(((user_1.clone(),), (user_1,))).await,
            HandlerResult::Continue
        ));
        assert!(matches!(
            handler.handle(((user_2.clone(),), (user_2,))).await,
            HandlerResult::Stop
        ));
        assert!(matches!(
            handler.handle(((user_3.clone(),), (user_3,))).await,
            HandlerResult::Error(_)
        ));
    }

    fn create_user(id: Integer) -> User {
        User {
            first_name: format!("test #{}", id),
            id,
            is_bot: false,
            last_name: None,
            language_code: None,
            username: None,
        }
    }

    async fn has_access(user: User) -> PredicateResult {
        if user.id != 2 {
            PredicateResult::True
        } else {
            PredicateResult::False(HandlerResult::Stop)
        }
    }

    async fn process_user(user: User) -> Result<HandlerResult, ProcessError> {
        log::info!("Processing user: {:?}", user);
        if user.id == 3 {
            Err(ProcessError)
        } else {
            Ok(HandlerResult::Continue)
        }
    }

    #[derive(Debug)]
    struct ProcessError;

    impl Error for ProcessError {}

    impl fmt::Display for ProcessError {
        fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(out, "Process error")
        }
    }

    #[test]
    fn convert_result() {
        assert!(matches!(true.into(), PredicateResult::True));
        assert!(matches!(false.into(), PredicateResult::False(HandlerResult::Continue)));
        assert!(matches!(Ok::<bool, ProcessError>(true).into(), PredicateResult::True));
        assert!(matches!(
            Ok::<bool, ProcessError>(false).into(),
            PredicateResult::False(HandlerResult::Continue)
        ));
        assert!(matches!(
            Err::<bool, ProcessError>(ProcessError).into(),
            PredicateResult::False(HandlerResult::Error(_))
        ));
    }
}
