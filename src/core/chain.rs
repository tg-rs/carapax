use crate::core::{
    convert::TryFromInput,
    handler::{Handler, HandlerError, HandlerInput, HandlerResult},
};
use futures_util::future::BoxFuture;
use std::{any::type_name, error::Error, future::Future, marker::PhantomData, sync::Arc};

/// Handlers chain
#[derive(Clone, Default)]
pub struct Chain {
    handlers: Arc<Vec<Box<dyn ChainHandler + Sync>>>,
}

impl Chain {
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
    #[allow(clippy::should_implement_trait)]
    pub fn add<H, I, O>(mut self, handler: H) -> Self
    where
        H: Handler<I, Output = O> + Sync + Clone + 'static,
        I: TryFromInput + Sync + 'static,
        O: IntoChainResult,
    {
        let handlers = Arc::get_mut(&mut self.handlers).expect("Can not add handler, chain is shared");
        handlers.push(ConvertHandler::boxed(handler));
        self
    }

    fn run(&self, input: HandlerInput) -> impl Future<Output = HandlerResult> {
        let handlers = self.handlers.clone();
        async move {
            for handler in handlers.iter() {
                let type_name = handler.get_type_name();
                log::debug!("Running '{}' handler...", type_name);
                let result = handler.handle(input.clone()).await;
                match result {
                    Ok(LoopResult::Continue) => {
                        log::debug!("'{}' handler returned {:?}, continue", type_name, LoopResult::Continue);
                    }
                    Ok(LoopResult::Stop) => {
                        log::debug!("'{}' handler returned {:?}, stop", type_name, LoopResult::Stop);
                        break;
                    }
                    Err(err) => {
                        log::debug!("'{}' handler returned {}, stop", type_name, err);
                        return Err(err);
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

trait ChainHandler: Send {
    fn handle(&self, input: HandlerInput) -> BoxFuture<'static, ChainResult>;

    fn get_type_name(&self) -> &'static str {
        type_name::<Self>()
    }
}

/// A specialized result for chain handler
pub type ChainResult = Result<LoopResult, HandlerError>;

/// Allows to control loop in chain
#[derive(Clone, Copy, Debug)]
pub enum LoopResult {
    /// Next handler will run
    Continue,
    /// Next handler will not run
    Stop,
}

impl From<()> for LoopResult {
    fn from(_: ()) -> Self {
        LoopResult::Stop
    }
}

/// Converts objects into ChainResult
pub trait IntoChainResult {
    /// Performs conversion
    fn into_result(self) -> ChainResult;
}

impl<T> IntoChainResult for T
where
    T: Into<LoopResult>,
{
    fn into_result(self) -> ChainResult {
        Ok(self.into())
    }
}

impl<T, E> IntoChainResult for Result<T, E>
where
    T: Into<LoopResult>,
    E: Error + Send + 'static,
{
    fn into_result(self) -> ChainResult {
        self.map(Into::into).map_err(HandlerError::new)
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
    R: IntoChainResult,
{
    fn handle(&self, input: HandlerInput) -> BoxFuture<'static, Result<LoopResult, HandlerError>> {
        let handler = self.handler.clone();
        Box::pin(async move {
            match I::try_from_input(input).await {
                Ok(Some(input)) => {
                    let future = handler.handle(input);
                    future.await.into_result()
                }
                Ok(None) => Ok(LoopResult::Continue),
                Err(err) => Err(HandlerError::new(err)),
            }
        })
    }

    fn get_type_name(&self) -> &'static str {
        type_name::<H>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::context::{Context, Ref},
        types::Update,
    };
    use std::{error::Error, fmt};
    use tokio::sync::Mutex;

    #[derive(Clone)]
    struct UpdateStore(Arc<Mutex<Vec<Update>>>);

    impl UpdateStore {
        fn new() -> Self {
            Self(Arc::new(Mutex::new(Vec::new())))
        }

        async fn push(&self, update: Update) {
            self.0.lock().await.push(update)
        }

        async fn count(&self) -> usize {
            self.0.lock().await.len()
        }
    }

    async fn handler_continue(store: Ref<UpdateStore>, update: Update) -> LoopResult {
        store.push(update).await;
        LoopResult::Continue
    }

    async fn handler_stop(store: Ref<UpdateStore>, update: Update) -> LoopResult {
        store.push(update).await;
        LoopResult::Stop
    }

    async fn handler_error(store: Ref<UpdateStore>, update: Update) -> HandlerResult {
        store.push(update).await;
        Err(HandlerError::new(ErrorMock))
    }

    #[derive(Debug)]
    struct ErrorMock;

    impl Error for ErrorMock {}

    impl fmt::Display for ErrorMock {
        fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
            write!(out, "Test error")
        }
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
    async fn chain() {
        macro_rules! assert_handle {
            ($count:expr, $($handler:expr),*) => {{
                let mut context = Context::default();
                context.insert(UpdateStore::new());
                let context = Arc::new(context);
                let mut chain = Chain::default();
                $(chain = chain.add($handler);)*
                let update = create_update();
                let input = HandlerInput {
                    context: context.clone(),
                    update
                };
                let result = chain.handle(input).await;
                let count = context.get::<UpdateStore>().unwrap().count().await;
                assert_eq!(count, $count);
                result
            }};
        }

        let result = assert_handle!(2, handler_continue, handler_error, handler_stop);
        assert!(matches!(result, Err(_)));

        let result = assert_handle!(1, handler_error, handler_continue);
        assert!(matches!(result, Err(_)));

        let result = assert_handle!(1, handler_stop, handler_continue);
        assert!(matches!(result, Ok(())));
    }
}
