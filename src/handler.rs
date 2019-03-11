use failure::Error;
use futures::{future, Future, Poll};
use shellwords::{split, MismatchedQuotes};
use std::{collections::HashMap, string::FromUtf16Error};
use tgbot::types::{
    CallbackQuery, ChosenInlineResult, InlineQuery, Message, PreCheckoutQuery, ShippingQuery, Update, UpdateKind,
};

/// A regular update handler
pub struct Handler<S> {
    kind: HandlerKind<S>,
}

impl<S> Handler<S> {
    fn new(kind: HandlerKind<S>) -> Self {
        Self { kind }
    }

    /// Create message handler
    pub fn message<H>(handler: H) -> Self
    where
        H: MessageHandler<S> + 'static + Send + Sync,
    {
        Self::new(HandlerKind::Message(Box::new(handler)))
    }

    /// Create inline query handler
    pub fn inline_query<H>(handler: H) -> Self
    where
        H: InlineQueryHandler<S> + 'static + Send + Sync,
    {
        Self::new(HandlerKind::InlineQuery(Box::new(handler)))
    }

    /// Create chosen inline result handler
    pub fn chosen_inline_result<H>(handler: H) -> Self
    where
        H: ChosenInlineResultHandler<S> + 'static + Send + Sync,
    {
        Self::new(HandlerKind::ChosenInlineResult(Box::new(handler)))
    }

    /// Create callback query handler
    pub fn callback_query<H>(handler: H) -> Self
    where
        H: CallbackQueryHandler<S> + 'static + Send + Sync,
    {
        Self::new(HandlerKind::CallbackQuery(Box::new(handler)))
    }

    /// Create shipping query handler
    pub fn shipping_query<H>(handler: H) -> Self
    where
        H: ShippingQueryHandler<S> + 'static + Send + Sync,
    {
        Self::new(HandlerKind::ShippingQuery(Box::new(handler)))
    }

    /// Create pre checkout query handler
    pub fn pre_checkout_query<H>(handler: H) -> Self
    where
        H: PreCheckoutQueryHandler<S> + 'static + Send + Sync,
    {
        Self::new(HandlerKind::PreCheckoutQuery(Box::new(handler)))
    }

    /// Create a regular update handler
    pub fn update<H>(handler: H) -> Self
    where
        H: UpdateHandler<S> + 'static + Send + Sync,
    {
        Self::new(HandlerKind::Update(Box::new(handler)))
    }
}

enum HandlerKind<S> {
    Message(Box<MessageHandler<S> + Send + Sync>),
    InlineQuery(Box<InlineQueryHandler<S> + Send + Sync>),
    ChosenInlineResult(Box<ChosenInlineResultHandler<S> + Send + Sync>),
    CallbackQuery(Box<CallbackQueryHandler<S> + Send + Sync>),
    ShippingQuery(Box<ShippingQueryHandler<S> + Send + Sync>),
    PreCheckoutQuery(Box<PreCheckoutQueryHandler<S> + Send + Sync>),
    Update(Box<UpdateHandler<S> + Send + Sync>),
}

impl<S> Handler<S> {
    pub(super) fn handle(&mut self, context: &mut S, update: &Update) -> HandlerFuture {
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
        impl<S, F, R> $handler<S> for F
        where
            F: FnMut(&mut S, &$arg) -> R,
            R: Into<HandlerFuture>,
        {
            fn handle(&mut self, context: &mut S, arg: &$arg) -> HandlerFuture {
                (self)(context, arg).into()
            }
        }
    };
}

/// A regular message handler
pub trait MessageHandler<S> {
    /// Handles a message
    fn handle(&mut self, context: &mut S, message: &Message) -> HandlerFuture;
}

impl_func!(MessageHandler(Message));

/// An inline query handler
pub trait InlineQueryHandler<S> {
    /// Handles a query
    fn handle(&mut self, context: &mut S, query: &InlineQuery) -> HandlerFuture;
}

impl_func!(InlineQueryHandler(InlineQuery));

/// A chosen inline result handler
pub trait ChosenInlineResultHandler<S> {
    /// Handles a result
    fn handle(&mut self, context: &mut S, result: &ChosenInlineResult) -> HandlerFuture;
}

impl_func!(ChosenInlineResultHandler(ChosenInlineResult));

/// A callback query handler
pub trait CallbackQueryHandler<S> {
    /// Handles a query
    fn handle(&mut self, context: &mut S, query: &CallbackQuery) -> HandlerFuture;
}

impl_func!(CallbackQueryHandler(CallbackQuery));

/// A shipping query handler
pub trait ShippingQueryHandler<S> {
    /// Handles a query
    fn handle(&mut self, context: &mut S, query: &ShippingQuery) -> HandlerFuture;
}

impl_func!(ShippingQueryHandler(ShippingQuery));

/// A pre checkout query handler
pub trait PreCheckoutQueryHandler<S> {
    /// Handles a query
    fn handle(&mut self, context: &mut S, query: &PreCheckoutQuery) -> HandlerFuture;
}

impl_func!(PreCheckoutQueryHandler(PreCheckoutQuery));

/// A regular update handler
pub trait UpdateHandler<S> {
    /// Handles an update
    fn handle(&mut self, context: &mut S, update: &Update) -> HandlerFuture;
}

impl_func!(UpdateHandler(Update));

/// A simple commands handler
///
/// Just takes a first command from a message and ignores others.
/// Assumes that all text after command is arguments.
/// Use quotes in order to include spaces in argument: `'hello word'`
pub struct CommandsHandler<S> {
    handlers: HashMap<String, Box<CommandHandler<S> + Send + Sync>>,
    not_found_handler: Option<Box<CommandHandler<S> + Send + Sync>>,
}

impl<S> Default for CommandsHandler<S> {
    fn default() -> Self {
        Self {
            handlers: HashMap::new(),
            not_found_handler: None,
        }
    }
}

impl<S> CommandsHandler<S> {
    /// Add command handler
    ///
    /// # Arguments
    ///
    /// - name - Command name (starts with `/`)
    /// - handler - Command handler
    pub fn add_handler<I, H>(mut self, name: I, handler: H) -> Self
    where
        I: Into<String>,
        H: CommandHandler<S> + 'static + Send + Sync,
    {
        self.handlers.insert(name.into(), Box::new(handler));
        self
    }

    /// Add not found command handler
    pub fn not_found_handler<H>(mut self, handler: H) -> Self
    where
        H: CommandHandler<S> + 'static + Send + Sync,
    {
        self.not_found_handler = Some(Box::new(handler));
        self
    }
}

/// An error occurred when parsing command arguments
#[derive(Debug, Fail)]
pub enum CommandError {
    /// Can not decode command arguments
    #[fail(display = "Can not decode command arguments: {:?}", _0)]
    FromUtf16(#[cause] FromUtf16Error),
    /// Can not split arguments: quotes mismatched
    #[fail(display = "Can not split command arguments: quotes mismatched")]
    MismatchedQuotes,
}

impl<S> MessageHandler<S> for CommandsHandler<S> {
    fn handle(&mut self, context: &mut S, message: &Message) -> HandlerFuture {
        match (&message.commands, message.get_text()) {
            (Some(ref commands), Some(ref text)) => {
                // tgbot guarantees that commands will never be empty, but we must be sure
                assert!(!commands.is_empty());
                // just take first command and ignore others
                let command = &commands[0];
                // assume that all text after command is arguments
                let pos = command.data.offset + command.data.length;
                // pos is UTF-16 offset
                let input: Vec<u16> = text.data.encode_utf16().skip(pos).collect();
                match String::from_utf16(&input) {
                    Ok(input) => match split(&input) {
                        Ok(args) => match self.handlers.get_mut(&command.command) {
                            Some(handler) => handler.handle(context, message, args),
                            None => match self.not_found_handler {
                                Some(ref mut handler) => handler.handle(context, message, args),
                                None => ().into(),
                            },
                        },
                        Err(MismatchedQuotes) => Err(CommandError::MismatchedQuotes).into(),
                    },
                    Err(err) => Err(CommandError::FromUtf16(err)).into(),
                }
            }
            _ => ().into(),
        }
    }
}

/// Actual command handler
pub trait CommandHandler<S> {
    /// Handles a command
    fn handle(&mut self, context: &mut S, message: &Message, args: Vec<String>) -> HandlerFuture;
}

impl<S, F, R> CommandHandler<S> for F
where
    F: FnMut(&mut S, &Message, Vec<String>) -> R,
    R: Into<HandlerFuture>,
{
    fn handle(&mut self, context: &mut S, message: &Message, args: Vec<String>) -> HandlerFuture {
        (self)(context, message, args).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::dispatcher::Dispatcher;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    struct Counter {
        calls: Arc<AtomicUsize>,
    }

    impl Counter {
        fn new() -> Self {
            Self {
                calls: Arc::new(AtomicUsize::new(0)),
            }
        }

        fn inc_calls(&self) {
            self.calls.fetch_add(1, Ordering::SeqCst);
        }

        fn get_calls(&self) -> usize {
            self.calls.load(Ordering::SeqCst)
        }
    }

    struct Args {
        items: Vec<String>,
    }

    impl Args {
        fn new() -> Self {
            Self {
                items: vec![],
            }
        }

        fn extend(&mut self, items: Vec<String>) {
            self.items.extend(items);
        }
    }

    fn command_handler(context: &mut Args, _message: &Message, args: Vec<String>) -> HandlerFuture {
        context.extend(args);
        ().into()
    }

    fn handle_message(context: &mut Counter, _message: &Message) -> HandlerFuture {
        context.inc_calls();
        ().into()
    }

    fn handle_inline_query(context: &mut Counter, _query: &InlineQuery) -> HandlerFuture {
        context.inc_calls();
        ().into()
    }

    fn handle_chose_inline_result(context: &mut Counter, _result: &ChosenInlineResult) -> HandlerFuture {
        context.inc_calls();
        ().into()
    }

    fn handle_callback_query(context: &mut Counter, _query: &CallbackQuery) -> HandlerFuture {
        context.inc_calls();
        ().into()
    }

    fn handle_shipping_query(context: &mut Counter, _query: &ShippingQuery) -> HandlerFuture {
        context.inc_calls();
        ().into()
    }

    fn handle_precheckout_query(context: &mut Counter, _query: &PreCheckoutQuery) -> HandlerFuture {
        context.inc_calls();
        ().into()
    }

    fn handle_update(context: &mut Counter, _update: &Update) -> HandlerFuture {
        context.inc_calls();
        ().into()
    }

    fn parse_update(data: &str) -> Update {
        serde_json::from_str(data).unwrap()
    }

    #[test]
    fn test_dispatch_message() {
        let mut dispatcher = Dispatcher::new(
            vec![],
            vec![Handler::message(handle_message), Handler::update(handle_update)],
            Counter::new(),
        );
        for data in &[
            r#"{
                "update_id": 1,
                "message": {
                    "message_id": 1111,
                    "date": 0,
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "private", "first_name": "test"},
                    "text": "test message from private chat"
                }
            }"#,
            r#"{
                "update_id": 1,
                "edited_message": {
                    "message_id": 1111,
                    "date": 0,
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "private", "first_name": "test"},
                    "text": "test edited message from private chat",
                    "edit_date": 1213
                }
            }"#,
            r#"{
                "update_id": 1,
                "channel_post": {
                    "message_id": 1111,
                    "date": 0,
                    "author_signature": "test",
                    "chat": {"id": 1, "type": "channel", "title": "channeltitle", "username": "channelusername"},
                    "text": "test message from channel"
                }
            }"#,
            r#"{
                "update_id": 1,
                "edited_channel_post": {
                    "message_id": 1111,
                    "date": 0,
                    "chat": {"id": 1, "type": "channel", "title": "channeltitle", "username": "channelusername"},
                    "text": "test edited message from channel",
                    "edit_date": 1213
                }
            }"#,
        ] {
            let update = parse_update(data);
            dispatcher.dispatch(update).wait().unwrap();
        }
        assert_eq!(dispatcher.context.lock().unwrap().get_calls(), 8);
    }

    #[test]
    fn test_dispatch_inline_query() {
        let mut dispatcher = Dispatcher::new(
            vec![],
            vec![
                Handler::inline_query(handle_inline_query),
                Handler::update(handle_update),
            ],
            Counter::new(),
        );
        let update = parse_update(
            r#"
                {
                    "update_id": 1,
                    "inline_query": {
                        "id": "id",
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "query": "query",
                        "offset": "offset"
                    }
                }
            "#,
        );
        dispatcher.dispatch(update).wait().unwrap();
        assert_eq!(dispatcher.context.lock().unwrap().get_calls(), 2);
    }

    #[test]
    fn test_dispatch_chosen_inline_result() {
        let mut dispatcher = Dispatcher::new(
            vec![],
            vec![
                Handler::chosen_inline_result(handle_chose_inline_result),
                Handler::update(handle_update),
            ],
            Counter::new(),
        );
        let update = parse_update(
            r#"
                {
                    "update_id": 1,
                    "chosen_inline_result": {
                        "result_id": "id",
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "query": "query"
                    }
                }
            "#,
        );
        dispatcher.dispatch(update).wait().unwrap();
        assert_eq!(dispatcher.context.lock().unwrap().get_calls(), 2);
    }

    #[test]
    fn test_dispatch_callback_query() {
        let mut dispatcher = Dispatcher::new(
            vec![],
            vec![
                Handler::callback_query(handle_callback_query),
                Handler::update(handle_update),
            ],
            Counter::new(),
        );
        let update = parse_update(
            r#"
                {
                    "update_id": 1,
                    "callback_query": {
                        "id": "id",
                        "from": {"id": 1, "is_bot": false, "first_name": "test"}
                    }
                }
            "#,
        );
        dispatcher.dispatch(update).wait().unwrap();
        assert_eq!(dispatcher.context.lock().unwrap().get_calls(), 2);
    }

    #[test]
    fn test_dispatch_shipping_query() {
        let mut dispatcher = Dispatcher::new(
            vec![],
            vec![
                Handler::shipping_query(handle_shipping_query),
                Handler::update(handle_update),
            ],
            Counter::new(),
        );
        let update = parse_update(
            r#"
                {
                    "update_id": 1,
                    "shipping_query": {
                        "id": "id",
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "invoice_payload": "payload",
                        "shipping_address": {
                            "country_code": "RU",
                            "state": "State",
                            "city": "City",
                            "street_line1": "Line 1",
                            "street_line2": "Line 2",
                            "post_code": "Post Code"
                        }
                    }
                }
            "#,
        );
        dispatcher.dispatch(update).wait().unwrap();
        assert_eq!(dispatcher.context.lock().unwrap().get_calls(), 2);
    }

    #[test]
    fn test_dispatch_pre_checkout_query() {
        let mut dispatcher = Dispatcher::new(
            vec![],
            vec![
                Handler::pre_checkout_query(handle_precheckout_query),
                Handler::update(handle_update),
            ],
            Counter::new(),
        );
        let update = parse_update(
            r#"
                {
                    "update_id": 1,
                    "pre_checkout_query": {
                        "id": "id",
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "currency": "RUB",
                        "total_amount": 145,
                        "invoice_payload": "payload"
                    }
                }
            "#,
        );
        dispatcher.dispatch(update).wait().unwrap();
        assert_eq!(dispatcher.context.lock().unwrap().get_calls(), 2);
    }

    #[test]
    fn test_commands_handler() {
        let update = parse_update(
            r#"{
                "update_id": 1,
                "message": {
                    "message_id": 1111,
                    "date": 0,
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "private", "first_name": "test"},
                    "text": "/testcommand 'arg1 v' arg2",
                    "entities": [
                        {"type": "bot_command", "offset": 0, "length": 12}
                    ]
                }
            }"#,
        );
        let commands = CommandsHandler::default().add_handler("/testcommand", command_handler);
        let mut dispatcher = Dispatcher::new(vec![], vec![Handler::message(commands)], Args::new());
        dispatcher.dispatch(update.clone()).wait().unwrap();
        let items: &Vec<String> = &dispatcher.context.lock().unwrap().items;
        assert_eq!(items, &vec![String::from("arg1 v"), String::from("arg2")]);
    }
}
