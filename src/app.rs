use crate::{
    dispatcher::{Dispatcher, ErrorStrategy},
    handler::Handler,
};
use futures::Future;
use tgbot::{handle_updates, Api, UpdateMethod};

/// A Telegram Bot App
pub struct App {
    handlers: Vec<Handler>,
    error_strategy: ErrorStrategy,
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
            handlers: vec![],
            error_strategy: ErrorStrategy::Abort,
        }
    }

    /// Set handler error strategy
    ///
    /// See [ErrorStrategy](enum.ErrorStrategy.html) for more information.
    /// Default values is `ErrorStrategy::Abort`.
    pub fn error_strategy(mut self, strategy: ErrorStrategy) -> Self {
        self.error_strategy = strategy;
        self
    }

    /// Add a handler
    ///
    /// When a handler fails with error, all next handlers will not run.
    /// Use `App::error_strategy()` to change this behaviour.
    pub fn add_handler(mut self, handler: Handler) -> Self {
        self.handlers.push(handler);
        self
    }

    /// Returns a future that will run app
    pub fn run(self, api: Api, method: UpdateMethod) -> impl Future<Item = (), Error = ()> {
        handle_updates(method, Dispatcher::new(api, self.handlers, self.error_strategy))
    }
}
