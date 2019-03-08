use crate::types::{
    BotCommand, CallbackQuery, ChosenInlineResult, InlineQuery, Message, PreCheckoutQuery,
    ShippingQuery, Update, UpdateKind,
};
use failure::Error;
use futures::{future, Future, Poll};

/// A regular update handler
pub struct Handler<C> {
    kind: HandlerKind<C>,
}

impl<C> Handler<C> {
    fn new(kind: HandlerKind<C>) -> Self {
        Self { kind }
    }

    /// Create message handler
    pub fn message<H>(handler: H) -> Self
    where
        H: MessageHandler<C> + 'static + Send + Sync,
    {
        Self::new(HandlerKind::Message(Box::new(handler)))
    }

    /// Create command handler
    pub fn command(handler: CommandHandler<C>) -> Self {
        Self::new(HandlerKind::Command(handler))
    }

    /// Create inline query handler
    pub fn inline_query<H>(handler: H) -> Self
    where
        H: InlineQueryHandler<C> + 'static + Send + Sync,
    {
        Self::new(HandlerKind::InlineQuery(Box::new(handler)))
    }

    /// Create chosen inline result handler
    pub fn chosen_inline_result<H>(handler: H) -> Self
    where
        H: ChosenInlineResultHandler<C> + 'static + Send + Sync,
    {
        Self::new(HandlerKind::ChosenInlineResult(Box::new(handler)))
    }

    /// Create callback query handler
    pub fn callback_query<H>(handler: H) -> Self
    where
        H: CallbackQueryHandler<C> + 'static + Send + Sync,
    {
        Self::new(HandlerKind::CallbackQuery(Box::new(handler)))
    }

    /// Create shipping query handler
    pub fn shipping_query<H>(handler: H) -> Self
    where
        H: ShippingQueryHandler<C> + 'static + Send + Sync,
    {
        Self::new(HandlerKind::ShippingQuery(Box::new(handler)))
    }

    /// Create pre checkout query handler
    pub fn pre_checkout_query<H>(handler: H) -> Self
    where
        H: PreCheckoutQueryHandler<C> + 'static + Send + Sync,
    {
        Self::new(HandlerKind::PreCheckoutQuery(Box::new(handler)))
    }

    /// Create a regular update handler
    pub fn update<H>(handler: H) -> Self
    where
        H: UpdateHandler<C> + 'static + Send + Sync,
    {
        Self::new(HandlerKind::Update(Box::new(handler)))
    }
}

enum HandlerKind<C> {
    Message(Box<MessageHandler<C> + Send + Sync>),
    Command(CommandHandler<C>),
    InlineQuery(Box<InlineQueryHandler<C> + Send + Sync>),
    ChosenInlineResult(Box<ChosenInlineResultHandler<C> + Send + Sync>),
    CallbackQuery(Box<CallbackQueryHandler<C> + Send + Sync>),
    ShippingQuery(Box<ShippingQueryHandler<C> + Send + Sync>),
    PreCheckoutQuery(Box<PreCheckoutQueryHandler<C> + Send + Sync>),
    Update(Box<UpdateHandler<C> + Send + Sync>),
}

impl<C> Handler<C> {
    pub(super) fn handle(
        &mut self,
        context: &C,
        update: &Update,
        commands: &Option<Vec<BotCommand>>,
    ) -> HandlerFuture {
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
            | UpdateKind::EditedChannelPost(ref msg) => match commands {
                Some(commands) => {
                    if let HandlerKind::Command(ref mut handler) = self.kind {
                        for command in commands {
                            if handler.accepts(command) {
                                return handler.handle(context, msg);
                            }
                        }
                    }
                }
                None => handle!(Message(msg)),
            },
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
        HandlerFuture::new(future::result(result.map_err(|e| e.into())))
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
        impl<C, F, R> $handler<C> for F
        where
            F: FnMut(&C, &$arg) -> R,
            R: Into<HandlerFuture>,
        {
            fn handle(&mut self, context: &C, arg: &$arg) -> HandlerFuture {
                (self)(context, arg).into()
            }
        }
    };
}

/// A regular message handler
pub trait MessageHandler<C> {
    /// Handles a message
    fn handle(&mut self, context: &C, message: &Message) -> HandlerFuture;
}

impl_func!(MessageHandler(Message));

/// A command handler
pub struct CommandHandler<C> {
    name: String,
    handler: Box<MessageHandler<C> + Send + Sync>,
}

impl<C> CommandHandler<C> {
    /// Creates a new command handler
    ///
    /// # Arguments
    ///
    /// - name - command name (starts with /)
    /// - handler - a message handler
    pub fn new<S, H>(name: S, handler: H) -> Self
    where
        S: Into<String>,
        H: MessageHandler<C> + 'static + Send + Sync,
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

impl<C> MessageHandler<C> for CommandHandler<C> {
    fn handle(&mut self, context: &C, message: &Message) -> HandlerFuture {
        self.handler.handle(context, message)
    }
}

/// An inline query handler
pub trait InlineQueryHandler<C> {
    /// Handles a query
    fn handle(&mut self, context: &C, query: &InlineQuery) -> HandlerFuture;
}

impl_func!(InlineQueryHandler(InlineQuery));

/// A chosen inline result handler
pub trait ChosenInlineResultHandler<C> {
    /// Handles a result
    fn handle(&mut self, context: &C, result: &ChosenInlineResult) -> HandlerFuture;
}

impl_func!(ChosenInlineResultHandler(ChosenInlineResult));

/// A callback query handler
pub trait CallbackQueryHandler<C> {
    /// Handles a query
    fn handle(&mut self, context: &C, query: &CallbackQuery) -> HandlerFuture;
}

impl_func!(CallbackQueryHandler(CallbackQuery));

/// A shipping query handler
pub trait ShippingQueryHandler<C> {
    /// Handles a query
    fn handle(&mut self, context: &C, query: &ShippingQuery) -> HandlerFuture;
}

impl_func!(ShippingQueryHandler(ShippingQuery));

/// A pre checkout query handler
pub trait PreCheckoutQueryHandler<C> {
    /// Handles a query
    fn handle(&mut self, context: &C, query: &PreCheckoutQuery) -> HandlerFuture;
}

impl_func!(PreCheckoutQueryHandler(PreCheckoutQuery));

/// A regular update handler
pub trait UpdateHandler<C> {
    /// Handles an update
    fn handle(&mut self, context: &C, update: &Update) -> HandlerFuture;
}

impl_func!(UpdateHandler(Update));
