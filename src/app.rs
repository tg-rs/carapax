use crate::{
    dispatcher::{Dispatcher, ErrorStrategy},
    handler::Handler,
    middleware::Middleware,
};
use futures::Future;
use tgbot::{handle_updates, Api, UpdateMethod};

/// A Telegram Bot App
pub struct App {
    middlewares: Vec<Box<Middleware + Send + Sync>>,
    middleware_error_strategy: ErrorStrategy,
    handlers: Vec<Handler>,
    handler_error_strategy: ErrorStrategy,
}

impl Default for App {
    fn default() -> App {
        App::new()
    }
}

impl App {
    /// Creates a new app
    pub fn new() -> Self {
        App {
            middlewares: vec![],
            middleware_error_strategy: ErrorStrategy::Abort,
            handlers: vec![],
            handler_error_strategy: ErrorStrategy::Abort,
        }
    }

    /// Set middleware error strategy
    ///
    /// See [ErrorStrategy](enum.ErrorStrategy.html) for more information.
    /// Default values is `ErrorStrategy::Abort`.
    pub fn middleware_error_strategy(mut self, strategy: ErrorStrategy) -> Self {
        self.middleware_error_strategy = strategy;
        self
    }

    /// Set handler error strategy
    ///
    /// See [ErrorStrategy](enum.ErrorStrategy.html) for more information.
    /// Default values is `ErrorStrategy::Abort`.
    pub fn handler_error_strategy(mut self, strategy: ErrorStrategy) -> Self {
        self.handler_error_strategy = strategy;
        self
    }

    /// Add middleware handler
    pub fn add_middleware<M>(mut self, middleware: M) -> Self
    where
        M: Middleware + Send + Sync + 'static,
    {
        self.middlewares.push(Box::new(middleware));
        self
    }

    /// Add a regular handler
    pub fn add_handler(mut self, handler: Handler) -> Self {
        self.handlers.push(handler);
        self
    }

    /// Run app
    pub fn run(self, api: Api, method: UpdateMethod) -> impl Future<Item = (), Error = ()> {
        handle_updates(
            method,
            Dispatcher::new(
                api,
                self.middlewares,
                self.handlers,
                self.middleware_error_strategy,
                self.handler_error_strategy,
            ),
        )
    }
}
