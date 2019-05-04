use crate::{
    context::Context,
    handler::{BoxedHandler, HandlerFuture, HandlerResult},
};
use failure::Error;
use futures::{Async, Future, Poll};
use std::sync::Arc;
use tgbot::{types::Update, Api, UpdateHandler};

/// Defines how to deal with errors in handlers
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorStrategy {
    /// Ignore any error in a handler and write it to log
    Ignore,
    /// Return first error, all next handlers will not run
    Abort,
}

pub(crate) struct Dispatcher {
    api: Api,
    handlers: Arc<Vec<BoxedHandler>>,
    error_strategy: ErrorStrategy,
}

impl Dispatcher {
    pub(crate) fn new(api: Api, handlers: Vec<BoxedHandler>, error_strategy: ErrorStrategy) -> Self {
        Self {
            api,
            handlers: Arc::new(handlers),
            error_strategy,
        }
    }

    pub(crate) fn dispatch(&self, update: Update) -> DispatcherFuture {
        let mut context = Context::default();
        context.set(self.api.clone());
        DispatcherFuture::new(self.handlers.clone(), context, self.error_strategy, update)
    }
}

impl UpdateHandler for Dispatcher {
    fn handle(&mut self, update: Update) {
        tokio_executor::spawn(self.dispatch(update).then(|r| {
            if let Err((e, _context)) = r {
                log::error!("Failed to dispatch update: {:?}", e);
            }
            Ok(())
        }));
    }
}

struct HandlersQueue {
    handlers: Arc<Vec<BoxedHandler>>,
    current: usize,
}

impl HandlersQueue {
    fn new(handlers: Arc<Vec<BoxedHandler>>) -> Self {
        HandlersQueue { handlers, current: 0 }
    }

    fn next(&mut self) -> Option<&BoxedHandler> {
        let handler = self.handlers.get(self.current);
        self.current += 1;
        handler
    }
}

#[must_use = "futures do nothing unless polled"]
pub(crate) struct DispatcherFuture {
    handlers: HandlersQueue,
    context: Option<Context>,
    error_strategy: ErrorStrategy,
    update: Update,
    handler: Option<HandlerFuture>,
}

impl DispatcherFuture {
    fn new(
        handlers: Arc<Vec<BoxedHandler>>,
        context: Context,
        error_strategy: ErrorStrategy,
        update: Update,
    ) -> DispatcherFuture {
        let mut fut = DispatcherFuture {
            handlers: HandlersQueue::new(handlers),
            context: Some(context),
            error_strategy,
            update,
            handler: None,
        };
        fut.switch_to_next_handler();
        fut
    }
}

impl DispatcherFuture {
    fn take_context(&mut self) -> Context {
        self.context.take().expect("Polled after completion")
    }

    fn switch_to_next_handler(&mut self) {
        let update = self.update.clone();
        let ctx = self
            .context
            .as_mut()
            .expect("No context found when switching to a next handler");
        self.handler = self.handlers.next().map(|handler| handler.handle(ctx, update));
    }

    fn stop(&mut self) {
        self.handler = None;
    }
}

impl Future for DispatcherFuture {
    type Item = Context;
    type Error = (Error, Context);

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let handler = match self.handler.as_mut() {
                Some(handler) => handler,
                None => return Ok(Async::Ready(self.take_context())),
            };
            match handler.poll() {
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Ok(Async::Ready(HandlerResult::Continue)) => self.switch_to_next_handler(),
                Ok(Async::Ready(HandlerResult::Stop)) => self.stop(),
                Err(err) => match self.error_strategy {
                    ErrorStrategy::Abort => return Err((err, self.take_context())),
                    ErrorStrategy::Ignore => {
                        log::warn!("An error has occurred in a handler: {:?}", err);
                        self.switch_to_next_handler();
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler::{FnHandler, HandlerWrapper};
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    struct Counter {
        calls: Arc<AtomicUsize>,
    }

    impl Counter {
        fn new() -> Self {
            Self {
                calls: Arc::new(AtomicUsize::new(0)),
            }
        }

        fn inc_calls(&self) {
            self.calls.fetch_add(1, Ordering::SeqCst);
        }

        fn get_calls(&self) -> usize {
            self.calls.load(Ordering::SeqCst)
        }
    }

    fn setup_context(context: &mut Context, _update: Update) {
        context.set(Counter::new());
    }

    #[derive(Debug, Fail)]
    #[fail(display = "Test error")]
    struct ErrorMock;

    fn handle_update_continue(context: &mut Context, _update: Update) {
        context.get::<Counter>().inc_calls();
    }

    fn handle_update_stop(context: &mut Context, _update: Update) -> HandlerResult {
        context.get::<Counter>().inc_calls();
        HandlerResult::Stop
    }

    fn handle_update_err(context: &mut Context, _update: Update) -> Result<HandlerResult, ErrorMock> {
        context.get::<Counter>().inc_calls();
        Err(ErrorMock)
    }

    #[test]
    fn error_strategy() {
        let update: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "message": {
                    "message_id": 1111,
                    "date": 0,
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "private", "first_name": "test"},
                    "text": "test"
                }
            }
        ))
        .unwrap();

        // Aborted
        let dispatcher = Dispatcher::new(
            Api::new("token").unwrap(),
            vec![
                HandlerWrapper::boxed(FnHandler::from(setup_context)),
                HandlerWrapper::boxed(FnHandler::from(handle_update_err)),
                HandlerWrapper::boxed(FnHandler::from(handle_update_continue)),
            ],
            ErrorStrategy::Abort,
        );
        let (_err, context) = dispatcher.dispatch(update.clone()).wait().unwrap_err();
        assert_eq!(context.get::<Counter>().get_calls(), 1);

        // Ignored
        let dispatcher = Dispatcher::new(
            Api::new("token").unwrap(),
            vec![
                HandlerWrapper::boxed(FnHandler::from(setup_context)),
                HandlerWrapper::boxed(FnHandler::from(handle_update_err)),
                HandlerWrapper::boxed(FnHandler::from(handle_update_continue)),
            ],
            ErrorStrategy::Ignore,
        );
        let context = dispatcher.dispatch(update.clone()).wait().unwrap();
        assert_eq!(context.get::<Counter>().get_calls(), 2);
    }

    #[test]
    fn handler_stopped() {
        let update: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "message": {
                    "message_id": 1111,
                    "date": 0,
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "private", "first_name": "test"},
                    "text": "test"
                }
            }
        ))
        .unwrap();

        let dispatcher = Dispatcher::new(
            Api::new("token").unwrap(),
            vec![
                HandlerWrapper::boxed(FnHandler::from(setup_context)),
                HandlerWrapper::boxed(FnHandler::from(handle_update_stop)),
                HandlerWrapper::boxed(FnHandler::from(handle_update_continue)),
            ],
            ErrorStrategy::Abort,
        );
        let context = dispatcher.dispatch(update.clone()).wait().unwrap();
        assert_eq!(context.get::<Counter>().get_calls(), 1);
    }
}
