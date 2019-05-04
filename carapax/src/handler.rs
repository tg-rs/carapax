use crate::context::Context;
use failure::Error;
use futures::{future, Future};
use regex::Regex;
use shellwords::{split, MismatchedQuotes};
use std::{collections::HashMap, marker::PhantomData, ops::Deref, string::FromUtf16Error};
use tgbot::types::{
    CallbackQuery, ChosenInlineResult, InlineQuery, Message, Poll, PreCheckoutQuery, ShippingQuery, Text, Update,
    UpdateKind,
};

/// Result of a handler
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerResult {
    /// Continue propagation
    ///
    /// Next handler (if exists) will run after current has finished
    Continue,
    /// Stop propagation
    ///
    /// Next handler (if exists) will not run after current has finished
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
        F: Future<Item = HandlerResult, Error = Error> + Send + 'static,
    {
        HandlerFuture { inner: Box::new(f) }
    }
}

impl From<HandlerResult> for HandlerFuture {
    fn from(result: HandlerResult) -> HandlerFuture {
        HandlerFuture::new(future::ok(result))
    }
}

impl From<()> for HandlerFuture {
    fn from(_: ()) -> Self {
        Self::from(HandlerResult::Continue)
    }
}

impl<E> From<Result<HandlerResult, E>> for HandlerFuture
where
    E: Into<Error>,
{
    fn from(result: Result<HandlerResult, E>) -> Self {
        HandlerFuture::new(future::result(result.map_err(Into::into)))
    }
}

impl Future for HandlerFuture {
    type Item = HandlerResult;
    type Error = Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

/// A trait to create object from Update
pub trait FromUpdate: Sized {
    /// Creates a reference from update
    fn from_update(update: Update) -> Option<Self>;
}

impl FromUpdate for Update {
    fn from_update(update: Update) -> Option<Self> {
        Some(update)
    }
}

impl FromUpdate for Message {
    fn from_update(update: Update) -> Option<Self> {
        match update.kind {
            UpdateKind::Message(msg)
            | UpdateKind::EditedMessage(msg)
            | UpdateKind::ChannelPost(msg)
            | UpdateKind::EditedChannelPost(msg) => Some(msg),
            _ => None,
        }
    }
}

impl FromUpdate for InlineQuery {
    fn from_update(update: Update) -> Option<Self> {
        match update.kind {
            UpdateKind::InlineQuery(query) => Some(query),
            _ => None,
        }
    }
}

impl FromUpdate for ChosenInlineResult {
    fn from_update(update: Update) -> Option<Self> {
        match update.kind {
            UpdateKind::ChosenInlineResult(result) => Some(result),
            _ => None,
        }
    }
}

impl FromUpdate for CallbackQuery {
    fn from_update(update: Update) -> Option<Self> {
        match update.kind {
            UpdateKind::CallbackQuery(query) => Some(query),
            _ => None,
        }
    }
}

impl FromUpdate for ShippingQuery {
    fn from_update(update: Update) -> Option<Self> {
        match update.kind {
            UpdateKind::ShippingQuery(query) => Some(query),
            _ => None,
        }
    }
}

impl FromUpdate for PreCheckoutQuery {
    fn from_update(update: Update) -> Option<Self> {
        match update.kind {
            UpdateKind::PreCheckoutQuery(query) => Some(query),
            _ => None,
        }
    }
}

impl FromUpdate for Poll {
    fn from_update(update: Update) -> Option<Self> {
        match update.kind {
            UpdateKind::Poll(poll) => Some(poll),
            _ => None,
        }
    }
}

/// A handler
pub trait Handler {
    /// A handler item
    type Item: FromUpdate;
    /// A handler result
    type Result: Into<HandlerFuture>;

    /// Handles an [item] with [context] and returns [result]
    ///
    /// [item]: Handler::Item
    /// [context]: Handler::Context
    /// [result]: Handler::Result
    fn handle(&self, context: &mut Context, item: Self::Item) -> Self::Result;
}

pub(crate) type BoxedHandler = Box<Handler<Item = Update, Result = HandlerFuture> + Send + Sync + 'static>;

pub(crate) struct HandlerWrapper<H> {
    handler: H,
}

impl<H> HandlerWrapper<H> {
    pub fn boxed(handler: H) -> Box<Self> {
        Box::new(Self { handler })
    }
}

impl<H, I, R> Handler for Box<H>
where
    H: Handler<Item = I, Result = R> + Sized,
    I: FromUpdate,
    R: Into<HandlerFuture>,
{
    type Item = I;
    type Result = R;

    fn handle(&self, context: &mut Context, item: Self::Item) -> Self::Result {
        self.deref().handle(context, item)
    }
}

impl<H, I, R> Handler for HandlerWrapper<H>
where
    H: Handler<Item = I, Result = R>,
    I: FromUpdate,
    R: Into<HandlerFuture>,
{
    type Item = Update;
    type Result = HandlerFuture;

    fn handle(&self, context: &mut Context, item: Self::Item) -> Self::Result {
        match I::from_update(item) {
            Some(item) => self.handler.handle(context, item).into(),
            _ => HandlerResult::Continue.into(),
        }
    }
}

/// A wrapper around function
pub struct FnHandler<F, I, R>
where
    F: Fn(&mut Context, I) -> R,
    I: FromUpdate,
    R: Into<HandlerFuture>,
{
    f: F,
    _item: PhantomData<I>,
}

impl<F, I, R> FnHandler<F, I, R>
where
    F: Fn(&mut Context, I) -> R,
    I: FromUpdate,
    R: Into<HandlerFuture>,
{
    #[cfg(test)]
    pub(crate) fn wrapped(f: F) -> Box<HandlerWrapper<Self>> {
        HandlerWrapper::boxed(Self::from(f))
    }
}

impl<F, I, R> From<F> for FnHandler<F, I, R>
where
    F: Fn(&mut Context, I) -> R,
    I: FromUpdate,
    R: Into<HandlerFuture>,
{
    fn from(f: F) -> Self {
        Self { f, _item: PhantomData }
    }
}

impl<F, I, R> Handler for FnHandler<F, I, R>
where
    F: Fn(&mut Context, I) -> R,
    I: FromUpdate,
    R: Into<HandlerFuture>,
{
    type Item = I;
    type Result = R;

    fn handle(&self, context: &mut Context, item: Self::Item) -> Self::Result {
        (self.f)(context, item)
    }
}

/// A simple commands handler
///
/// Just takes a first command from a message and ignores others.
/// Assumes that all text after command is arguments.
/// Use quotes in order to include spaces in argument: `'hello word'`
#[derive(Default)]
pub struct CommandsHandler {
    handlers: HashMap<String, BoxedCommandHandler>,
    not_found_handler: Option<BoxedCommandHandler>,
}

type BoxedCommandHandler = Box<CommandHandler<Result = HandlerFuture> + Send + Sync>;

impl CommandsHandler {
    /// Add command handler
    ///
    /// # Arguments
    ///
    /// - name - Command name (starts with `/`)
    /// - handler - Command handler
    pub fn add_handler<S, H, O>(mut self, name: S, handler: H) -> Self
    where
        S: Into<String>,
        H: CommandHandler<Result = O> + Send + Sync + 'static,
        O: Into<HandlerFuture>,
    {
        self.handlers.insert(name.into(), HandlerWrapper::boxed(handler));
        self
    }

    /// Add not found command handler
    pub fn not_found_handler<H, O>(mut self, handler: H) -> Self
    where
        H: CommandHandler<Result = O> + Send + Sync + 'static,
        O: Into<HandlerFuture>,
    {
        self.not_found_handler = Some(HandlerWrapper::boxed(handler));
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

impl Handler for CommandsHandler {
    type Item = Message;
    type Result = HandlerFuture;

    fn handle(&self, context: &mut Context, message: Self::Item) -> Self::Result {
        match (&message.commands, message.get_text()) {
            (Some(commands), Some(text)) => {
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
                        Ok(args) => match self.handlers.get(&command.command) {
                            Some(handler) => handler.handle(context, &message, args),
                            None => match self.not_found_handler {
                                Some(ref handler) => handler.handle(context, &message, args),
                                None => HandlerResult::Continue.into(),
                            },
                        },
                        Err(MismatchedQuotes) => Err(CommandError::MismatchedQuotes).into(),
                    },
                    Err(err) => Err(CommandError::FromUtf16(err)).into(),
                }
            }
            _ => HandlerResult::Continue.into(),
        }
    }
}

/// Actual command handler
pub trait CommandHandler {
    /// A handler result
    type Result: Into<HandlerFuture>;

    /// Handles command with [context] and returns [result]
    ///
    /// [context]: CommandHandler::Context
    /// [result]: CommandHandler::Result
    fn handle(&self, context: &mut Context, message: &Message, args: Vec<String>) -> Self::Result;
}

impl<H, O> CommandHandler for HandlerWrapper<H>
where
    H: CommandHandler<Result = O>,
    O: Into<HandlerFuture>,
{
    type Result = HandlerFuture;

    fn handle(&self, context: &mut Context, message: &Message, args: Vec<String>) -> Self::Result {
        self.handler.handle(context, message, args).into()
    }
}

/// A wrapper around function
pub struct FnCommandHandler<F, R>
where
    F: Fn(&mut Context, &Message, Vec<String>) -> R,
    R: Into<HandlerFuture>,
{
    f: F,
}

impl<F, R> From<F> for FnCommandHandler<F, R>
where
    F: Fn(&mut Context, &Message, Vec<String>) -> R,
    R: Into<HandlerFuture>,
{
    fn from(f: F) -> Self {
        Self { f }
    }
}

impl<F, R> CommandHandler for FnCommandHandler<F, R>
where
    F: Fn(&mut Context, &Message, Vec<String>) -> R,
    R: Into<HandlerFuture>,
{
    type Result = R;

    fn handle(&self, context: &mut Context, message: &Message, args: Vec<String>) -> Self::Result {
        (self.f)(context, message, args)
    }
}

/// Rule for text handler
pub trait TextRule {
    /// Whether handler should accept message with given text
    fn accepts(&self, text: &Text) -> bool;
}

#[doc(hidden)]
pub struct TextRuleContains {
    substring: String,
}

impl TextRule for TextRuleContains {
    fn accepts(&self, text: &Text) -> bool {
        text.data.contains(&self.substring)
    }
}

#[doc(hidden)]
pub struct TextRuleEquals {
    text: String,
}

impl TextRule for TextRuleEquals {
    fn accepts(&self, text: &Text) -> bool {
        text.data == self.text
    }
}

#[doc(hidden)]
pub struct TextRuleMatches {
    pattern: Regex,
}

impl TextRule for TextRuleMatches {
    fn accepts(&self, text: &Text) -> bool {
        self.pattern.is_match(&text.data)
    }
}

impl<F> TextRule for F
where
    F: Fn(&Text) -> bool,
{
    fn accepts(&self, text: &Text) -> bool {
        (self)(text)
    }
}

/// A rules based message text handler
pub struct TextHandler<R, H> {
    rule: R,
    handler: H,
}

impl<R, H> TextHandler<R, H> {
    /// Creates a new handler
    pub fn new(rule: R, handler: H) -> Self {
        Self { rule, handler }
    }
}

impl<H> TextHandler<TextRuleContains, H> {
    /// Create a handler for messages contains given text
    pub fn contains<S>(text: S, handler: H) -> Self
    where
        S: Into<String>,
    {
        Self::new(TextRuleContains { substring: text.into() }, handler)
    }
}

impl<H> TextHandler<TextRuleEquals, H> {
    /// Create a handler for messages equals given text
    pub fn equals<S>(text: S, handler: H) -> Self
    where
        S: Into<String>,
    {
        Self::new(TextRuleEquals { text: text.into() }, handler)
    }
}

impl<H> TextHandler<TextRuleMatches, H> {
    /// Create a handler for messages matches given text
    ///
    /// See [regex](https://docs.rs/regex) crate for more information about patterns
    pub fn matches<S>(pattern: S, handler: H) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        Ok(Self::new(
            TextRuleMatches {
                pattern: Regex::new(pattern.as_ref())?,
            },
            handler,
        ))
    }
}

impl<TR, H, R> Handler for TextHandler<TR, H>
where
    TR: TextRule,
    H: Handler<Item = Message, Result = R>,
    R: Into<HandlerFuture>,
{
    type Item = Message;
    type Result = HandlerFuture;

    fn handle(&self, context: &mut Context, message: Self::Item) -> Self::Result {
        if message.get_text().map(|text| self.rule.accepts(text)).unwrap_or(false) {
            self.handler.handle(context, message).into()
        } else {
            HandlerResult::Continue.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dispatcher::{Dispatcher, ErrorStrategy};
    use serde_json::{from_value, json};
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    use tgbot::Api;

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
            Self { items: vec![] }
        }

        fn extend(&mut self, items: Vec<String>) {
            self.items.extend(items);
        }
    }

    fn setup_context(context: &mut Context, _update: Update) {
        context.set(Args::new());
        context.set(Counter::new());
    }

    fn command_handler(context: &mut Context, _message: &Message, args: Vec<String>) {
        context.get_mut::<Args>().extend(args);
    }

    fn handle_message(context: &mut Context, _message: Message) {
        context.get_mut::<Counter>().inc_calls();
    }

    fn handle_inline_query(context: &mut Context, _query: InlineQuery) {
        context.get_mut::<Counter>().inc_calls();
    }

    fn handle_chose_inline_result(context: &mut Context, _result: ChosenInlineResult) {
        context.get_mut::<Counter>().inc_calls();
    }

    fn handle_callback_query(context: &mut Context, _query: CallbackQuery) {
        context.get_mut::<Counter>().inc_calls();
    }

    fn handle_shipping_query(context: &mut Context, _query: ShippingQuery) {
        context.get_mut::<Counter>().inc_calls();
    }

    fn handle_pre_checkout_query(context: &mut Context, _query: PreCheckoutQuery) {
        context.get_mut::<Counter>().inc_calls();
    }

    fn handle_poll(context: &mut Context, _poll: Poll) {
        context.get_mut::<Counter>().inc_calls();
    }

    fn handle_update(context: &mut Context, _update: Update) {
        context.get_mut::<Counter>().inc_calls();
    }

    #[test]
    fn dispatch_message() {
        let dispatcher = Dispatcher::new(
            Api::new("token").unwrap(),
            vec![
                FnHandler::wrapped(setup_context),
                FnHandler::wrapped(handle_message),
                FnHandler::wrapped(handle_update),
            ],
            ErrorStrategy::Abort,
        );
        for data in vec![
            json!({
                "update_id": 1,
                "message": {
                    "message_id": 1111,
                    "date": 0,
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "private", "first_name": "test"},
                    "text": "test message from private chat"
                }
            }),
            json!({
                "update_id": 1,
                "edited_message": {
                    "message_id": 1111,
                    "date": 0,
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "private", "first_name": "test"},
                    "text": "test edited message from private chat",
                    "edit_date": 1213
                }
            }),
            json!({
                "update_id": 1,
                "channel_post": {
                    "message_id": 1111,
                    "date": 0,
                    "author_signature": "test",
                    "chat": {"id": 1, "type": "channel", "title": "channeltitle", "username": "channelusername"},
                    "text": "test message from channel"
                }
            }),
            json!({
                "update_id": 1,
                "edited_channel_post": {
                    "message_id": 1111,
                    "date": 0,
                    "chat": {"id": 1, "type": "channel", "title": "channeltitle", "username": "channelusername"},
                    "text": "test edited message from channel",
                    "edit_date": 1213
                }
            }),
        ] {
            let update = from_value(data).unwrap();
            let context = dispatcher.dispatch(update).wait().unwrap();
            assert_eq!(context.get::<Counter>().get_calls(), 2);
        }
    }

    #[test]
    fn dispatch_inline_query() {
        let dispatcher = Dispatcher::new(
            Api::new("token").unwrap(),
            vec![
                FnHandler::wrapped(setup_context),
                FnHandler::wrapped(handle_inline_query),
                FnHandler::wrapped(handle_update),
            ],
            ErrorStrategy::Abort,
        );
        let update = from_value(json!(
            {
                "update_id": 1,
                "inline_query": {
                    "id": "id",
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "query": "query",
                    "offset": "offset"
                }
            }
        ))
        .unwrap();
        let context = dispatcher.dispatch(update).wait().unwrap();
        assert_eq!(context.get::<Counter>().get_calls(), 2);
    }

    #[test]
    fn dispatch_chosen_inline_result() {
        let dispatcher = Dispatcher::new(
            Api::new("token").unwrap(),
            vec![
                FnHandler::wrapped(setup_context),
                FnHandler::wrapped(handle_chose_inline_result),
                FnHandler::wrapped(handle_update),
            ],
            ErrorStrategy::Abort,
        );
        let update = from_value(json!(
            {
                "update_id": 1,
                "chosen_inline_result": {
                    "result_id": "id",
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "query": "query"
                }
            }
        ))
        .unwrap();
        let context = dispatcher.dispatch(update).wait().unwrap();
        assert_eq!(context.get::<Counter>().get_calls(), 2);
    }

    #[test]
    fn dispatch_callback_query() {
        let dispatcher = Dispatcher::new(
            Api::new("token").unwrap(),
            vec![
                FnHandler::wrapped(setup_context),
                FnHandler::wrapped(handle_callback_query),
                FnHandler::wrapped(handle_update),
            ],
            ErrorStrategy::Abort,
        );
        let update = from_value(json!(
            {
                "update_id": 1,
                "callback_query": {
                    "id": "id",
                    "from": {"id": 1, "is_bot": false, "first_name": "test"}
                }
            }
        ))
        .unwrap();
        let context = dispatcher.dispatch(update).wait().unwrap();
        assert_eq!(context.get::<Counter>().get_calls(), 2);
    }

    #[test]
    fn dispatch_shipping_query() {
        let dispatcher = Dispatcher::new(
            Api::new("token").unwrap(),
            vec![
                FnHandler::wrapped(setup_context),
                FnHandler::wrapped(handle_shipping_query),
                FnHandler::wrapped(handle_update),
            ],
            ErrorStrategy::Abort,
        );
        let update = from_value(json!(
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
        ))
        .unwrap();
        let context = dispatcher.dispatch(update).wait().unwrap();
        assert_eq!(context.get::<Counter>().get_calls(), 2);
    }

    #[test]
    fn dispatch_pre_checkout_query() {
        let dispatcher = Dispatcher::new(
            Api::new("token").unwrap(),
            vec![
                FnHandler::wrapped(setup_context),
                FnHandler::wrapped(handle_pre_checkout_query),
                FnHandler::wrapped(handle_update),
            ],
            ErrorStrategy::Abort,
        );
        let update = from_value(json!(
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
        ))
        .unwrap();
        let context = dispatcher.dispatch(update).wait().unwrap();
        assert_eq!(context.get::<Counter>().get_calls(), 2);
    }

    #[test]
    fn dispatch_poll() {
        let dispatcher = Dispatcher::new(
            Api::new("token").unwrap(),
            vec![
                FnHandler::wrapped(setup_context),
                FnHandler::wrapped(handle_poll),
                FnHandler::wrapped(handle_update),
            ],
            ErrorStrategy::Abort,
        );
        let update = from_value(json!(
            {
                "update_id": 1,
                "poll": {
                    "id": "id",
                    "question": "test poll",
                    "options": [
                        {"text": "opt 1", "voter_count": 1},
                        {"text": "opt 2", "voter_count": 2}
                    ],
                    "is_closed": false
                }
            }
        ))
        .unwrap();
        let context = dispatcher.dispatch(update).wait().unwrap();
        assert_eq!(context.get::<Counter>().get_calls(), 2);
    }

    #[test]
    fn commands_handler() {
        let update: Update = from_value(json!(
            {
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
            }
        ))
        .unwrap();
        let commands = CommandsHandler::default().add_handler("/testcommand", FnCommandHandler::from(command_handler));
        let dispatcher = Dispatcher::new(
            Api::new("token").unwrap(),
            vec![FnHandler::wrapped(setup_context), HandlerWrapper::boxed(commands)],
            ErrorStrategy::Abort,
        );
        let context = dispatcher.dispatch(update.clone()).wait().unwrap();
        let args = context.get::<Args>();
        assert_eq!(args.items, vec![String::from("arg1 v"), String::from("arg2")]);
    }

    #[test]
    fn text_handler() {
        let cases: Vec<(_, BoxedHandler)> = vec![
            (
                json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 1111,
                        "date": 0,
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 1, "type": "private", "first_name": "test"},
                        "text": "test substring contains"
                    }
                }),
                HandlerWrapper::boxed(TextHandler::contains("substring", FnHandler::from(handle_message))),
            ),
            (
                json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 1111,
                        "date": 0,
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 1, "type": "private", "first_name": "test"},
                        "text": "test equals"
                    }
                }),
                HandlerWrapper::boxed(TextHandler::equals("test equals", FnHandler::from(handle_message))),
            ),
            (
                json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 1111,
                        "date": 0,
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 1, "type": "private", "first_name": "test"},
                        "text": "test matches"
                    }
                }),
                HandlerWrapper::boxed(TextHandler::matches("matches$", FnHandler::from(handle_message)).unwrap()),
            ),
            (
                json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 1111,
                        "date": 0,
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 1, "type": "private", "first_name": "test"},
                        "text": "test predicate"
                    }
                }),
                HandlerWrapper::boxed(TextHandler::new(
                    |text: &Text| text.data.contains("predicate"),
                    FnHandler::from(handle_message),
                )),
            ),
        ];
        for (update, handler) in cases {
            let update = from_value(update).unwrap();
            let dispatcher = Dispatcher::new(
                Api::new("token").unwrap(),
                vec![FnHandler::wrapped(setup_context), handler],
                ErrorStrategy::Abort,
            );
            let context = dispatcher.dispatch(update).wait().unwrap();
            assert_eq!(context.get::<Counter>().get_calls(), 1);
        }
    }

    #[test]
    fn update_skipped() {
        let dispatcher = Dispatcher::new(
            Api::new("token").unwrap(),
            vec![FnHandler::wrapped(setup_context), FnHandler::wrapped(handle_message)],
            ErrorStrategy::Abort,
        );
        let update = from_value(json!(
            {
                "update_id": 1,
                "poll": {
                    "id": "id",
                    "question": "test poll",
                    "options": [
                        {"text": "opt 1", "voter_count": 1},
                        {"text": "opt 2", "voter_count": 2}
                    ],
                    "is_closed": false
                }
            }
        ))
        .unwrap();
        let context = dispatcher.dispatch(update).wait().unwrap();
        assert_eq!(context.get::<Counter>().get_calls(), 0);
    }
}
