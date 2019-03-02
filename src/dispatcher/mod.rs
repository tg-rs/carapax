use crate::api::Api;
use crate::types::{Update, UpdateKind};
use failure::Error;
use futures::{future, Async, Future, Poll, Stream};

mod handler;
mod middleware;
#[cfg(test)]
mod tests;

pub use self::handler::*;
pub use self::middleware::*;

/// Dispatcher
pub struct Dispatcher {
    api: Api,
    middleware: Vec<Box<Middleware + Send + Sync>>,
    message: Vec<Box<MessageHandler + Send + Sync>>,
    command: Vec<CommandHandler>,
    inline_query: Vec<Box<InlineQueryHandler + Send + Sync>>,
    chosen_inline_result: Vec<Box<ChosenInlineResultHandler + Send + Sync>>,
    callback_query: Vec<Box<CallbackQueryHandler + Send + Sync>>,
    shipping_query: Vec<Box<ShippingQueryHandler + Send + Sync>>,
    pre_checkout_query: Vec<Box<PreCheckoutQueryHandler + Send + Sync>>,
}

impl Dispatcher {
    /// Creates a new dispatcher
    pub fn new(api: Api) -> Self {
        Dispatcher {
            api,
            middleware: vec![],
            message: vec![],
            command: vec![],
            inline_query: vec![],
            chosen_inline_result: vec![],
            callback_query: vec![],
            shipping_query: vec![],
            pre_checkout_query: vec![],
        }
    }

    /// Add middleware handler
    pub fn add_middleware<H>(mut self, handler: H) -> Self
    where
        H: Middleware + 'static + Send + Sync,
    {
        self.middleware.push(Box::new(handler));
        self
    }

    /// Add message handler
    pub fn add_message_handler<H>(mut self, handler: H) -> Self
    where
        H: MessageHandler + 'static + Send + Sync,
    {
        self.message.push(Box::new(handler));
        self
    }

    /// Add command handler
    pub fn add_command_handler(mut self, handler: CommandHandler) -> Self {
        self.command.push(handler);
        self
    }

    /// Add inline query handler
    pub fn add_inline_query_handler<H>(mut self, handler: H) -> Self
    where
        H: InlineQueryHandler + 'static + Send + Sync,
    {
        self.inline_query.push(Box::new(handler));
        self
    }

    /// Add chosen inline result handler
    pub fn add_chosen_inline_result_handler<H>(mut self, handler: H) -> Self
    where
        H: ChosenInlineResultHandler + 'static + Send + Sync,
    {
        self.chosen_inline_result.push(Box::new(handler));
        self
    }

    /// Add callback query handler
    pub fn add_callback_query_handler<H>(mut self, handler: H) -> Self
    where
        H: CallbackQueryHandler + 'static + Send + Sync,
    {
        self.callback_query.push(Box::new(handler));
        self
    }

    /// Add shipping query handler
    pub fn add_shipping_query_handler<H>(mut self, handler: H) -> Self
    where
        H: ShippingQueryHandler + 'static + Send + Sync,
    {
        self.shipping_query.push(Box::new(handler));
        self
    }

    /// Add pre checkout query handler
    pub fn add_pre_checkout_query_handler<H>(mut self, handler: H) -> Self
    where
        H: PreCheckoutQueryHandler + 'static + Send + Sync,
    {
        self.pre_checkout_query.push(Box::new(handler));
        self
    }

    /// Dispatch an update
    pub fn dispatch(&self, update: &Update) -> DispatcherFuture {
        let before = IterMiddlewareFuture::new(
            self.middleware
                .iter()
                .map(|h| h.before(&self.api, &update))
                .collect(),
        );
        let main = IterHandlerFuture::new(match update.kind {
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
        });
        let after = IterMiddlewareFuture::new(
            self.middleware
                .iter()
                .map(|h| h.after(&self.api, &update))
                .collect(),
        );
        DispatcherFuture::new(before, main, after)
    }

    /// Spawns a polling stream
    pub fn start_polling(self) {
        tokio::run(future::lazy(move || {
            self.api
                .get_updates()
                .for_each(move |update| {
                    self.api.spawn(self.dispatch(&update));
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
///
/// Returns number of executed handlers on success
/// (including middlewares, before and after separately)
#[must_use = "futures do nothing unless polled"]
pub struct DispatcherFuture {
    before: IterMiddlewareFuture,
    main: IterHandlerFuture,
    after: IterMiddlewareFuture,
    state: DispatcherFutureState,
}

impl DispatcherFuture {
    fn new(
        before: IterMiddlewareFuture,
        main: IterHandlerFuture,
        after: IterMiddlewareFuture,
    ) -> DispatcherFuture {
        DispatcherFuture {
            before,
            main,
            after,
            state: Default::default(),
        }
    }
}

impl Future for DispatcherFuture {
    type Item = usize;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::DispatcherFutureState::*;
        match self.state {
            Before => match self.before.poll() {
                Ok(Async::Ready((MiddlewareResult::Continue, num))) => {
                    self.state = Main(num);
                    Ok(Async::NotReady)
                }
                Ok(Async::Ready((MiddlewareResult::Stop, num))) => {
                    self.state = After(num);
                    Ok(Async::NotReady)
                }
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Err(err) => Err(err),
            },
            Main(before_num) => match self.main.poll() {
                Ok(Async::Ready(num)) => {
                    self.state = After(before_num + num);
                    Ok(Async::NotReady)
                }
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Err(err) => Err(err),
            },
            After(total_num) => match self.after.poll() {
                Ok(Async::Ready((_, num))) => Ok(Async::Ready(total_num + num)),
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Err(err) => Err(err),
            },
        }
    }
}

enum DispatcherFutureState {
    Before,
    Main(usize),
    After(usize),
}

impl Default for DispatcherFutureState {
    fn default() -> Self {
        DispatcherFutureState::Before
    }
}
