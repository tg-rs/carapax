use crate::{
    context::Context,
    handler::{Handler, HandlerFuture},
    middleware::{Middleware, MiddlewareFuture, MiddlewareResult},
};
use failure::Error;
use futures::{task, Async, Future, Poll};
use std::sync::{Arc, Mutex};
use tgbot::{types::Update, Api, UpdateHandler};

/// Defines how to handle errors in middlewares and handlers
#[derive(Debug, Clone, Copy)]
pub enum ErrorStrategy {
    /// Ignore any error in a handler or middleware and write it to log
    Ignore,
    /// Return first error, all next handlers or middlewares will not run
    Abort,
}

pub(crate) struct Dispatcher {
    api: Api,
    middlewares: Arc<Mutex<Vec<Box<Middleware + Send + Sync>>>>,
    handlers: Arc<Mutex<Vec<Handler>>>,
    middleware_error_strategy: ErrorStrategy,
    handler_error_strategy: ErrorStrategy,
}

impl Dispatcher {
    pub(crate) fn new(
        api: Api,
        middlewares: Vec<Box<Middleware + Send + Sync>>,
        handlers: Vec<Handler>,
        middleware_error_strategy: ErrorStrategy,
        handler_error_strategy: ErrorStrategy,
    ) -> Self {
        Self {
            api,
            middlewares: Arc::new(Mutex::new(middlewares)),
            handlers: Arc::new(Mutex::new(handlers)),
            middleware_error_strategy,
            handler_error_strategy,
        }
    }

    pub(crate) fn dispatch(&self, update: Update) -> DispatcherFuture {
        let mut context = Context::default();
        context.set(self.api.clone());
        DispatcherFuture::new(
            self.middlewares.clone(),
            self.handlers.clone(),
            context,
            self.middleware_error_strategy,
            self.handler_error_strategy,
            update,
        )
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
    middlewares: Arc<Mutex<Vec<Box<Middleware + Send + Sync>>>>,
    handlers: Arc<Mutex<Vec<Handler>>>,
    context: Option<Context>,
    middleware_error_strategy: ErrorStrategy,
    handler_error_strategy: ErrorStrategy,
    update: Update,
    state: DispatcherFutureState,
    middleware: Option<MiddlewareFuture>,
    handler: Option<HandlerFuture>,
}

enum DispatcherFutureState {
    Before(usize),
    Main(usize),
    After(usize),
}

macro_rules! context_lost {
    () => {
        panic!("Surprise! Context was lost...");
    };
}

impl DispatcherFuture {
    fn new(
        middlewares: Arc<Mutex<Vec<Box<Middleware + Send + Sync>>>>,
        handlers: Arc<Mutex<Vec<Handler>>>,
        context: Context,
        middleware_error_strategy: ErrorStrategy,
        handler_error_strategy: ErrorStrategy,
        update: Update,
    ) -> DispatcherFuture {
        DispatcherFuture {
            middlewares,
            handlers,
            context: Some(context),
            middleware_error_strategy,
            handler_error_strategy,
            update,
            state: DispatcherFutureState::Before(0),
            middleware: None,
            handler: None,
        }
    }

    fn handle_before(&mut self, idx: usize) -> Poll<Context, (Error, Context)> {
        match self.middleware {
            Some(ref mut f) => match f.poll() {
                Ok(Async::Ready(MiddlewareResult::Continue(context))) => {
                    self.context = Some(context);
                    self.state = DispatcherFutureState::Before(idx + 1);
                    self.middleware = None;
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Ok(Async::Ready(MiddlewareResult::Stop(context))) => {
                    self.context = Some(context);
                    self.state = DispatcherFutureState::After(0);
                    self.middleware = None;
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Ok(Async::NotReady) => {
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Err((err, context)) => match self.middleware_error_strategy {
                    ErrorStrategy::Abort => Err((err, context)),
                    ErrorStrategy::Ignore => {
                        log::error!("An error has occurred in before middleware: {:?}", err);
                        self.context = Some(context);
                        self.state = DispatcherFutureState::Before(idx + 1);
                        self.middleware = None;
                        task::current().notify();
                        Ok(Async::NotReady)
                    }
                },
            },
            None => match self.middlewares.lock().unwrap().get_mut(idx) {
                Some(ref mut middleware) => {
                    if let Some(context) = self.context.take() {
                        self.middleware = Some(middleware.before(context, &self.update));
                        task::current().notify();
                        Ok(Async::NotReady)
                    } else {
                        context_lost!()
                    }
                }
                None => {
                    self.state = DispatcherFutureState::Main(0);
                    self.handler = None;
                    task::current().notify();
                    Ok(Async::NotReady)
                }
            },
        }
    }

    fn handle_main(&mut self, idx: usize) -> Poll<Context, (Error, Context)> {
        match self.handler {
            Some(ref mut f) => match f.poll() {
                Ok(Async::Ready(context)) => {
                    self.context = Some(context);
                    self.state = DispatcherFutureState::Main(idx + 1);
                    self.handler = None;
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Ok(Async::NotReady) => {
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Err((err, context)) => match self.handler_error_strategy {
                    ErrorStrategy::Abort => Err((err, context)),
                    ErrorStrategy::Ignore => {
                        log::error!("An error has occurred in handler: {:?}", err);
                        self.context = Some(context);
                        self.state = DispatcherFutureState::Main(idx + 1);
                        self.handler = None;
                        task::current().notify();
                        Ok(Async::NotReady)
                    }
                },
            },
            None => match self.handlers.lock().unwrap().get_mut(idx) {
                Some(handler) => {
                    if let Some(context) = self.context.take() {
                        self.handler = Some(handler.handle(context, &self.update));
                        task::current().notify();
                        Ok(Async::NotReady)
                    } else {
                        context_lost!()
                    }
                }
                None => {
                    self.state = DispatcherFutureState::After(0);
                    self.middleware = None;
                    task::current().notify();
                    Ok(Async::NotReady)
                }
            },
        }
    }

    fn handle_after(&mut self, idx: usize) -> Poll<Context, (Error, Context)> {
        match self.middleware {
            Some(ref mut f) => match f.poll() {
                Ok(Async::Ready(MiddlewareResult::Continue(context))) => {
                    self.context = Some(context);
                    self.state = DispatcherFutureState::After(idx + 1);
                    self.middleware = None;
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Ok(Async::Ready(MiddlewareResult::Stop(context))) => Ok(Async::Ready(context)),
                Ok(Async::NotReady) => {
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Err((err, context)) => match self.middleware_error_strategy {
                    ErrorStrategy::Abort => Err((err, context)),
                    ErrorStrategy::Ignore => {
                        log::error!("An error has occurred in after middleware: {:?}", err);
                        self.context = Some(context);
                        self.state = DispatcherFutureState::After(idx + 1);
                        self.middleware = None;
                        task::current().notify();
                        Ok(Async::NotReady)
                    }
                },
            },
            None => {
                if let Some(context) = self.context.take() {
                    match self.middlewares.lock().unwrap().get_mut(idx) {
                        Some(ref mut middleware) => {
                            self.middleware = Some(middleware.after(context, &self.update));
                            task::current().notify();
                            Ok(Async::NotReady)
                        }
                        None => Ok(Async::Ready(context)),
                    }
                } else {
                    context_lost!()
                }
            }
        }
    }

    fn handle(&mut self) -> Poll<Context, (Error, Context)> {
        use self::DispatcherFutureState::*;
        match self.state {
            Before(idx) => self.handle_before(idx),
            Main(idx) => self.handle_main(idx),
            After(idx) => self.handle_after(idx),
        }
    }
}

impl Future for DispatcherFuture {
    type Item = Context;
    type Error = (Error, Context);

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.handle()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn parse_update(data: &str) -> Update {
        serde_json::from_str(data).unwrap()
    }

    #[derive(Debug, Fail)]
    #[fail(display = "Test error")]
    struct ErrorMock;

    struct ErrorMiddleware;

    impl Middleware for ErrorMiddleware {
        fn before(&mut self, context: Context, _update: &Update) -> MiddlewareFuture {
            context.get::<Counter>().inc_calls();
            Err((ErrorMock, context)).into()
        }

        fn after(&mut self, context: Context, _update: &Update) -> MiddlewareFuture {
            context.get::<Counter>().inc_calls();
            Err((ErrorMock, context)).into()
        }
    }

    fn handle_update_error(context: Context, _update: &Update) -> HandlerFuture {
        context.get::<Counter>().inc_calls();
        Err((ErrorMock, context)).into()
    }

    struct CounterMiddleware;

    impl Middleware for CounterMiddleware {
        fn before(&mut self, mut context: Context, _update: &Update) -> MiddlewareFuture {
            context.set(Counter::new());
            MiddlewareResult::Continue(context).into()
        }
    }

    #[test]
    fn test_error_strategy() {
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

        // Aborted on first call
        let dispatcher = Dispatcher::new(
            Api::new("token", None::<&str>).unwrap(),
            vec![
                Box::new(CounterMiddleware),
                Box::new(ErrorMiddleware),
                Box::new(ErrorMiddleware),
            ],
            vec![Handler::update(handle_update_error)],
            ErrorStrategy::Abort,
            ErrorStrategy::Abort,
        );
        let (_err, context) = dispatcher.dispatch(update.clone()).wait().unwrap_err();
        assert_eq!(context.get::<Counter>().get_calls(), 1);

        // Aborted on handler call
        let dispatcher = Dispatcher::new(
            Api::new("token", None::<&str>).unwrap(),
            vec![
                Box::new(CounterMiddleware),
                Box::new(ErrorMiddleware),
                Box::new(ErrorMiddleware),
            ],
            vec![Handler::update(handle_update_error)],
            ErrorStrategy::Ignore,
            ErrorStrategy::Abort,
        );
        let (_err, context) = dispatcher.dispatch(update.clone()).wait().unwrap_err();
        assert_eq!(context.get::<Counter>().get_calls(), 3);

        // Ignore all errors
        let dispatcher = Dispatcher::new(
            Api::new("token", None::<&str>).unwrap(),
            vec![
                Box::new(CounterMiddleware),
                Box::new(ErrorMiddleware),
                Box::new(ErrorMiddleware),
            ],
            vec![Handler::update(handle_update_error)],
            ErrorStrategy::Ignore,
            ErrorStrategy::Ignore,
        );
        let context = dispatcher.dispatch(update.clone()).wait().unwrap();
        assert_eq!(context.get::<Counter>().get_calls(), 5);
    }
}
