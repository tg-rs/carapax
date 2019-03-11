use crate::{
    dispatcher::{Dispatcher, ErrorStrategy},
    handler::Handler,
    middleware::Middleware,
};
use tgbot::{handle_updates, UpdateMethod};

/// A Telegram Bot App
pub struct App<C> {
    context: C,
    middlewares: Vec<Box<Middleware<C> + Send + Sync>>,
    middleware_error_strategy: ErrorStrategy,
    handler_error_strategy: ErrorStrategy,
    handlers: Vec<Handler<C>>,
}

impl<C> App<C> {
    /// Creates a new app
    ///
    /// # Arguments
    ///
    /// * context - Any type you want to use as context
    pub fn new(context: C) -> Self {
        App {
            middlewares: vec![],
            middleware_error_strategy: ErrorStrategy::default(),
            handlers: vec![],
            handler_error_strategy: ErrorStrategy::default(),
            context,
        }
    }

    /// Set middleware error strategy
    ///
    /// See `ErrorStrategy` for more information
    pub fn middleware_error_strategy(mut self, strategy: ErrorStrategy) -> Self {
        self.middleware_error_strategy = strategy;
        self
    }

    /// Set handler error strategy
    ///
    /// See `ErrorStrategy` for more information
    pub fn handler_error_strategy(mut self, strategy: ErrorStrategy) -> Self {
        self.handler_error_strategy = strategy;
        self
    }

    /// Add middleware handler
    pub fn add_middleware<M>(mut self, middleware: M) -> Self
    where
        M: Middleware<C> + Send + Sync + 'static,
    {
        self.middlewares.push(Box::new(middleware));
        self
    }

    /// Add a regular handler
    pub fn add_handler(mut self, handler: Handler<C>) -> Self {
        self.handlers.push(handler);
        self
    }
}

impl<C> App<C>
where
    C: Send + Sync + 'static,
{
    /// Run app
    pub fn run(self, method: UpdateMethod) {
        let dispatcher = Dispatcher::new(self.middlewares, self.handlers, self.context)
            .middleware_error_strategy(self.middleware_error_strategy)
            .handler_error_strategy(self.handler_error_strategy);
        handle_updates(method, dispatcher);
    }
}
