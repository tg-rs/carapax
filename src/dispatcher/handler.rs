use crate::api::Api;
use crate::types::{
    BotCommand, CallbackQuery, ChosenInlineResult, InlineQuery, Message, PreCheckoutQuery,
    ShippingQuery,
};
use failure::Error;
use futures::{Future, Poll};

/// Result of a handler
#[derive(Copy, Clone, Debug)]
pub enum HandlerResult {
    /// Continue propagation
    ///
    /// Next handler will run after current has finished
    Continue,
    /// Stop propagation
    ///
    /// Next handler will not run after current has finished
    Stop,
}

/// A handler future
#[must_use = "futures do nothing unless polled"]
pub struct HandlerFuture {
    inner: Box<Future<Item = HandlerResult, Error = Error> + Send>,
}

impl HandlerFuture {
    /// Creates a new handler future
    pub fn new<F>(f: F) -> HandlerFuture
    where
        F: Future<Item = HandlerResult, Error = Error> + 'static + Send,
    {
        HandlerFuture { inner: Box::new(f) }
    }
}

impl From<HandlerResult> for HandlerFuture {
    fn from(result: HandlerResult) -> HandlerFuture {
        HandlerFuture::new(futures::future::ok(result))
    }
}

impl Future for HandlerFuture {
    type Item = HandlerResult;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

/// A regular message handler
pub trait MessageHandler {
    /// Handles a message
    fn handle(&self, api: &Api, message: &Message) -> HandlerFuture;
}

/// A command handler
pub struct CommandHandler {
    name: String,
    handler: Box<MessageHandler + Send>,
}

impl CommandHandler {
    /// Creates a new command handler
    ///
    /// # Arguments
    ///
    /// - name - command name (starts with /)
    /// - handler - a message handler
    pub fn new<S: Into<String>, H: MessageHandler + 'static + Send>(name: S, handler: H) -> Self {
        CommandHandler {
            name: name.into(),
            handler: Box::new(handler),
        }
    }

    pub(crate) fn accepts(&self, command: &BotCommand) -> bool {
        self.name == command.command
    }
}

impl MessageHandler for CommandHandler {
    fn handle(&self, api: &Api, message: &Message) -> HandlerFuture {
        self.handler.handle(api, message)
    }
}

/// An inline query handler
pub trait InlineQueryHandler {
    /// Handles a query
    fn handle(&self, api: &Api, query: &InlineQuery) -> HandlerFuture;
}

/// A chosen inline result handler
pub trait ChosenInlineResultHandler {
    /// Handles a result
    fn handle(&self, api: &Api, result: &ChosenInlineResult) -> HandlerFuture;
}

/// A callback query handler
pub trait CallbackQueryHandler {
    /// Handles a query
    fn handle(&self, api: &Api, query: &CallbackQuery) -> HandlerFuture;
}

/// A shipping query handler
pub trait ShippingQueryHandler {
    /// Handles a query
    fn handle(&self, api: &Api, query: &ShippingQuery) -> HandlerFuture;
}

/// A pre checkout query handler
pub trait PreCheckoutQueryHandler {
    /// Handles a query
    fn handle(&self, api: &Api, query: &PreCheckoutQuery) -> HandlerFuture;
}
