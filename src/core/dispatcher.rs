use crate::{
    core::{
        context::Context,
        convert::TryFromInput,
        handler::base::{
            BoxedErrorHandler, BoxedHandler, ConvertErrorHandler, ConvertHandler, ErrorHandler, Handler, HandlerInput,
            HandlerResult, LoggingErrorHandler,
        },
    },
    types::Update,
    UpdateHandler,
};
use futures_util::future::BoxFuture;
use std::{future::Future, sync::Arc};

/// Updates dispatcher
///
/// Implements [UpdateHandler](trait.UpdateHandler.html) trait, so you can use it
/// in [LongPoll](longpoll/struct.LongPoll.html) struct
/// or [webhook::run_server](webhook/fn.run_server.html) function.
pub struct Dispatcher {
    context: Arc<Context>,
    handlers: Vec<BoxedHandler>,
    error_handler: Arc<BoxedErrorHandler>,
}

impl Dispatcher {
    /// Creates a new Dispatcher
    ///
    /// # Arguments
    ///
    /// * context - A context to share data between handlers
    pub fn new(context: Context) -> Self {
        Self {
            context: Arc::new(context),
            handlers: Vec::new(),
            error_handler: Arc::new(Box::new(LoggingErrorHandler)),
        }
    }

    /// Adds a handler to dispatcher
    ///
    /// # Arguments
    ///
    /// * handler - A handler to add
    ///
    /// Handlers will be dispatched in the same order as they are added
    pub fn add_handler<H, I>(&mut self, handler: H) -> &mut Self
    where
        H: Handler<I> + Sync + Clone + 'static,
        I: TryFromInput + 'static,
        <H::Future as Future>::Output: Into<HandlerResult>,
    {
        self.handlers.push(ConvertHandler::boxed(handler));
        self
    }

    /// Sets a handler to be executed when an error has occurred
    ///
    /// # Arguments
    ///
    /// * handler - A handler to set
    ///
    /// Error handler will be called if one of update handlers returned
    /// [HandlerResult::Error](enum.HandlerResult.html)
    ///
    /// If this method is not called,
    /// [LoggingErrorHandler](struct.LoggingErrorHandler.html)
    /// will be used as default handler.
    pub fn set_error_handler<H>(&mut self, handler: H) -> &mut Self
    where
        H: ErrorHandler + Sync + 'static,
    {
        self.error_handler = Arc::new(ConvertErrorHandler::boxed(handler));
        self
    }

    fn dispatch(&self, update: Update) -> impl Future<Output = ()> {
        let input = HandlerInput {
            update,
            context: self.context.clone(),
        };
        let futures: Vec<_> = self.handlers.iter().map(|h| h.handle(input.clone())).collect();
        let error_handler = self.error_handler.clone();
        async move {
            for future in futures {
                let result = future.await;
                match result {
                    HandlerResult::Continue => {}
                    HandlerResult::Stop => break,
                    HandlerResult::Error(err) => {
                        error_handler.handle(err).await;
                        break;
                    }
                }
            }
        }
    }
}

impl UpdateHandler for Dispatcher {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        Box::pin(self.dispatch(update))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{context::Ref, handler::HandlerError};
    use futures_util::future::FutureExt;
    use std::{error::Error, fmt};
    use tokio::sync::{
        mpsc::{channel, Sender},
        Mutex,
    };

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

    async fn handler_continue(store: Ref<UpdateStore>, update: Update) -> HandlerResult {
        store.push(update).await;
        HandlerResult::Continue
    }

    async fn handler_stop(store: Ref<UpdateStore>, update: Update) -> HandlerResult {
        store.push(update).await;
        HandlerResult::Stop
    }

    async fn handler_error(store: Ref<UpdateStore>, update: Update) -> HandlerResult {
        store.push(update).await;
        HandlerResult::from(Err::<(), ErrorMock>(ErrorMock))
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
    async fn dispatch_default() {
        macro_rules! assert_dispatch {
            ($count:expr, $($handler:expr),*) => {{
                let mut context = Context::default();
                context.insert(UpdateStore::new());
                let mut dispatcher = Dispatcher::new(context);
                $(dispatcher.add_handler($handler);)*
                let update = create_update();
                dispatcher.dispatch(update).await;
                let count = dispatcher.context.get::<UpdateStore>().unwrap().count().await;
                assert_eq!(count, $count);
            }};
        }
        assert_dispatch!(2, handler_continue, handler_stop, handler_error);
        assert_dispatch!(1, handler_stop, handler_continue, handler_error);
        assert_dispatch!(1, handler_error, handler_stop, handler_continue);
    }

    struct MockErrorHandler {
        sender: Arc<Sender<HandlerError>>,
    }

    impl MockErrorHandler {
        fn new(sender: Sender<HandlerError>) -> Self {
            MockErrorHandler {
                sender: Arc::new(sender),
            }
        }
    }

    impl ErrorHandler for MockErrorHandler {
        type Future = BoxFuture<'static, ()>;

        fn handle(&self, err: HandlerError) -> Self::Future {
            let sender = self.sender.clone();
            Box::pin(async move {
                sender.send(err).await.unwrap();
            })
        }
    }

    #[tokio::test]
    async fn dispatch_custom_error_handler() {
        let update = create_update();
        let mut context = Context::default();
        context.insert(UpdateStore::new());
        let mut dispatcher = Dispatcher::new(context);
        dispatcher.add_handler(handler_error);
        dispatcher.add_handler(handler_continue);
        let (tx, mut rx) = channel(1);
        dispatcher.set_error_handler(MockErrorHandler::new(tx));
        dispatcher.dispatch(update.clone()).await;
        rx.close();
        let count = dispatcher.context.get::<UpdateStore>().unwrap().count().await;
        assert_eq!(count, 1);
        assert!(rx.recv().now_or_never().is_some());
    }
}
