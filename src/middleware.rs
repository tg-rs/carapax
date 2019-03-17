use crate::context::Context;
use failure::Error;
use futures::{future, Future, Poll};
use tgbot::types::Update;

/// Result of a middleware
pub enum MiddlewareResult {
    /// Continue propagation
    ///
    /// Next middleware and all handlers (if exists) will run after current has finished
    Continue(Context),
    /// Stop propagation
    ///
    /// Next middleware and all handlers (if exists) will not run after current has finished
    Stop(Context),
}

/// A middleware future
#[must_use = "futures do nothing unless polled"]
pub struct MiddlewareFuture {
    inner: Box<Future<Item = MiddlewareResult, Error = (Error, Context)> + Send>,
}

impl MiddlewareFuture {
    /// Creates a new middleware future
    pub fn new<F>(f: F) -> MiddlewareFuture
    where
        F: Future<Item = MiddlewareResult, Error = (Error, Context)> + Send + 'static,
    {
        MiddlewareFuture { inner: Box::new(f) }
    }
}

impl<E> From<Result<MiddlewareResult, (E, Context)>> for MiddlewareFuture
where
    E: Into<Error>,
{
    fn from(result: Result<MiddlewareResult, (E, Context)>) -> Self {
        MiddlewareFuture::new(future::result(result.map_err(|(err, context)| (err.into(), context))))
    }
}

impl From<MiddlewareResult> for MiddlewareFuture {
    fn from(result: MiddlewareResult) -> MiddlewareFuture {
        MiddlewareFuture::new(future::ok(result))
    }
}

impl Future for MiddlewareFuture {
    type Item = MiddlewareResult;
    type Error = (Error, Context);

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

/// Middleware handler
pub trait Middleware {
    /// Called before all handlers
    fn before(&mut self, context: Context, _update: &Update) -> MiddlewareFuture {
        MiddlewareResult::Continue(context).into()
    }

    /// Called after all handlers
    fn after(&mut self, context: Context, _update: &Update) -> MiddlewareFuture {
        MiddlewareResult::Continue(context).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        dispatcher::{Dispatcher, ErrorStrategy},
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
        continue_before: bool,
        continue_after: bool,
    }

    impl Middleware for MockMiddleware {
        fn before(&mut self, context: Context, _update: &Update) -> MiddlewareFuture {
            context.get::<Counter>().inc_calls();
            if self.continue_before {
                MiddlewareResult::Continue(context)
            } else {
                MiddlewareResult::Stop(context)
            }
            .into()
        }

        fn after(&mut self, context: Context, _update: &Update) -> MiddlewareFuture {
            context.get::<Counter>().inc_calls();
            if self.continue_after {
                MiddlewareResult::Continue(context)
            } else {
                MiddlewareResult::Stop(context)
            }
            .into()
        }
    }

    fn handle_message(context: Context, _message: &Message) -> HandlerFuture {
        context.get::<Counter>().inc_calls();
        context.into()
    }

    struct CounterMiddleware;

    impl Middleware for CounterMiddleware {
        fn before(&mut self, mut context: Context, _update: &Update) -> MiddlewareFuture {
            context.set(Counter::new());
            MiddlewareResult::Continue(context).into()
        }
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

        let dispatcher = Dispatcher::new(
            vec![
                Box::new(CounterMiddleware),
                Box::new(MockMiddleware {
                    continue_before: true,
                    continue_after: true,
                }),
                Box::new(MockMiddleware {
                    continue_before: false,
                    continue_after: true,
                }),
                Box::new(MockMiddleware {
                    continue_before: true,
                    continue_after: false,
                }),
                Box::new(MockMiddleware {
                    continue_before: true,
                    continue_after: false,
                }),
            ],
            vec![Handler::message(handle_message)],
            ErrorStrategy::Abort,
            ErrorStrategy::Abort,
        );
        let context = dispatcher.dispatch(update.clone()).wait().unwrap();
        assert_eq!(context.get::<Counter>().get_calls(), 5);

        let dispatcher = Dispatcher::new(
            vec![
                Box::new(CounterMiddleware),
                Box::new(MockMiddleware {
                    continue_before: true,
                    continue_after: false,
                }),
                Box::new(MockMiddleware {
                    continue_before: true,
                    continue_after: true,
                }),
            ],
            vec![Handler::message(handle_message)],
            ErrorStrategy::Abort,
            ErrorStrategy::Abort,
        );
        let context = dispatcher.dispatch(update).wait().unwrap();
        assert_eq!(context.get::<Counter>().get_calls(), 4);
    }
}
