use std::{future::Future, marker::PhantomData};

use crate::core::{
    convert::TryFromInput,
    handler::{Handler, HandlerError, HandlerInput, HandlerResult, IntoHandlerResult},
};

#[cfg(test)]
mod tests;

/// Allows to process an error returned by a handler.
pub struct ErrorDecorator<E, H, HI> {
    error_handler: E,
    handler: H,
    handler_input: PhantomData<HI>,
}

impl<E, H, HI> ErrorDecorator<E, H, HI> {
    /// Creates a new `ErrorDecorator`.
    ///
    /// # Arguments
    ///
    /// * `error_handler` - A handler for errors returned by a decorated handler.
    /// * `handler` - The handler to be decorated.
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
    E: ErrorHandler + Clone + Sync + 'static,
    H: Handler<HI> + Sync + 'static,
    HI: TryFromInput + Sync,
    HI::Error: 'static,
    H::Output: IntoHandlerResult,
{
    type Output = HandlerResult;

    async fn handle(&self, input: HandlerInput) -> Self::Output {
        let future = HI::try_from_input(input);
        match future.await {
            Ok(Some(input)) => match self.handler.handle(input).await.into_result() {
                Err(err) => Err(self.error_handler.handle(err).await),
                result => result,
            },
            Ok(None) => Ok(()),
            Err(err) => Err(self.error_handler.handle(HandlerError::new(err)).await),
        }
    }
}

/// Allows to process errors returned by handlers.
pub trait ErrorHandler: Send {
    /// Handles a errors.
    ///
    /// # Arguments
    ///
    /// * `err` - An error to handle.
    fn handle(&self, err: HandlerError) -> impl Future<Output = HandlerError> + Send;
}

impl<H, F> ErrorHandler for H
where
    H: Fn(HandlerError) -> F + Send + Sync,
    F: Future<Output = HandlerError> + Send,
{
    async fn handle(&self, err: HandlerError) -> HandlerError {
        (self)(err).await
    }
}

/// Provides a shortcut for creating error decorator.
pub trait ErrorExt<E, HI>: Sized {
    /// A shortcut to create a new error decorator.
    ///
    /// Example: `handler.on_error(error_handler)`
    ///
    /// # Arguments
    ///
    /// * `error_handler` - An error handler.
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
