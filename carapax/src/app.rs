use crate::{
    dispatcher::{Dispatcher, ErrorStrategy},
    handler::BoxedHandler,
    FromUpdate, Handler, HandlerFuture, HandlerWrapper,
};
use futures::Future;
use tgbot::{handle_updates, Api, UpdateMethod};

/// A Telegram Bot App
pub struct App {
    handlers: Vec<BoxedHandler>,
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
    pub fn add_handler<H, I, R>(mut self, handler: H) -> Self
    where
        H: Handler<Item = I, Result = R> + Send + Sync + 'static,
        I: FromUpdate,
        R: Into<HandlerFuture>,
    {
        self.handlers.push(HandlerWrapper::boxed(handler));
        self
    }

    /// Returns a future that will run app
    pub fn run(self, api: Api, method: UpdateMethod) -> impl Future<Item = (), Error = ()> {
        handle_updates(method, Dispatcher::new(api, self.handlers, self.error_strategy))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{context::Context, core::types::Update, FnHandler};

    fn update_handler(_context: &mut Context, _update: Update) {}

    #[test]
    fn handlers() {
        let mut app = App::new();
        assert_eq!(app.handlers.len(), 0);
        app = app.add_handler(FnHandler::from(update_handler));
        assert_eq!(app.handlers.len(), 1);
    }

    #[test]
    fn error_strategy() {
        let mut app = App::default();
        assert_eq!(app.error_strategy, ErrorStrategy::Abort);
        app = app.error_strategy(ErrorStrategy::Ignore);
        assert_eq!(app.error_strategy, ErrorStrategy::Ignore);
    }
}
