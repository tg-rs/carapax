use crate::{
    context::Context,
    handler::{Handler, HandlerFuture, HandlerResult},
};
use failure::Error;
use futures::{task, Async, Future, Poll};
use std::sync::Arc;
use tgbot::{types::Update, Api, UpdateHandler};

/// Defines how to deal with errors in handlers
#[derive(Debug, Clone, Copy)]
pub enum ErrorStrategy {
    /// Ignore any error in a handler or middleware and write it to log
    Ignore,
    /// Return first error, all next handlers or middlewares will not run
    Abort,
}

pub(crate) struct Dispatcher {
    api: Api,
    handlers: Arc<Vec<Handler>>,
    error_strategy: ErrorStrategy,
}

impl Dispatcher {
    pub(crate) fn new(api: Api, handlers: Vec<Handler>, error_strategy: ErrorStrategy) -> Self {
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

#[must_use = "futures do nothing unless polled"]
pub(crate) struct DispatcherFuture {
    handlers: Arc<Vec<Handler>>,
    context: Option<Context>,
    error_strategy: ErrorStrategy,
    update: Update,
    handler_idx: usize,
    handler: Option<HandlerFuture>,
}

impl DispatcherFuture {
    fn new(
        handlers: Arc<Vec<Handler>>,
        context: Context,
        error_strategy: ErrorStrategy,
        update: Update,
    ) -> DispatcherFuture {
        DispatcherFuture {
            handlers,
            context: Some(context),
            error_strategy,
            update,
            handler_idx: 0,
            handler: None,
        }
    }
}

impl DispatcherFuture {
    fn take_context(&mut self) -> Context {
        self.context.take().expect("Surprise! Context lost...")
    }
}

impl Future for DispatcherFuture {
    type Item = Context;
    type Error = (Error, Context);

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.handler {
            Some(ref mut f) => match f.poll() {
                Ok(Async::Ready(HandlerResult::Continue)) => {
                    self.handler_idx += 1;
                    self.handler = None;
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Ok(Async::Ready(HandlerResult::Stop)) => Ok(Async::Ready(self.take_context())),
                Ok(Async::NotReady) => {
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Err(err) => match self.error_strategy {
                    ErrorStrategy::Abort => Err((err, self.take_context())),
                    ErrorStrategy::Ignore => {
                        log::error!("An error has occurred in after middleware: {:?}", err);
                        self.handler_idx += 1;
                        self.handler = None;
                        task::current().notify();
                        Ok(Async::NotReady)
                    }
                },
            },
            None => match self.handlers.get(self.handler_idx) {
                Some(handler) => {
                    self.handler = Some(handler.handle(
                        match self.context {
                            Some(ref mut context) => context,
                            None => panic!("Suprise! Context lost..."),
                        },
                        &self.update,
                    ));
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                None => Ok(Async::Ready(self.take_context())),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_value, json};
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

    fn setup_context(context: &mut Context, _update: &Update) -> HandlerFuture {
        context.set(Counter::new());
        HandlerResult::Continue.into()
    }

    #[derive(Debug, Fail)]
    #[fail(display = "Test error")]
    struct ErrorMock;

    fn handle_update_ok(context: &mut Context, _update: &Update) -> HandlerFuture {
        context.get::<Counter>().inc_calls();
        HandlerResult::Continue.into()
    }

    fn handle_update_err(context: &mut Context, _update: &Update) -> HandlerFuture {
        context.get::<Counter>().inc_calls();
        Err(ErrorMock).into()
    }

    #[test]
    fn test_error_strategy() {
        let update: Update = from_value(json!(
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
                Handler::update(setup_context),
                Handler::update(handle_update_err),
                Handler::update(handle_update_ok),
            ],
            ErrorStrategy::Abort,
        );
        let (_err, context) = dispatcher.dispatch(update.clone()).wait().unwrap_err();
        assert_eq!(context.get::<Counter>().get_calls(), 1);

        // Ignored
        let dispatcher = Dispatcher::new(
            Api::new("token").unwrap(),
            vec![
                Handler::update(setup_context),
                Handler::update(handle_update_err),
                Handler::update(handle_update_ok),
            ],
            ErrorStrategy::Ignore,
        );
        let context = dispatcher.dispatch(update.clone()).wait().unwrap();
        assert_eq!(context.get::<Counter>().get_calls(), 2);
    }
}
