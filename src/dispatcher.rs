use crate::{
    context::Context,
    handler::{Handler, HandlerFuture},
    middleware::{Middleware, MiddlewareFuture, MiddlewareResult},
};
use failure::Error;
use futures::{task, Async, Future, Poll};
use std::sync::{Arc, Mutex};
use tgbot::{types::Update, UpdateHandler};

/// Defines how to handle errors in middlewares and handlers
#[derive(Debug, Clone, Copy)]
pub enum ErrorStrategy {
    /// Ignore any error in a handler or middleware and write it to log
    Ignore,
    /// Return first error, all next handlers or middlewares will not run
    Abort,
}

pub(crate) struct Dispatcher {
    middlewares: Arc<Mutex<Vec<Box<Middleware + Send + Sync>>>>,
    handlers: Arc<Mutex<Vec<Handler>>>,
    pub(crate) context: Arc<Mutex<Context>>,
    middleware_error_strategy: ErrorStrategy,
    handler_error_strategy: ErrorStrategy,
}

impl Dispatcher {
    pub(crate) fn new(
        middlewares: Vec<Box<Middleware + Send + Sync>>,
        handlers: Vec<Handler>,
        context: Context,
        middleware_error_strategy: ErrorStrategy,
        handler_error_strategy: ErrorStrategy,
    ) -> Self {
        Self {
            middlewares: Arc::new(Mutex::new(middlewares)),
            handlers: Arc::new(Mutex::new(handlers)),
            context: Arc::new(Mutex::new(context)),
            middleware_error_strategy,
            handler_error_strategy,
        }
    }

    pub(crate) fn dispatch(&mut self, update: Update) -> DispatcherFuture {
        DispatcherFuture::new(
            self.middlewares.clone(),
            self.handlers.clone(),
            self.context.clone(),
            self.middleware_error_strategy,
            self.handler_error_strategy,
            update,
        )
    }
}

impl UpdateHandler for Dispatcher {
    fn handle(&mut self, update: Update) {
        tokio_executor::spawn(self.dispatch(update).then(|r| {
            if let Err(e) = r {
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
    context: Arc<Mutex<Context>>,
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

impl DispatcherFuture {
    fn new(
        middlewares: Arc<Mutex<Vec<Box<Middleware + Send + Sync>>>>,
        handlers: Arc<Mutex<Vec<Handler>>>,
        context: Arc<Mutex<Context>>,
        middleware_error_strategy: ErrorStrategy,
        handler_error_strategy: ErrorStrategy,
        update: Update,
    ) -> DispatcherFuture {
        DispatcherFuture {
            middlewares,
            handlers,
            context,
            middleware_error_strategy,
            handler_error_strategy,
            update,
            state: DispatcherFutureState::Before(0),
            middleware: None,
            handler: None,
        }
    }

    fn handle_before(&mut self, idx: usize) -> Poll<(), Error> {
        match self.middleware {
            Some(ref mut f) => match f.poll() {
                Ok(Async::Ready(MiddlewareResult::Continue)) => {
                    self.state = DispatcherFutureState::Before(idx + 1);
                    self.middleware = None;
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Ok(Async::Ready(MiddlewareResult::Stop)) => {
                    self.state = DispatcherFutureState::After(0);
                    self.middleware = None;
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Ok(Async::NotReady) => {
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Err(err) => match self.middleware_error_strategy {
                    ErrorStrategy::Abort => Err(err),
                    ErrorStrategy::Ignore => {
                        log::error!("An error has occurred in before middleware: {:?}", err);
                        self.state = DispatcherFutureState::Before(idx + 1);
                        self.middleware = None;
                        task::current().notify();
                        Ok(Async::NotReady)
                    }
                },
            },
            None => match self.middlewares.lock().unwrap().get_mut(idx) {
                Some(ref mut middleware) => {
                    let context = self.context.clone();
                    self.middleware = Some(middleware.before(&mut context.lock().unwrap(), &self.update));
                    task::current().notify();
                    Ok(Async::NotReady)
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

    fn handle_main(&mut self, idx: usize) -> Poll<(), Error> {
        match self.handler {
            Some(ref mut f) => match f.poll() {
                Ok(Async::Ready(())) => {
                    self.state = DispatcherFutureState::Main(idx + 1);
                    self.handler = None;
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Ok(Async::NotReady) => {
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Err(err) => match self.handler_error_strategy {
                    ErrorStrategy::Abort => Err(err),
                    ErrorStrategy::Ignore => {
                        log::error!("An error has occurred in handler: {:?}", err);
                        self.state = DispatcherFutureState::Main(idx + 1);
                        self.handler = None;
                        task::current().notify();
                        Ok(Async::NotReady)
                    }
                },
            },
            None => match self.handlers.lock().unwrap().get_mut(idx) {
                Some(handler) => {
                    let context = self.context.clone();
                    self.handler = Some(handler.handle(&mut context.lock().unwrap(), &self.update));
                    task::current().notify();
                    Ok(Async::NotReady)
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

    fn handle_after(&mut self, idx: usize) -> Poll<(), Error> {
        match self.middleware {
            Some(ref mut f) => match f.poll() {
                Ok(Async::Ready(MiddlewareResult::Continue)) => {
                    self.state = DispatcherFutureState::After(idx + 1);
                    self.middleware = None;
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Ok(Async::Ready(MiddlewareResult::Stop)) => Ok(Async::Ready(())),
                Ok(Async::NotReady) => {
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Err(err) => match self.middleware_error_strategy {
                    ErrorStrategy::Abort => Err(err),
                    ErrorStrategy::Ignore => {
                        log::error!("An error has occurred in after middleware: {:?}", err);
                        self.state = DispatcherFutureState::After(idx + 1);
                        self.middleware = None;
                        task::current().notify();
                        Ok(Async::NotReady)
                    }
                },
            },
            None => match self.middlewares.lock().unwrap().get_mut(idx) {
                Some(ref mut middleware) => {
                    let context = self.context.clone();
                    self.middleware = Some(middleware.after(&mut context.lock().unwrap(), &self.update));
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                None => Ok(Async::Ready(())),
            },
        }
    }

    fn handle(&mut self) -> Poll<(), Error> {
        use self::DispatcherFutureState::*;
        match self.state {
            Before(idx) => self.handle_before(idx),
            Main(idx) => self.handle_main(idx),
            After(idx) => self.handle_after(idx),
        }
    }
}

impl Future for DispatcherFuture {
    type Item = ();
    type Error = Error;

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
        fn before(&mut self, context: &mut Context, _update: &Update) -> MiddlewareFuture {
            context.get::<Counter>().inc_calls();
            Err(ErrorMock).into()
        }

        fn after(&mut self, context: &mut Context, _update: &Update) -> MiddlewareFuture {
            context.get::<Counter>().inc_calls();
            Err(ErrorMock).into()
        }
    }

    fn handle_update_error(context: &mut Context, _update: &Update) -> HandlerFuture {
        context.get::<Counter>().inc_calls();
        Err(ErrorMock).into()
    }

    fn create_context() -> Context {
        let mut context = Context::default();
        context.set(Counter::new());
        context
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
        let mut dispatcher = Dispatcher::new(
            vec![Box::new(ErrorMiddleware), Box::new(ErrorMiddleware)],
            vec![Handler::update(handle_update_error)],
            create_context(),
            ErrorStrategy::Abort,
            ErrorStrategy::Abort,
        );
        dispatcher.dispatch(update.clone()).wait().unwrap_err();
        assert_eq!(dispatcher.context.lock().unwrap().get::<Counter>().get_calls(), 1);

        // Aborted on handler call
        let mut dispatcher = Dispatcher::new(
            vec![Box::new(ErrorMiddleware), Box::new(ErrorMiddleware)],
            vec![Handler::update(handle_update_error)],
            create_context(),
            ErrorStrategy::Ignore,
            ErrorStrategy::Abort,
        );
        dispatcher.dispatch(update.clone()).wait().unwrap_err();
        assert_eq!(dispatcher.context.lock().unwrap().get::<Counter>().get_calls(), 3);

        // Ignore all errors
        let mut dispatcher = Dispatcher::new(
            vec![Box::new(ErrorMiddleware), Box::new(ErrorMiddleware)],
            vec![Handler::update(handle_update_error)],
            create_context(),
            ErrorStrategy::Ignore,
            ErrorStrategy::Ignore,
        );
        dispatcher.dispatch(update.clone()).wait().unwrap();
        assert_eq!(dispatcher.context.lock().unwrap().get::<Counter>().get_calls(), 5);
    }
}
