use crate::{
    dispatcher::{Dispatcher, ErrorStrategy},
    handler::Handler,
    middleware::Middleware,
};
use std::net::SocketAddr;
use tgbot::{handle_updates, Api, UpdateMethod, UpdatesStream, UpdatesStreamOptions};

/// Defines how to get updates
pub struct RunMethod {
    kind: RunMethodKind,
}

impl RunMethod {
    /// Get updates using long polling
    pub fn poll(options: UpdatesStreamOptions) -> Self {
        Self {
            kind: RunMethodKind::Poll(options),
        }
    }

    /// Get updates via webhook
    ///
    /// # Arguments
    ///
    /// - addr - Bind address
    /// - path - URL path for webhook
    pub fn webhook<A, S>(addr: A, path: S) -> Self
    where
        A: Into<SocketAddr>,
        S: Into<String>,
    {
        Self {
            kind: RunMethodKind::Webhook(addr.into(), path.into()),
        }
    }
}

enum RunMethodKind {
    Poll(UpdatesStreamOptions),
    Webhook(SocketAddr, String),
}

/// A Telegram Bot App
pub struct App<C> {
    api: Api,
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
    /// * api - tgbot::Api
    /// * context - any type you want to use as context
    pub fn new(api: Api, context: C) -> Self {
        App {
            api,
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
    pub fn run(self, method: RunMethod) {
        let dispatcher = Dispatcher::new(self.middlewares, self.handlers, self.context)
            .middleware_error_strategy(self.middleware_error_strategy)
            .handler_error_strategy(self.handler_error_strategy);
        let update_method = match method.kind {
            RunMethodKind::Poll(options) => UpdateMethod::poll(UpdatesStream::new(self.api.clone()).options(options)),
            RunMethodKind::Webhook(addr, path) => UpdateMethod::webhook(addr, path),
        };
        handle_updates(update_method, dispatcher);
    }
}
