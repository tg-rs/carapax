use std::{any::type_name, error::Error, future::Future, marker::PhantomData, sync::Arc};

use futures_util::future::BoxFuture;

use crate::core::{
    convert::TryFromInput,
    handler::{Handler, HandlerError, HandlerInput, HandlerResult, IntoHandlerResult},
    predicate::PredicateOutput,
};

#[cfg(test)]
mod tests;

/// Handlers chain
#[derive(Clone)]
pub struct Chain {
    handlers: Arc<Vec<Box<dyn ChainHandler + Sync>>>,
    strategy: ChainStrategy,
}

impl Chain {
    fn new(strategy: ChainStrategy) -> Self {
        Self {
            handlers: Arc::new(Vec::new()),
            strategy,
        }
    }

    /// Creates a new chain
    ///
    /// Only a first handler found for given input will run.
    /// Use `all()` method if you want to run all handlers.
    pub fn once() -> Self {
        Self::new(ChainStrategy::FirstFound)
    }

    /// Creates a new chain
    ///
    /// Runs all given handlers
    pub fn all() -> Self {
        Self::new(ChainStrategy::All)
    }

    /// Adds a handler
    ///
    /// # Arguments
    ///
    /// * handler - Handler to add
    ///
    /// Handlers will be dispatched in the same order as they are added.
    ///
    /// If a handler returns an error, subsequent handlers will not run.
    ///
    /// # Panics
    ///
    /// Panics when trying to add a handler to a shared chain.
    pub fn with<H, I, O>(mut self, handler: H) -> Self
    where
        H: Handler<I, Output = O> + Sync + Clone + 'static,
        I: TryFromInput + Sync + 'static,
        O: Into<ChainResult>,
    {
        let handlers = Arc::get_mut(&mut self.handlers).expect("Can not add handler, chain is shared");
        handlers.push(ConvertHandler::boxed(handler));
        self
    }

    fn run(&self, input: HandlerInput) -> impl Future<Output = HandlerResult> {
        let handlers = self.handlers.clone();
        let strategy = self.strategy;
        async move {
            for handler in handlers.iter() {
                let type_name = handler.get_type_name();
                log::debug!("Running '{}' handler...", type_name);
                let result = handler.handle(input.clone()).await;
                match result {
                    ChainResult::Done(result) => match strategy {
                        ChainStrategy::All => match result {
                            Ok(()) => {
                                log::debug!("[CONTINUE] Handler '{}' succeeded", type_name);
                            }
                            Err(err) => {
                                log::debug!("[STOP] Handler '{}' returned an error: {}", type_name, err);
                                return Err(err);
                            }
                        },
                        ChainStrategy::FirstFound => {
                            log::debug!("[STOP] First found handler: '{}'", type_name);
                            return result;
                        }
                    },
                    ChainResult::Err(err) => {
                        log::debug!("[STOP] Could not convert input for '{}' handler: {}", type_name, err);
                        return Err(err);
                    }
                    ChainResult::Skipped => {
                        log::debug!("[CONTINUE] Input not found for '{}' handler", type_name);
                    }
                }
            }
            Ok(())
        }
    }
}

impl Handler<HandlerInput> for Chain {
    type Output = HandlerResult;
    type Future = BoxFuture<'static, Self::Output>;

    fn handle(&self, input: HandlerInput) -> Self::Future {
        Box::pin(self.run(input))
    }
}

#[derive(Clone, Copy)]
enum ChainStrategy {
    All,
    FirstFound,
}

trait ChainHandler: Send {
    fn handle(&self, input: HandlerInput) -> BoxFuture<'static, ChainResult>;

    fn get_type_name(&self) -> &'static str {
        type_name::<Self>()
    }
}

/// A specialized result for Chain
pub enum ChainResult {
    /// Handler has run
    Done(HandlerResult),
    /// An error has occurred before handler execution
    Err(HandlerError),
    /// Handler has not run
    Skipped,
}

impl From<()> for ChainResult {
    fn from(_: ()) -> Self {
        ChainResult::Done(Ok(()))
    }
}

impl<E> From<Result<(), E>> for ChainResult
where
    E: Error + Send + 'static,
{
    fn from(result: Result<(), E>) -> Self {
        ChainResult::Done(result.map_err(HandlerError::new))
    }
}

impl From<PredicateOutput> for ChainResult {
    fn from(output: PredicateOutput) -> Self {
        match output {
            PredicateOutput::True(result) => ChainResult::Done(result),
            PredicateOutput::False => ChainResult::Skipped,
            PredicateOutput::Err(err) => ChainResult::Err(err),
        }
    }
}

impl IntoHandlerResult for ChainResult {
    fn into_result(self) -> HandlerResult {
        match self {
            ChainResult::Done(result) => result,
            ChainResult::Err(err) => Err(err),
            ChainResult::Skipped => Ok(()),
        }
    }
}

#[derive(Clone)]
struct ConvertHandler<H, I> {
    handler: H,
    input: PhantomData<I>,
}

impl<H, I> ConvertHandler<H, I> {
    pub(in crate::core) fn boxed(handler: H) -> Box<Self> {
        Box::new(Self {
            handler,
            input: PhantomData,
        })
    }
}

impl<H, I, R> ChainHandler for ConvertHandler<H, I>
where
    H: Handler<I, Output = R> + 'static,
    I: TryFromInput,
    I::Error: 'static,
    R: Into<ChainResult>,
{
    fn handle(&self, input: HandlerInput) -> BoxFuture<'static, ChainResult> {
        let handler = self.handler.clone();
        Box::pin(async move {
            match I::try_from_input(input).await {
                Ok(Some(input)) => {
                    let future = handler.handle(input);
                    future.await.into()
                }
                Ok(None) => ChainResult::Skipped,
                Err(err) => ChainResult::Err(HandlerError::new(err)),
            }
        })
    }

    fn get_type_name(&self) -> &'static str {
        type_name::<H>()
    }
}
