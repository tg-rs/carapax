use crate::context::Context;
use failure::Error;
use futures::{future, Future, Poll};
use tgbot::types::{
    CallbackQuery, ChosenInlineResult, InlineQuery, Message, PreCheckoutQuery, ShippingQuery, Update, UpdateKind,
};

/// A regular update handler
pub struct Handler {
    kind: HandlerKind,
}

impl Handler {
    fn new(kind: HandlerKind) -> Self {
        Self { kind }
    }

    /// Create message handler
    pub fn message<H>(handler: H) -> Self
    where
        H: MessageHandler + 'static + Send + Sync,
    {
        Self::new(HandlerKind::Message(Box::new(handler)))
    }

    /// Create inline query handler
    pub fn inline_query<H>(handler: H) -> Self
    where
        H: InlineQueryHandler + 'static + Send + Sync,
    {
        Self::new(HandlerKind::InlineQuery(Box::new(handler)))
    }

    /// Create chosen inline result handler
    pub fn chosen_inline_result<H>(handler: H) -> Self
    where
        H: ChosenInlineResultHandler + 'static + Send + Sync,
    {
        Self::new(HandlerKind::ChosenInlineResult(Box::new(handler)))
    }

    /// Create callback query handler
    pub fn callback_query<H>(handler: H) -> Self
    where
        H: CallbackQueryHandler + 'static + Send + Sync,
    {
        Self::new(HandlerKind::CallbackQuery(Box::new(handler)))
    }

    /// Create shipping query handler
    pub fn shipping_query<H>(handler: H) -> Self
    where
        H: ShippingQueryHandler + 'static + Send + Sync,
    {
        Self::new(HandlerKind::ShippingQuery(Box::new(handler)))
    }

    /// Create pre checkout query handler
    pub fn pre_checkout_query<H>(handler: H) -> Self
    where
        H: PreCheckoutQueryHandler + 'static + Send + Sync,
    {
        Self::new(HandlerKind::PreCheckoutQuery(Box::new(handler)))
    }

    /// Create a regular update handler
    pub fn update<H>(handler: H) -> Self
    where
        H: UpdateHandler + 'static + Send + Sync,
    {
        Self::new(HandlerKind::Update(Box::new(handler)))
    }
}

enum HandlerKind {
    Message(Box<MessageHandler + Send + Sync>),
    InlineQuery(Box<InlineQueryHandler + Send + Sync>),
    ChosenInlineResult(Box<ChosenInlineResultHandler + Send + Sync>),
    CallbackQuery(Box<CallbackQueryHandler + Send + Sync>),
    ShippingQuery(Box<ShippingQueryHandler + Send + Sync>),
    PreCheckoutQuery(Box<PreCheckoutQueryHandler + Send + Sync>),
    Update(Box<UpdateHandler + Send + Sync>),
}

impl Handler {
    pub(super) fn handle(&mut self, context: &Context, update: &Update) -> HandlerFuture {
        macro_rules! handle {
            ($kind:ident($val:ident)) => {
                if let HandlerKind::$kind(ref mut handler) = self.kind {
                    return handler.handle(context, $val);
                }
            };
        }

        handle!(Update(update));

        match update.kind {
            UpdateKind::Message(ref msg)
            | UpdateKind::EditedMessage(ref msg)
            | UpdateKind::ChannelPost(ref msg)
            | UpdateKind::EditedChannelPost(ref msg) => handle!(Message(msg)),
            UpdateKind::InlineQuery(ref val) => handle!(InlineQuery(val)),
            UpdateKind::ChosenInlineResult(ref val) => handle!(ChosenInlineResult(val)),
            UpdateKind::CallbackQuery(ref val) => handle!(CallbackQuery(val)),
            UpdateKind::ShippingQuery(ref val) => handle!(ShippingQuery(val)),
            UpdateKind::PreCheckoutQuery(ref val) => handle!(PreCheckoutQuery(val)),
        }
        ().into()
    }
}

/// A handler future
#[must_use = "futures do nothing unless polled"]
pub struct HandlerFuture {
    inner: Box<Future<Item = (), Error = Error> + Send>,
}

impl HandlerFuture {
    /// Creates a new handler future
    pub fn new<F>(f: F) -> HandlerFuture
    where
        F: Future<Item = (), Error = Error> + 'static + Send,
    {
        HandlerFuture { inner: Box::new(f) }
    }
}

impl From<()> for HandlerFuture {
    fn from(_: ()) -> HandlerFuture {
        HandlerFuture::new(future::ok(()))
    }
}

impl<E> From<Result<(), E>> for HandlerFuture
where
    E: Into<Error>,
{
    fn from(result: Result<(), E>) -> Self {
        HandlerFuture::new(future::result(result.map_err(Into::into)))
    }
}

impl Future for HandlerFuture {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

macro_rules! impl_func {
    ($handler:ident($arg:ident)) => {
        impl<F, R> $handler for F
        where
            F: FnMut(&Context, &$arg) -> R,
            R: Into<HandlerFuture>,
        {
            fn handle(&mut self, context: &Context, arg: &$arg) -> HandlerFuture {
                (self)(context, arg).into()
            }
        }
    };
}

/// A regular message handler
pub trait MessageHandler {
    /// Handles a message
    fn handle(&mut self, context: &Context, message: &Message) -> HandlerFuture;
}

impl_func!(MessageHandler(Message));

/// An inline query handler
pub trait InlineQueryHandler {
    /// Handles a query
    fn handle(&mut self, context: &Context, query: &InlineQuery) -> HandlerFuture;
}

impl_func!(InlineQueryHandler(InlineQuery));

/// A chosen inline result handler
pub trait ChosenInlineResultHandler {
    /// Handles a result
    fn handle(&mut self, context: &Context, result: &ChosenInlineResult) -> HandlerFuture;
}

impl_func!(ChosenInlineResultHandler(ChosenInlineResult));

/// A callback query handler
pub trait CallbackQueryHandler {
    /// Handles a query
    fn handle(&mut self, context: &Context, query: &CallbackQuery) -> HandlerFuture;
}

impl_func!(CallbackQueryHandler(CallbackQuery));

/// A shipping query handler
pub trait ShippingQueryHandler {
    /// Handles a query
    fn handle(&mut self, context: &Context, query: &ShippingQuery) -> HandlerFuture;
}

impl_func!(ShippingQueryHandler(ShippingQuery));

/// A pre checkout query handler
pub trait PreCheckoutQueryHandler {
    /// Handles a query
    fn handle(&mut self, context: &Context, query: &PreCheckoutQuery) -> HandlerFuture;
}

impl_func!(PreCheckoutQueryHandler(PreCheckoutQuery));

/// A regular update handler
pub trait UpdateHandler {
    /// Handles an update
    fn handle(&mut self, context: &Context, update: &Update) -> HandlerFuture;
}

impl_func!(UpdateHandler(Update));
