use crate::api::Api;
use crate::types::{
    BotCommand, CallbackQuery, ChosenInlineResult, InlineQuery, Message, PreCheckoutQuery,
    ShippingQuery,
};
use failure::Error;
use futures::{future, Async, Future, Poll};
use std::error::Error as StdError;

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
        HandlerFuture::new(future::ok(result))
    }
}

impl<E> From<Result<HandlerResult, E>> for HandlerFuture
where
    E: StdError + Sync + Send + 'static,
{
    fn from(result: Result<HandlerResult, E>) -> Self {
        HandlerFuture::new(future::result(result.map_err(Error::from)))
    }
}

impl Future for HandlerFuture {
    type Item = HandlerResult;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

#[must_use = "futures do nothing unless polled"]
pub(super) struct IterHandlerFuture {
    items: Vec<HandlerFuture>,
    current: usize,
}

impl IterHandlerFuture {
    pub(super) fn new(items: Vec<HandlerFuture>) -> IterHandlerFuture {
        IterHandlerFuture { items, current: 0 }
    }
}

impl Future for IterHandlerFuture {
    type Item = usize;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let items_len = self.items.len();
        if items_len == 0 {
            return Ok(Async::Ready(0));
        }
        if self.current >= items_len {
            return Ok(Async::Ready(self.current));
        }
        let f = &mut self.items[self.current];
        match f.poll() {
            Ok(Async::Ready(HandlerResult::Continue)) => {
                self.current += 1;
                Ok(Async::NotReady)
            }
            Ok(Async::Ready(HandlerResult::Stop)) => Ok(Async::Ready(self.current + 1)),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(err) => Err(err),
        }
    }
}

macro_rules! impl_func {
    ($handler:ident($arg:ident)) => {
        impl<F, R> $handler for F
        where
            F: FnMut(&Api, &$arg) -> R,
            R: Into<HandlerFuture>,
        {
            fn handle(&mut self, api: &Api, arg: &$arg) -> HandlerFuture {
                (self)(api, arg).into()
            }
        }
    };
}

/// A regular message handler
pub trait MessageHandler {
    /// Handles a message
    fn handle(&mut self, api: &Api, message: &Message) -> HandlerFuture;
}

impl_func!(MessageHandler(Message));

/// A command handler
pub struct CommandHandler {
    name: String,
    handler: Box<MessageHandler + Send + Sync>,
}

impl CommandHandler {
    /// Creates a new command handler
    ///
    /// # Arguments
    ///
    /// - name - command name (starts with /)
    /// - handler - a message handler
    pub fn new<S, H>(name: S, handler: H) -> Self
    where
        S: Into<String>,
        H: MessageHandler + 'static + Send + Sync,
    {
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
    fn handle(&mut self, api: &Api, message: &Message) -> HandlerFuture {
        self.handler.handle(api, message)
    }
}

/// An inline query handler
pub trait InlineQueryHandler {
    /// Handles a query
    fn handle(&mut self, api: &Api, query: &InlineQuery) -> HandlerFuture;
}

impl_func!(InlineQueryHandler(InlineQuery));

/// A chosen inline result handler
pub trait ChosenInlineResultHandler {
    /// Handles a result
    fn handle(&mut self, api: &Api, result: &ChosenInlineResult) -> HandlerFuture;
}

impl_func!(ChosenInlineResultHandler(ChosenInlineResult));

/// A callback query handler
pub trait CallbackQueryHandler {
    /// Handles a query
    fn handle(&mut self, api: &Api, query: &CallbackQuery) -> HandlerFuture;
}

impl_func!(CallbackQueryHandler(CallbackQuery));

/// A shipping query handler
pub trait ShippingQueryHandler {
    /// Handles a query
    fn handle(&mut self, api: &Api, query: &ShippingQuery) -> HandlerFuture;
}

impl_func!(ShippingQueryHandler(ShippingQuery));

/// A pre checkout query handler
pub trait PreCheckoutQueryHandler {
    /// Handles a query
    fn handle(&mut self, api: &Api, query: &PreCheckoutQuery) -> HandlerFuture;
}

impl_func!(PreCheckoutQueryHandler(PreCheckoutQuery));
