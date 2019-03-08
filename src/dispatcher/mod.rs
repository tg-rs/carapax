use crate::api::Api;
use crate::types::{BotCommand, Update, UpdateKind};
use failure::Error;
use futures::{future, task, Async, Future, Poll, Stream};
use std::sync::{Arc, Mutex};

mod handler;
mod middleware;

#[cfg(test)]
mod tests;

pub use self::handler::*;
pub use self::middleware::*;

struct Store<C> {
    middlewares: Vec<Box<Middleware<C> + Send + Sync>>,
    handlers: Vec<Handler<C>>,
}

/// Defines how to handle errors in middlewares and handlers
#[derive(Debug, Clone, Copy)]
pub enum ErrorStrategy {
    /// Ignore any error in a handler or middleware and write it to log
    Ignore,
    /// Return first error, all next handlers or middlewares will not run
    Abort,
}

/// Dispatcher builder
pub struct DispatcherBuilder<C> {
    store: Store<C>,
    middleware_error_strategy: ErrorStrategy,
    handler_error_strategy: ErrorStrategy,
}

impl<C> DispatcherBuilder<C> {
    /// Creates a new builder
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            store: Store {
                middlewares: vec![],
                handlers: vec![],
            },
            middleware_error_strategy: ErrorStrategy::Abort,
            handler_error_strategy: ErrorStrategy::Abort,
        }
    }

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
        H: Middleware<C> + 'static + Send + Sync,
    {
        self.store.middlewares.push(Box::new(handler));
        self
    }

    /// Add a regular handler
    pub fn add_handler(mut self, handler: Handler<C>) -> Self {
        self.store.handlers.push(handler);
        self
    }

    /// Create a dispatcher
    pub fn build(self, context: C) -> Dispatcher<C> {
        Dispatcher {
            context: Arc::new(context),
            store: Arc::new(Mutex::new(self.store)),
            middleware_error_strategy: self.middleware_error_strategy,
            handler_error_strategy: self.handler_error_strategy,
        }
    }
}

/// Dispatcher
pub struct Dispatcher<C> {
    context: Arc<C>,
    store: Arc<Mutex<Store<C>>>,
    middleware_error_strategy: ErrorStrategy,
    handler_error_strategy: ErrorStrategy,
}

impl<C> Dispatcher<C>
where
    C: Send + Sync + 'static,
{
    /// Dispatch an update
    pub fn dispatch(&mut self, update: Update) -> DispatcherFuture<C> {
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

/// Dispatcher future
#[must_use = "futures do nothing unless polled"]
pub struct DispatcherFuture<C> {
    store: Arc<Mutex<Store<C>>>,
    context: Arc<C>,
    middleware_error_strategy: ErrorStrategy,
    handler_error_strategy: ErrorStrategy,
    update: Update,
    state: DispatcherFutureState,
    middleware: Option<MiddlewareFuture>,
    handler: Option<HandlerFuture>,
    commands: Option<Vec<BotCommand>>,
}

enum DispatcherFutureState {
    Before(usize),
    Main(usize),
    After(usize),
}

impl<C> DispatcherFuture<C>
where
    C: Send + Sync + 'static,
{
    fn new(
        store: Arc<Mutex<Store<C>>>,
        context: Arc<C>,
        middleware_error_strategy: ErrorStrategy,
        handler_error_strategy: ErrorStrategy,
        update: Update,
    ) -> DispatcherFuture<C> {
        DispatcherFuture {
            store,
            context,
            middleware_error_strategy,
            handler_error_strategy,
            update,
            state: DispatcherFutureState::Before(0),
            middleware: None,
            handler: None,
            commands: None,
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
                    self.commands = match self.update.kind {
                        UpdateKind::Message(ref msg)
                        | UpdateKind::EditedMessage(ref msg)
                        | UpdateKind::ChannelPost(ref msg)
                        | UpdateKind::EditedChannelPost(ref msg) => msg.get_commands(),
                        _ => None,
                    };
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
                    self.handler =
                        Some(handler.handle(&self.context, &self.update, &self.commands));
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

impl<C> Future for DispatcherFuture<C>
where
    C: Send + Sync + 'static,
{
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.handle()
    }
}
