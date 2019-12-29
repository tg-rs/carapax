use crate::handler::{ConvertHandler, Handler, HandlerError, HandlerResult};
use async_trait::async_trait;
use std::sync::Arc;
use tgbot::{types::Update, UpdateHandler};
use tokio::sync::Mutex;

/// A Telegram Update dispatcher
pub struct Dispatcher<C> {
    handlers: Vec<Box<dyn Handler<C, Input = Update, Output = HandlerResult> + Send>>,
    context: Arc<Mutex<C>>,
    error_handler: Option<Box<dyn ErrorHandler + Send>>,
}

impl<C> Dispatcher<C>
where
    C: Send + 'static,
{
    /// Creates a new Dispatcher
    ///
    /// # Arguments
    ///
    /// * context - Context passed to each handler
    pub fn new(context: C) -> Self {
        Self {
            context: Arc::new(Mutex::new(context)),
            handlers: Vec::new(),
            error_handler: None,
        }
    }

    /// Add a handler to dispatcher
    ///
    /// Handlers will be dispatched in the same order as they are added
    pub fn add_handler<H>(&mut self, handler: H)
    where
        H: Handler<C> + Send + 'static,
    {
        self.handlers.push(ConvertHandler::boxed(handler))
    }

    pub(crate) async fn dispatch(&mut self, update: Update) {
        let context = self.context.clone();
        let mut context = context.lock().await;
        for handler in &mut self.handlers {
            let result = handler.handle(&mut context, update.clone()).await;
            match result {
                HandlerResult::Continue => { /* noop */ }
                HandlerResult::Stop => {
                    break;
                }
                HandlerResult::Error(err) => match &mut self.error_handler {
                    Some(handler) => {
                        if handler.handle(err).await {
                            break;
                        }
                    }
                    None => {
                        log::error!("An error has occurred: {}", err);
                        break;
                    }
                },
            }
        }
    }
}

#[async_trait]
impl<C> UpdateHandler for Dispatcher<C>
where
    C: Send + 'static,
{
    async fn handle(&mut self, update: Update) {
        self.dispatch(update).await
    }
}

/// A handler for errors occurred when dispatching update
#[async_trait]
pub trait ErrorHandler {
    /// Handles a error
    ///
    /// Return `true` if you need to stop update propagation.
    /// Otherwise update will be passed to a next handler.
    async fn handle(&mut self, err: HandlerError) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{error::Error, fmt};

    type Updates = Vec<Update>;

    struct HandlerMock {
        result: Option<HandlerResult>,
    }

    impl HandlerMock {
        fn new(result: HandlerResult) -> Self {
            Self { result: Some(result) }
        }

        fn with_continue() -> Self {
            Self::new(HandlerResult::Continue)
        }

        fn with_stop() -> Self {
            Self::new(HandlerResult::Stop)
        }

        fn with_error() -> Self {
            Self::new(HandlerResult::from(Err::<(), ErrorMock>(ErrorMock)))
        }
    }

    #[async_trait]
    impl Handler<Updates> for HandlerMock {
        type Input = Update;
        type Output = HandlerResult;

        async fn handle(&mut self, context: &mut Updates, input: Self::Input) -> Self::Output {
            context.push(input);
            self.result.take().unwrap()
        }
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
    async fn dispatch() {
        macro_rules! assert_dispatch {
            ($count:expr, $($handler:expr),*) => {{
                let updates = Updates::new();
                let mut dispatcher = Dispatcher::new(updates);
                $(dispatcher.add_handler($handler);)*
                let update = create_update();
                let result = dispatcher.dispatch(update).await;
                let context = dispatcher.context.lock().await;
                assert_eq!(context.len(), $count);
                result
            }};
        }

        assert_dispatch!(
            2,
            HandlerMock::with_continue(),
            HandlerMock::with_stop(),
            HandlerMock::with_error()
        );

        assert_dispatch!(
            1,
            HandlerMock::with_stop(),
            HandlerMock::with_continue(),
            HandlerMock::with_error()
        );

        assert_dispatch!(
            1,
            HandlerMock::with_error(),
            HandlerMock::with_stop(),
            HandlerMock::with_continue()
        );
    }
}
