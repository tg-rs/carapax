use crate::core::{
    convert::TryFromInput,
    handler::{Handler, HandlerResult},
    predicate::result::PredicateResult,
};
use futures_util::future::BoxFuture;
use std::marker::PhantomData;

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
    PI: TryFromInput + Clone + 'static,
    PI::Error: 'static,
    H: Handler<HI> + Clone + 'static,
    H::Output: Into<HandlerResult>,
    HI: TryFromInput + Clone + 'static,
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
}
