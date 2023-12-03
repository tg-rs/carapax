use std::{future::Future, marker::PhantomData};

use futures_util::future::BoxFuture;

use crate::core::{
    convert::TryFromInput,
    handler::{Handler, HandlerError, HandlerInput, HandlerResult, IntoHandlerResult},
};

#[cfg(test)]
mod tests;

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
