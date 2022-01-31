use crate::{
    core::{context::Context, convert::TryFromInput},
    types::Update,
};
use std::{error::Error, future::Future, sync::Arc};

/// Allows to handle an update
pub trait Handler<I>: Clone + Send
where
    I: TryFromInput,
{
    /// A future output returned by `handle` method
    ///
    /// You should use [HandlerResult](enum.HandlerResult.html)
    /// (or any type, which can be converted into it)
    /// if you want to use that handler in [App](struct.App.html)
    ///
    /// It is possible to use any other type, e.g. if you want to use it in a decorator.
    /// But finally you need to convert it into `HandlerResult`.
    type Output: Send;

    /// A future returned by `handle` method
    type Future: Future<Output = Self::Output> + Send;

    /// Handles a specific input
    ///
    /// # Arguments
    ///
    /// * input - An input to handle
    ///
    /// See [TryFromInput](trait.TryFromInput.html) trait implementations
    /// for a list of supported types
    fn handle(&self, input: I) -> Self::Future;
}

macro_rules! impl_fn {
    ($($I:ident),+) => {
        #[allow(non_snake_case)]
        impl<X, $($I,)+ R> Handler<($($I,)+)> for X
        where
            X: Fn($($I,)+) -> R + Clone + Send + Sync,
            ($($I,)+): TryFromInput,
            R: Future + Send,
            R::Output: Send
        {
            type Output = R::Output;
            type Future = R;

            fn handle(&self, ($($I,)+): ($($I,)+)) -> Self::Future {
                (self)($($I,)+)
            }
        }
    };
}

impl_fn!(A);
impl_fn!(A, B);
impl_fn!(A, B, C);
impl_fn!(A, B, C, D);
impl_fn!(A, B, C, D, E);
impl_fn!(A, B, C, D, E, F);
impl_fn!(A, B, C, D, E, F, G);
impl_fn!(A, B, C, D, E, F, G, H);
impl_fn!(A, B, C, D, E, F, G, H, I);
impl_fn!(A, B, C, D, E, F, G, H, I, J);

/// An input for a handler
#[derive(Clone, Debug)]
pub struct HandlerInput {
    /// An Update received from Telegram API
    pub update: Update,
    /// A context to share data betweeen handlers
    pub context: Arc<Context>,
}

impl From<Update> for HandlerInput {
    fn from(update: Update) -> Self {
        HandlerInput {
            update,
            context: Arc::new(Default::default()),
        }
    }
}

/// An error returned by a handler
pub type HandlerError = Box<dyn Error + Send>;

/// A result returned by a handler
#[derive(Debug)]
pub enum HandlerResult {
    /// Success
    Ok,
    /// Contains the error
    Err(HandlerError),
}

impl From<()> for HandlerResult {
    fn from(_: ()) -> Self {
        HandlerResult::Ok
    }
}

impl<T, E> From<Result<T, E>> for HandlerResult
where
    T: Into<HandlerResult>,
    E: Error + Send + 'static,
{
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(res) => res.into(),
            Err(err) => HandlerResult::Err(Box::new(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    #[test]
    fn convert_input() {
        let update: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "message": {
                    "message_id": 1111,
                    "date": 0,
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "private", "first_name": "test"},
                    "text": "test",
                }
            }
        ))
        .unwrap();
        assert_eq!(HandlerInput::from(update).update.id, 1);
    }

    #[derive(Debug)]
    struct ExampleError;

    impl Error for ExampleError {}

    impl fmt::Display for ExampleError {
        fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(out, "Example error")
        }
    }

    #[test]
    fn convert() {
        assert!(matches!(HandlerResult::from(()), HandlerResult::Ok));
        assert!(matches!(
            HandlerResult::from(Ok::<(), ExampleError>(())),
            HandlerResult::Ok
        ));
        assert!(matches!(
            HandlerResult::from(Err::<(), ExampleError>(ExampleError)),
            HandlerResult::Err(_)
        ));
    }
}
