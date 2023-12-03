use std::marker::PhantomData;

use futures_util::future::BoxFuture;

use crate::core::{
    convert::TryFromInput,
    handler::{Handler, HandlerError, HandlerResult, IntoHandlerResult},
    predicate::result::PredicateResult,
};

#[cfg(test)]
mod tests;

/// A predicate decorator
///
/// Wraps a handler with a predicate which allows to decide should handler run or not.
///
/// Predicate must return [PredicateResult](enum.PredicateResult.html)
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
    P: Handler<PI> + 'static,
    P::Output: Into<PredicateResult>,
    PI: TryFromInput + 'static,
    PI::Error: 'static,
    H: Handler<HI> + 'static,
    H::Output: IntoHandlerResult,
    HI: TryFromInput + 'static,
    HI::Error: 'static,
{
    type Output = PredicateOutput;
    type Future = BoxFuture<'static, Self::Output>;

    fn handle(&self, (predicate_input, handler_input): (PI, HI)) -> Self::Future {
        let predicate = self.predicate.clone();
        let handler = self.handler.clone();
        Box::pin(async move {
            let predicate_future = predicate.handle(predicate_input);
            let predicate_result = predicate_future.await.into();
            if let PredicateResult::True = predicate_result {
                let handler_future = handler.handle(handler_input);
                handler_future.await.into_result().into()
            } else {
                predicate_result.into()
            }
        })
    }
}

impl<P, PI, H, HI> Clone for Predicate<P, PI, H, HI>
where
    P: Clone,
    H: Clone,
{
    fn clone(&self) -> Self {
        Predicate {
            predicate: self.predicate.clone(),
            predicate_input: self.predicate_input,
            handler: self.handler.clone(),
            handler_input: self.handler_input,
        }
    }
}

/// Output of the predicate decorator
pub enum PredicateOutput {
    /// Decorated handler has run
    True(HandlerResult),
    /// Decorated handler has not run
    False,
    /// An error has occurred in predicate
    Err(HandlerError),
}

impl From<PredicateResult> for PredicateOutput {
    fn from(result: PredicateResult) -> Self {
        match result {
            PredicateResult::True => PredicateOutput::True(Ok(())),
            PredicateResult::False => PredicateOutput::False,
            PredicateResult::Err(err) => PredicateOutput::Err(err),
        }
    }
}

impl From<HandlerResult> for PredicateOutput {
    fn from(result: HandlerResult) -> Self {
        PredicateOutput::True(result)
    }
}

impl IntoHandlerResult for PredicateOutput {
    fn into_result(self) -> HandlerResult {
        match self {
            PredicateOutput::True(result) => result,
            PredicateOutput::False => Ok(()),
            PredicateOutput::Err(err) => Err(err),
        }
    }
}
