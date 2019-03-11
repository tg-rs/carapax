use failure::Error;
use futures::{future, Future, Poll};
use tgbot::types::Update;

/// Result of a middleware
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MiddlewareResult {
    /// Continue propagation
    ///
    /// Next middleware and all handlers (if exists) will run after current has finished
    Continue,
    /// Stop propagation
    ///
    /// Next middleware and all handlers (if exists) will not run after current has finished
    Stop,
}

/// A middleware future
#[must_use = "futures do nothing unless polled"]
pub struct MiddlewareFuture {
    inner: Box<Future<Item = MiddlewareResult, Error = Error> + Send>,
}

impl MiddlewareFuture {
    /// Creates a new middleware future
    pub fn new<F>(f: F) -> MiddlewareFuture
    where
        F: Future<Item = MiddlewareResult, Error = Error> + Send + 'static,
    {
        MiddlewareFuture { inner: Box::new(f) }
    }
}

impl<E> From<Result<MiddlewareResult, E>> for MiddlewareFuture
where
    E: Into<Error>,
{
    fn from(result: Result<MiddlewareResult, E>) -> Self {
        MiddlewareFuture::new(future::result(result.map_err(Into::into)))
    }
}

impl From<MiddlewareResult> for MiddlewareFuture {
    fn from(result: MiddlewareResult) -> MiddlewareFuture {
        MiddlewareFuture::new(future::ok(result))
    }
}

impl Future for MiddlewareFuture {
    type Item = MiddlewareResult;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

/// Middleware handler
pub trait Middleware<C> {
    /// Called before all handlers
    fn before(&mut self, _context: &mut C, _update: &Update) -> MiddlewareFuture {
        MiddlewareResult::Continue.into()
    }

    /// Called after all handlers
    fn after(&mut self, _context: &mut C, _update: &Update) -> MiddlewareFuture {
        MiddlewareResult::Continue.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        dispatcher::Dispatcher,
        handler::{Handler, HandlerFuture},
    };
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    use tgbot::types::Message;

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

    fn parse_update(data: &str) -> Update {
        serde_json::from_str(data).unwrap()
    }

    struct MockMiddleware {
        before_result: MiddlewareResult,
        after_result: MiddlewareResult,
    }

    impl Middleware<Counter> for MockMiddleware {
        fn before(&mut self, context: &mut Counter, _update: &Update) -> MiddlewareFuture {
            context.inc_calls();
            self.before_result.into()
        }

        fn after(&mut self, context: &mut Counter, _update: &Update) -> MiddlewareFuture {
            context.inc_calls();
            self.after_result.into()
        }
    }

    fn handle_message(context: &mut Counter, _message: &Message) -> HandlerFuture {
        context.inc_calls();
        ().into()
    }

    #[test]
    fn test_middleware() {
        let update = parse_update(
            r#"{
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test"
            }
        }"#,
        );

        let mut dispatcher = Dispatcher::new(
            vec![
                Box::new(MockMiddleware {
                    before_result: MiddlewareResult::Continue,
                    after_result: MiddlewareResult::Continue,
                }),
                Box::new(MockMiddleware {
                    before_result: MiddlewareResult::Stop,
                    after_result: MiddlewareResult::Continue,
                }),
                Box::new(MockMiddleware {
                    before_result: MiddlewareResult::Continue,
                    after_result: MiddlewareResult::Stop,
                }),
                Box::new(MockMiddleware {
                    before_result: MiddlewareResult::Continue,
                    after_result: MiddlewareResult::Continue,
                }),
            ],
            vec![Handler::message(handle_message)],
            Counter::new(),
            Default::default(),
            Default::default(),
        );
        dispatcher.dispatch(update.clone()).wait().unwrap();
        assert_eq!(dispatcher.context.lock().unwrap().get_calls(), 5);

        let mut dispatcher = Dispatcher::new(
            vec![
                Box::new(MockMiddleware {
                    before_result: MiddlewareResult::Continue,
                    after_result: MiddlewareResult::Stop,
                }),
                Box::new(MockMiddleware {
                    before_result: MiddlewareResult::Continue,
                    after_result: MiddlewareResult::Continue,
                }),
            ],
            vec![Handler::message(handle_message)],
            Counter::new(),
            Default::default(),
            Default::default(),
        );
        dispatcher.dispatch(update).wait().unwrap();
        assert_eq!(dispatcher.context.lock().unwrap().get_calls(), 4);
    }
}
