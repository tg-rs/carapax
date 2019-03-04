use self::middleware::{IterMiddlewareFuture, Middleware, MiddlewareResult};
use crate::api::Api;
use crate::types::{Update, UpdateKind};
use failure::Error;
use futures::{future, task, Async, Future, Poll, Stream};

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
    pub fn dispatch(&mut self, update: &Update) -> DispatcherFuture {
        // Use 'for' loop instead of iterators to avoid "cannot borrow self as mutable twice"
        macro_rules! collect {
            ($vec:ident, $func:ident, $var:expr) => {{
                let mut futures = vec![];
                for h in &mut self.$vec {
                    futures.push(h.$func(&self.api, $var))
                }
                futures
            }};
            ($vec:ident, $data:expr) => {
                collect!($vec, handle, $data)
            };
            ($name:ident) => {
                collect!($name, $name)
            };
        }

        let before = IterMiddlewareFuture::new(collect!(middleware, before, &update));
        let main = IterHandlerFuture::new(match update.kind {
            UpdateKind::Message(ref msg)
            | UpdateKind::EditedMessage(ref msg)
            | UpdateKind::ChannelPost(ref msg)
            | UpdateKind::EditedChannelPost(ref msg) => match msg.get_commands() {
                Some(commands) => {
                    let mut futures = vec![];
                    for handler in &mut self.command {
                        for command in &commands {
                            if handler.accepts(command) {
                                futures.push(handler.handle(&self.api, &msg));
                            }
                        }
                    }
                    futures
                }
                None => collect!(message, &msg),
            },
            UpdateKind::InlineQuery(ref inline_query) => collect!(inline_query),
            UpdateKind::ChosenInlineResult(ref chosen_inline_result) => {
                collect!(chosen_inline_result)
            }
            UpdateKind::CallbackQuery(ref callback_query) => collect!(callback_query),
            UpdateKind::ShippingQuery(ref shipping_query) => collect!(shipping_query),
            UpdateKind::PreCheckoutQuery(ref pre_checkout_query) => collect!(pre_checkout_query),
        });
        let after = IterMiddlewareFuture::new(collect!(middleware, after, &update));
        DispatcherFuture::new(before, main, after)
    }

    /// Spawns a polling stream
    pub fn start_polling(mut self) {
        tokio::run(future::lazy(move || {
            self.api
                .get_updates()
                .for_each(move |update| {
                    let fut = self.dispatch(&update);
                    self.api.spawn(fut);
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
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Ok(Async::Ready((MiddlewareResult::Stop, num))) => {
                    self.state = After(num);
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Ok(Async::NotReady) => {
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Err(err) => Err(err),
            },
            Main(before_num) => match self.main.poll() {
                Ok(Async::Ready(num)) => {
                    self.state = After(before_num + num);
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Ok(Async::NotReady) => {
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Err(err) => Err(err),
            },
            After(total_num) => match self.after.poll() {
                Ok(Async::Ready((_, num))) => Ok(Async::Ready(total_num + num)),
                Ok(Async::NotReady) => {
                    task::current().notify();
                    Ok(Async::NotReady)
                }
                Err(err) => Err(err),
            },
        }
    }
}

#[derive(Debug)]
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
