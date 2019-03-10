use crate::context::Context;
use failure::Error;
use futures::{future, task, Async, Future, Poll, Stream};
use std::sync::{Arc, Mutex};
use tgbot::{types::Update, webhook::UpdateHandler as WebhookUpdateHandler, Api};

mod handler;
mod middleware;

#[cfg(test)]
mod tests;

pub use self::{handler::*, middleware::*};

#[derive(Default)]
struct Store {
    middlewares: Vec<Box<Middleware + Send + Sync>>,
    handlers: Vec<Handler>,
}

/// Defines how to handle errors in middlewares and handlers
#[derive(Debug, Clone, Copy)]
pub enum ErrorStrategy {
    /// Ignore any error in a handler or middleware and write it to log
    Ignore,
    /// Return first error, all next handlers or middlewares will not run
    Abort,
}

impl Default for ErrorStrategy {
    fn default() -> ErrorStrategy {
        ErrorStrategy::Abort
    }
}

/// Dispatcher builder
#[derive(Default)]
pub struct DispatcherBuilder {
    store: Store,
    middleware_error_strategy: ErrorStrategy,
    handler_error_strategy: ErrorStrategy,
}

impl DispatcherBuilder {
    /// Set middleware error strategy
    pub fn middleware_error_strategy(mut self, strategy: ErrorStrategy) -> Self {
        self.middleware_error_strategy = strategy;
        self
    }

    /// Set handler error strategy
    pub fn handler_error_strategy(mut self, strategy: ErrorStrategy) -> Self {
        self.handler_error_strategy = strategy;
        self
    }

    /// Add middleware handler
    pub fn add_middleware<H>(mut self, handler: H) -> Self
    where
        H: Middleware + 'static + Send + Sync,
    {
        self.store.middlewares.push(Box::new(handler));
        self
    }

    /// Add a regular handler
    pub fn add_handler(mut self, handler: Handler) -> Self {
        self.store.handlers.push(handler);
        self
    }

    /// Create a dispatcher
    pub fn build(self, context: Context) -> Dispatcher {
        Dispatcher {
            context: Arc::new(context),
            store: Arc::new(Mutex::new(self.store)),
            middleware_error_strategy: self.middleware_error_strategy,
            handler_error_strategy: self.handler_error_strategy,
        }
    }
}

/// Dispatcher
pub struct Dispatcher {
    context: Arc<Context>,
    store: Arc<Mutex<Store>>,
    middleware_error_strategy: ErrorStrategy,
    handler_error_strategy: ErrorStrategy,
}

impl Dispatcher {
    /// Dispatch an update
    pub fn dispatch(&mut self, update: Update) -> DispatcherFuture {
        DispatcherFuture::new(
            self.store.clone(),
            self.context.clone(),
            self.middleware_error_strategy,
            self.handler_error_strategy,
            update,
        )
    }

    /// Starts a polling stream
    pub fn start_polling(mut self, api: Api) {
        tokio::run(future::lazy(move || {
            api.get_updates()
                .for_each(move |update| {
                    let f = self.dispatch(update);
                    api.spawn(f);
                    Ok(())
                })
                .then(|r| {
                    if let Err(e) = r {
                        log::error!("Polling error: {:?}", e)
                    }
                    Ok(())
                })
        }));
    }
}

impl WebhookUpdateHandler for Dispatcher {
    fn handle(&mut self, update: Update) {
        tokio::spawn(self.dispatch(update).then(|r| {
            if let Err(e) = r {
                log::error!("Failed to dispatch update: {:?}", e);
            }
            Ok(())
        }));
    }
}

/// Dispatcher future
#[must_use = "futures do nothing unless polled"]
pub struct DispatcherFuture {
    store: Arc<Mutex<Store>>,
    context: Arc<Context>,
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
        store: Arc<Mutex<Store>>,
        context: Arc<Context>,
        middleware_error_strategy: ErrorStrategy,
        handler_error_strategy: ErrorStrategy,
        update: Update,
    ) -> DispatcherFuture {
        DispatcherFuture {
            store,
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
            None => match self.store.lock().unwrap().middlewares.get_mut(idx) {
                Some(ref mut middleware) => {
                    self.middleware = Some(middleware.before(&self.context, &self.update));
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
            None => match self.store.lock().unwrap().handlers.get_mut(idx) {
                Some(ref mut handler) => {
                    self.handler = Some(handler.handle(&self.context, &self.update));
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
            None => match self.store.lock().unwrap().middlewares.get_mut(idx) {
                Some(ref mut middleware) => {
                    self.middleware = Some(middleware.after(&self.context, &self.update));
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
