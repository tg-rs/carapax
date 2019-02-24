use crate::api::Api;
use crate::types::{Update, UpdateKind};
use failure::Error;
use futures::{Async, Future, Poll};

mod handler;
#[cfg(test)]
mod tests;

pub use self::handler::*;

/// Dispatcher
pub struct Dispatcher {
    api: Api,
    message: Vec<Box<MessageHandler + Send>>,
    command: Vec<CommandHandler>,
    inline_query: Vec<Box<InlineQueryHandler + Send>>,
    chosen_inline_result: Vec<Box<ChosenInlineResultHandler + Send>>,
    callback_query: Vec<Box<CallbackQueryHandler + Send>>,
    shipping_query: Vec<Box<ShippingQueryHandler + Send>>,
    pre_checkout_query: Vec<Box<PreCheckoutQueryHandler + Send>>,
}

impl Dispatcher {
    /// Creates a new dispatcher
    pub fn new(api: Api) -> Self {
        Dispatcher {
            api,
            message: vec![],
            command: vec![],
            inline_query: vec![],
            chosen_inline_result: vec![],
            callback_query: vec![],
            shipping_query: vec![],
            pre_checkout_query: vec![],
        }
    }

    /// Add message handler
    pub fn add_message_handler<H: MessageHandler + 'static + Send>(
        &mut self,
        handler: H,
    ) -> &mut Self {
        self.message.push(Box::new(handler));
        self
    }

    /// Add command handler
    pub fn add_command_handler(&mut self, handler: CommandHandler) -> &mut Self {
        self.command.push(handler);
        self
    }

    /// Add inline query handler
    pub fn add_inline_query_handler<H: InlineQueryHandler + 'static + Send>(
        &mut self,
        handler: H,
    ) -> &mut Self {
        self.inline_query.push(Box::new(handler));
        self
    }

    /// Add chosen inline result handler
    pub fn add_chosen_inline_result_handler<H: ChosenInlineResultHandler + 'static + Send>(
        &mut self,
        handler: H,
    ) -> &mut Self {
        self.chosen_inline_result.push(Box::new(handler));
        self
    }

    /// Add callback query handler
    pub fn add_callback_query_handler<H: CallbackQueryHandler + 'static + Send>(
        &mut self,
        handler: H,
    ) -> &mut Self {
        self.callback_query.push(Box::new(handler));
        self
    }

    /// Add shipping query handler
    pub fn add_shipping_query_handler<H: ShippingQueryHandler + 'static + Send>(
        &mut self,
        handler: H,
    ) -> &mut Self {
        self.shipping_query.push(Box::new(handler));
        self
    }

    /// Add pre checkout query handler
    pub fn add_pre_checkout_query_handler<H: PreCheckoutQueryHandler + 'static + Send>(
        &mut self,
        handler: H,
    ) -> &mut Self {
        self.pre_checkout_query.push(Box::new(handler));
        self
    }

    /// Dispatch an update
    pub fn dispatch(&self, update: &Update) -> DispatcherFuture {
        DispatcherFuture::new(match update.kind {
            UpdateKind::Message(ref msg)
            | UpdateKind::EditedMessage(ref msg)
            | UpdateKind::ChannelPost(ref msg)
            | UpdateKind::EditedChannelPost(ref msg) => match msg.get_commands() {
                Some(commands) => self
                    .command
                    .iter()
                    .filter_map(|h| {
                        for command in &commands {
                            if h.accepts(command) {
                                return Some(h.handle(&self.api, &msg));
                            }
                        }
                        None
                    })
                    .collect(),
                None => self
                    .message
                    .iter()
                    .map(|h| h.handle(&self.api, &msg))
                    .collect(),
            },
            UpdateKind::InlineQuery(ref inline_query) => self
                .inline_query
                .iter()
                .map(|h| h.handle(&self.api, inline_query))
                .collect(),
            UpdateKind::ChosenInlineResult(ref chosen_inline_result) => self
                .chosen_inline_result
                .iter()
                .map(|h| h.handle(&self.api, chosen_inline_result))
                .collect(),
            UpdateKind::CallbackQuery(ref callback_query) => self
                .callback_query
                .iter()
                .map(|h| h.handle(&self.api, callback_query))
                .collect(),
            UpdateKind::ShippingQuery(ref shipping_query) => self
                .shipping_query
                .iter()
                .map(|h| h.handle(&self.api, shipping_query))
                .collect(),
            UpdateKind::PreCheckoutQuery(ref pre_checkout_query) => self
                .pre_checkout_query
                .iter()
                .map(|h| h.handle(&self.api, pre_checkout_query))
                .collect(),
        })
    }
}

/// Dispatcher future
#[must_use = "futures do nothing unless polled"]
pub struct DispatcherFuture {
    items: Vec<HandlerFuture>,
    current: usize,
}

impl DispatcherFuture {
    fn new(items: Vec<HandlerFuture>) -> DispatcherFuture {
        DispatcherFuture { items, current: 0 }
    }
}

impl Future for DispatcherFuture {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let items_len = self.items.len();
        if items_len == 0 || self.current > items_len {
            return Ok(Async::Ready(()));
        }
        let f = &mut self.items[self.current];
        match f.poll() {
            Ok(Async::Ready(HandlerResult::Continue)) => {
                self.current += 1;
                Ok(Async::NotReady)
            }
            Ok(Async::Ready(HandlerResult::Stop)) => Ok(Async::Ready(())),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(err) => Err(err),
        }
    }
}
