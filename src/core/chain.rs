use crate::{
    core::{
        convert::TryFromInput,
        handler::{Handler, HandlerInput, HandlerResult},
    },
    HandlerError, IntoHandlerResult,
};
use futures_util::future::BoxFuture;
use std::{any::type_name, future::Future, marker::PhantomData, sync::Arc};

/// Handlers chain
#[derive(Clone, Default)]
pub struct Chain {
    handlers: Arc<Vec<Box<dyn InputHandler + Sync>>>,
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
    pub fn add<H, I>(mut self, handler: H) -> Self
    where
        H: Handler<I> + Sync + Clone + 'static,
        I: TryFromInput + Sync + 'static,
        <H::Future as Future>::Output: IntoHandlerResult,
    {
        let handlers = Arc::get_mut(&mut self.handlers).expect("Can not add handler, chain is shared");
        handlers.push(ConvertInputHandler::boxed(handler));
        self
    }

    fn run(&self, input: HandlerInput) -> impl Future<Output = HandlerResult> {
        let handlers = self.handlers.clone();
        async move {
            for handler in handlers.iter() {
                let type_name = handler.get_type_name();
                log::debug!("Running '{}' handler...", type_name);
                let result = handler.handle(input.clone()).await;
                if let Err(err) = result {
                    log::debug!("'{}' handler returned {}, loop stopped", type_name, err);
                    return Err(err);
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

trait InputHandler: Send {
    fn handle(&self, input: HandlerInput) -> BoxFuture<'static, HandlerResult>;

    fn get_type_name(&self) -> &'static str {
        type_name::<Self>()
    }
}

#[derive(Clone)]
struct ConvertInputHandler<H, I> {
    handler: H,
    input: PhantomData<I>,
}

impl<H, I> ConvertInputHandler<H, I> {
    pub(in crate::core) fn boxed(handler: H) -> Box<Self> {
        Box::new(Self {
            handler,
            input: PhantomData,
        })
    }
}

impl<H, I> InputHandler for ConvertInputHandler<H, I>
where
    H: Handler<I> + 'static,
    I: TryFromInput,
    I::Error: 'static,
    <H::Future as Future>::Output: IntoHandlerResult,
{
    fn handle(&self, input: HandlerInput) -> BoxFuture<'static, HandlerResult> {
        let handler = self.handler.clone();
        Box::pin(async move {
            match I::try_from_input(input).await {
                Ok(Some(input)) => {
                    let future = handler.handle(input);
                    future.await.into_result()
                }
                Ok(None) => Ok(()),
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

    async fn handler_ok(store: Ref<UpdateStore>, update: Update) -> HandlerResult {
        store.push(update).await;
        Ok(())
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

        let result = assert_handle!(2, handler_ok, handler_error);
        assert!(matches!(result, Err(_)));

        let result = assert_handle!(1, handler_error, handler_ok);
        assert!(matches!(result, Err(_)));

        let result = assert_handle!(2, handler_ok, handler_ok);
        assert!(matches!(result, Ok(())));
    }
}
