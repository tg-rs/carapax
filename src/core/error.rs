use crate::{
    core::{
        convert::TryFromInput,
        handler::{Handler, HandlerError, HandlerInput, HandlerResult},
    },
    IntoHandlerResult,
};
use futures_util::future::BoxFuture;
use std::{future::Future, marker::PhantomData};

/// Allows to process an error returned by a handler
pub struct ErrorDecorator<E, H, HI> {
    error_handler: E,
    handler: H,
    handler_input: PhantomData<HI>,
}

impl<E, H, HI> ErrorDecorator<E, H, HI> {
    /// Creates a new ErrorDecorator
    ///
    /// # Arguments
    ///
    /// * error_handler - A error handler
    /// * handler - A handler to decorate
    pub fn new(error_handler: E, handler: H) -> Self {
        Self {
            error_handler,
            handler,
            handler_input: PhantomData,
        }
    }
}

impl<E, H, HI> Clone for ErrorDecorator<E, H, HI>
where
    E: Clone,
    H: Clone,
{
    fn clone(&self) -> Self {
        Self {
            error_handler: self.error_handler.clone(),
            handler: self.handler.clone(),
            handler_input: PhantomData,
        }
    }
}

impl<E, H, HI> Handler<HandlerInput> for ErrorDecorator<E, H, HI>
where
    E: ErrorHandler + Clone + 'static,
    H: Handler<HI> + 'static,
    HI: TryFromInput,
    HI::Error: 'static,
    H::Output: IntoHandlerResult,
{
    type Output = HandlerResult;
    type Future = BoxFuture<'static, Self::Output>;

    fn handle(&self, input: HandlerInput) -> Self::Future {
        let handler = self.handler.clone();
        let error_handler = self.error_handler.clone();
        Box::pin(async move {
            let future = HI::try_from_input(input);
            match future.await {
                Ok(Some(input)) => {
                    let future = handler.handle(input);
                    match future.await.into_result() {
                        Err(err) => {
                            let future = error_handler.handle(err);
                            Err(future.await)
                        }
                        result => result,
                    }
                }
                Ok(None) => Ok(()),
                Err(err) => {
                    let future = error_handler.handle(HandlerError::new(err));
                    Err(future.await)
                }
            }
        })
    }
}

/// Allows to process errors returned by handlers
pub trait ErrorHandler: Send {
    /// A future returned by `handle` method
    type Future: Future<Output = HandlerError> + Send;

    /// Handles a errors
    ///
    /// # Arguments
    ///
    /// * err - An error to handle
    ///
    /// You need to return the error in order to pass it to subsequent handler
    fn handle(&self, err: HandlerError) -> Self::Future;
}

impl<H, F> ErrorHandler for H
where
    H: Fn(HandlerError) -> F + Send,
    F: Future<Output = HandlerError> + Send,
{
    type Future = F;

    fn handle(&self, err: HandlerError) -> Self::Future {
        (self)(err)
    }
}

/// Error decorator shortcuts
pub trait ErrorExt<E, HI>: Sized {
    /// Shortcut to create a new error decorator (`handler.error(error_handler)`)
    ///
    /// # Arguments
    ///
    /// * error_handler - An error handler
    fn on_error(self, error_handler: E) -> ErrorDecorator<E, Self, HI> {
        ErrorDecorator::new(error_handler, self)
    }
}

impl<E, H, HI> ErrorExt<E, HI> for H
where
    H: Handler<HI>,
    HI: TryFromInput,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{core::handler::HandlerInput, types::Update};
    use std::{error::Error, fmt, sync::Arc};
    use tokio::sync::Mutex;

    #[derive(Clone)]
    struct Condition {
        value: Arc<Mutex<bool>>,
    }

    #[derive(Debug)]
    struct ExampleError;

    impl fmt::Display for ExampleError {
        fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
            write!(out, "Example error")
        }
    }

    impl Error for ExampleError {}

    impl ErrorHandler for Condition {
        type Future = BoxFuture<'static, HandlerError>;

        fn handle(&self, err: HandlerError) -> Self::Future {
            let value = self.value.clone();
            Box::pin(async move {
                *value.lock().await = true;
                err
            })
        }
    }

    async fn handler(_: ()) -> Result<(), ExampleError> {
        Err(ExampleError)
    }

    fn create_update() -> Update {
        serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test message from private chat"
            }
        }))
        .unwrap()
    }

    #[tokio::test]
    async fn error_decorator() {
        let condition = Condition {
            value: Arc::new(Mutex::new(false)),
        };
        let handler = ErrorDecorator::new(condition.clone(), handler);
        let update = create_update();
        let input = HandlerInput::from(update);
        let result = handler.handle(input).await;
        assert!(matches!(result, Err(_)));
        assert!(*condition.value.lock().await)
    }
}
