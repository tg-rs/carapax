use crate::{
    core::{context::Context, convert::TryFromInput},
    types::Update,
};
use std::{error::Error, fmt, future::Future, sync::Arc};

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
// this type is needed because `dyn ...` is not Sized
// this is required by
// https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#impl-From%3CE%3E
pub struct HandlerError(Box<dyn Error + Send>);

impl HandlerError {
    /// Returns error in a box
    pub fn boxed<E>(err: E) -> Self
    where
        E: Error + Send + 'static,
    {
        Self(Box::new(err))
    }
}

impl fmt::Debug for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for HandlerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}

/// A result returned by a handler
pub type HandlerResult = Result<(), HandlerError>;

/// Converts objects into HandlerResult
pub trait IntoHandlerResult {
    /// Returns converted object
    fn into_handler_result(self) -> HandlerResult;
}

impl IntoHandlerResult for () {
    fn into_handler_result(self) -> HandlerResult {
        Ok(self)
    }
}

impl<T, E> IntoHandlerResult for Result<T, E>
where
    T: IntoHandlerResult,
    E: Error + Send + 'static,
{
    fn into_handler_result(self) -> HandlerResult {
        match self {
            Ok(ok) => ok.into_handler_result(),
            Err(err) => Err(HandlerError::boxed(err)),
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
        assert!(matches!(().into_handler_result(), Ok(())));
        assert!(matches!(Ok::<(), ExampleError>(()).into_handler_result(), Ok(())));
        assert!(matches!(
            Err::<(), ExampleError>(ExampleError).into_handler_result(),
            HandlerResult::Err(_)
        ));
    }
}
