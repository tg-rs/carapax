use crate::{
    core::{
        context::Context,
        dispatcher::Dispatcher,
        handler::{Handler, HandlerError, HandlerInput, HandlerResult},
    },
    types::Update,
    UpdateHandler,
};
use futures_util::future::BoxFuture;
use std::{future::Future, sync::Arc};

/// The main entry point
///
/// Implements [UpdateHandler](trait.UpdateHandler.html) trait, so you can use it
/// in [LongPoll](longpoll/struct.LongPoll.html) struct
/// or [webhook::run_server](webhook/fn.run_server.html) function.
pub struct App {
    context: Arc<Context>,
    dispatcher: Dispatcher,
    error_handler: Arc<BoxedErrorHandler>,
}

impl App {
    /// Creates a new App
    ///
    /// # Arguments
    ///
    /// * context - A context to share data between handlers
    /// * dispatcher - A dispatcher with update handlers
    pub fn new(context: Context, dispatcher: Dispatcher) -> Self {
        Self {
            context: Arc::new(context),
            dispatcher,
            error_handler: Arc::new(Box::new(LoggingErrorHandler)),
        }
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

    fn run(&self, update: Update) -> impl Future<Output = ()> {
        let future = self.dispatcher.handle(HandlerInput {
            update,
            context: self.context.clone(),
        });
        let error_handler = self.error_handler.clone();
        async move {
            if let HandlerResult::Error(err) = future.await {
                error_handler.handle(err).await;
            }
        }
    }
}

impl UpdateHandler for App {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        Box::pin(self.run(update))
    }
}

/// Allows to process errors returned by handlers
pub trait ErrorHandler: Send {
    /// A future returned by `handle` method
    type Future: Future<Output = ()> + Send;

    /// Handles a errors
    ///
    /// # Arguments
    ///
    /// * err - An error to handle
    fn handle(&self, err: HandlerError) -> Self::Future;
}

type BoxedErrorHandler = Box<dyn ErrorHandler<Future = BoxFuture<'static, ()>> + Sync>;

struct ConvertErrorHandler<H>(H);

impl<H> ConvertErrorHandler<H> {
    pub(in crate::core) fn boxed(handler: H) -> Box<Self> {
        Box::new(Self(handler))
    }
}

impl<H> ErrorHandler for ConvertErrorHandler<H>
where
    H: ErrorHandler,
    H::Future: 'static,
{
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, err: HandlerError) -> Self::Future {
        Box::pin(self.0.handle(err))
    }
}

/// Writes an error to log
pub struct LoggingErrorHandler;

impl ErrorHandler for LoggingErrorHandler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, err: HandlerError) -> Self::Future {
        Box::pin(async move {
            log::error!("An error has occurred: {}", err);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{context::Ref, dispatcher::DispatcherBuilder};
    use std::{error::Error, fmt};
    use tokio::sync::Mutex;

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
        type Future = BoxFuture<'static, ()>;

        fn handle(&self, _err: HandlerError) -> Self::Future {
            let value = self.value.clone();
            Box::pin(async move {
                *value.lock().await = true;
            })
        }
    }

    async fn success_handler(condition: Ref<Condition>) {
        *condition.value.lock().await = true;
    }

    async fn error_handler(_: ()) -> Result<(), ExampleError> {
        Err(ExampleError)
    }

    #[tokio::test]
    async fn handle_ok() {
        let condition = Condition {
            value: Arc::new(Mutex::new(false)),
        };

        let mut context = Context::default();
        context.insert(condition.clone());

        let mut builder = DispatcherBuilder::default();
        builder.add_handler(success_handler);

        let dispatcher = builder.build();
        let app = App::new(context, dispatcher);

        let update = create_update();
        app.handle(update).await;

        assert!(*condition.value.lock().await);
    }

    #[tokio::test]
    async fn handle_err() {
        let condition = Condition {
            value: Arc::new(Mutex::new(false)),
        };

        let context = Context::default();
        let mut builder = DispatcherBuilder::default();
        builder.add_handler(error_handler);

        let dispatcher = builder.build();
        let mut app = App::new(context, dispatcher);
        app.set_error_handler(condition.clone());

        let update = create_update();
        app.handle(update).await;

        assert!(*condition.value.lock().await);
    }
}
