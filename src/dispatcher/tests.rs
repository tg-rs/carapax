use super::*;
use crate::types::{CallbackQuery, ChosenInlineResult, InlineQuery, Message, PreCheckoutQuery, ShippingQuery, Update};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

struct MockContext {
    calls: Arc<AtomicUsize>,
}

impl MockContext {
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

fn handle_message(context: &MockContext, _message: &Message) -> HandlerFuture {
    context.inc_calls();
    ().into()
}

fn handle_inline_query(context: &MockContext, _query: &InlineQuery) -> HandlerFuture {
    context.inc_calls();
    ().into()
}

fn handle_chose_inline_result(context: &MockContext, _result: &ChosenInlineResult) -> HandlerFuture {
    context.inc_calls();
    ().into()
}

fn handle_callback_query(context: &MockContext, _query: &CallbackQuery) -> HandlerFuture {
    context.inc_calls();
    ().into()
}

fn handle_shipping_query(context: &MockContext, _query: &ShippingQuery) -> HandlerFuture {
    context.inc_calls();
    ().into()
}

fn handle_precheckout_query(context: &MockContext, _query: &PreCheckoutQuery) -> HandlerFuture {
    context.inc_calls();
    ().into()
}

fn handle_update(context: &MockContext, _update: &Update) -> HandlerFuture {
    context.inc_calls();
    ().into()
}

fn parse_update(data: &str) -> Update {
    serde_json::from_str(data).unwrap()
}

#[test]
fn test_dispatch_message() {
    let mut dispatcher = DispatcherBuilder::new()
        .add_handler(Handler::message(handle_message))
        .add_handler(Handler::update(handle_update))
        .build(MockContext::new());
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
    assert_eq!(dispatcher.context.get_calls(), 8);
}

#[test]
fn test_dispatch_command() {
    let mut dispatcher = DispatcherBuilder::new()
        .add_handler(Handler::command(CommandHandler::new("/testcommand", handle_message)))
        .add_handler(Handler::update(handle_update))
        .build(MockContext::new());
    for data in &[
        r#"{
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "/testcommand arg1 arg2",
                "entities": [
                    {"type": "bot_command", "offset": 0, "length": 12}
                ]
            }
        }"#,
        r#"{
            "update_id": 1,
            "edited_message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "/testcommand",
                "edit_date": 1213,
                "entities": [
                    {"type": "bot_command", "offset": 0, "length": 12}
                ]
            }
        }"#,
        r#"{
            "update_id": 1,
            "channel_post": {
                "message_id": 1111,
                "date": 0,
                "author_signature": "test",
                "chat": {"id": 1, "type": "channel", "title": "channeltitle", "username": "channelusername"},
                "text": "i /testcommand i",
                "entities": [
                    {"type": "bot_command", "offset": 2, "length": 12}
                ]
            }
        }"#,
        r#"{
            "update_id": 1,
            "edited_channel_post": {
                "message_id": 1111,
                "date": 0,
                "chat": {"id": 1, "type": "channel", "title": "channeltitle", "username": "channelusername"},
                "text": "/testcommand",
                "entities": [
                    {"type": "bot_command", "offset": 0, "length": 12}
                ],
                "edit_date": 1213
            }
        }"#,
    ] {
        let update = parse_update(data);
        dispatcher.dispatch(update).wait().unwrap();
    }
    assert_eq!(dispatcher.context.get_calls(), 8);

    // command not found
    let update = parse_update(
        r#"{
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "/unknowncommand",
                "entities": [
                    {"type": "bot_command", "offset": 0, "length": 15}
                ]
            }
        }"#,
    );
    dispatcher.dispatch(update).wait().unwrap();
    assert_eq!(dispatcher.context.get_calls(), 9);
}

#[test]
fn test_dispatch_inline_query() {
    let mut dispatcher = DispatcherBuilder::new()
        .add_handler(Handler::inline_query(handle_inline_query))
        .add_handler(Handler::update(handle_update))
        .build(MockContext::new());
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
    assert_eq!(dispatcher.context.get_calls(), 2);
}

#[test]
fn test_dispatch_chosen_inline_result() {
    let mut dispatcher = DispatcherBuilder::new()
        .add_handler(Handler::chosen_inline_result(handle_chose_inline_result))
        .add_handler(Handler::update(handle_update))
        .build(MockContext::new());
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
    assert_eq!(dispatcher.context.get_calls(), 2);
}

#[test]
fn test_dispatch_callback_query() {
    let mut dispatcher = DispatcherBuilder::new()
        .add_handler(Handler::callback_query(handle_callback_query))
        .add_handler(Handler::update(handle_update))
        .build(MockContext::new());
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
    assert_eq!(dispatcher.context.get_calls(), 2);
}

#[test]
fn test_dispatch_shipping_query() {
    let mut dispatcher = DispatcherBuilder::new()
        .add_handler(Handler::shipping_query(handle_shipping_query))
        .add_handler(Handler::update(handle_update))
        .build(MockContext::new());
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
    assert_eq!(dispatcher.context.get_calls(), 2);
}

#[test]
fn test_dispatch_pre_checkout_query() {
    let mut dispatcher = DispatcherBuilder::new()
        .add_handler(Handler::pre_checkout_query(handle_precheckout_query))
        .add_handler(Handler::update(handle_update))
        .build(MockContext::new());
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
    assert_eq!(dispatcher.context.get_calls(), 2);
}

struct MockMiddleware {
    before_result: MiddlewareResult,
    after_result: MiddlewareResult,
}

impl Middleware<MockContext> for MockMiddleware {
    fn before(&mut self, context: &MockContext, _update: &Update) -> MiddlewareFuture {
        context.inc_calls();
        self.before_result.into()
    }

    fn after(&mut self, context: &MockContext, _update: &Update) -> MiddlewareFuture {
        context.inc_calls();
        self.after_result.into()
    }
}

#[test]
fn test_middleware() {
    let update = parse_update(
        r#"{
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test"
            }
        }"#,
    );

    let mut dispatcher = DispatcherBuilder::new()
        .add_middleware(MockMiddleware {
            before_result: MiddlewareResult::Continue,
            after_result: MiddlewareResult::Continue,
        })
        .add_middleware(MockMiddleware {
            before_result: MiddlewareResult::Stop,
            after_result: MiddlewareResult::Continue,
        })
        .add_middleware(MockMiddleware {
            before_result: MiddlewareResult::Continue,
            after_result: MiddlewareResult::Stop,
        })
        .add_middleware(MockMiddleware {
            before_result: MiddlewareResult::Continue,
            after_result: MiddlewareResult::Continue,
        })
        .add_handler(Handler::message(handle_message))
        .build(MockContext::new());
    dispatcher.dispatch(update.clone()).wait().unwrap();
    assert_eq!(dispatcher.context.get_calls(), 5);

    let mut dispatcher = DispatcherBuilder::new()
        .add_middleware(MockMiddleware {
            before_result: MiddlewareResult::Continue,
            after_result: MiddlewareResult::Stop,
        })
        .add_middleware(MockMiddleware {
            before_result: MiddlewareResult::Continue,
            after_result: MiddlewareResult::Continue,
        })
        .add_handler(Handler::message(handle_message))
        .build(MockContext::new());
    dispatcher.dispatch(update).wait().unwrap();
    assert_eq!(dispatcher.context.get_calls(), 4);
}

#[derive(Debug, Fail)]
#[fail(display = "Test error")]
struct ErrorMock;

struct ErrorMiddleware;

impl Middleware<MockContext> for ErrorMiddleware {
    fn before(&mut self, context: &MockContext, _update: &Update) -> MiddlewareFuture {
        context.inc_calls();
        Err(ErrorMock).into()
    }

    fn after(&mut self, context: &MockContext, _update: &Update) -> MiddlewareFuture {
        context.inc_calls();
        Err(ErrorMock).into()
    }
}

fn handle_update_error(context: &MockContext, _update: &Update) -> HandlerFuture {
    context.inc_calls();
    Err(ErrorMock).into()
}

#[test]
fn test_error_strategy() {
    let update = parse_update(
        r#"{
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test"
            }
        }"#,
    );

    // Aborted on first call by default
    let mut dispatcher = DispatcherBuilder::new()
        .add_middleware(ErrorMiddleware)
        .add_middleware(ErrorMiddleware)
        .add_handler(Handler::update(handle_update_error))
        .build(MockContext::new());
    dispatcher.dispatch(update.clone()).wait().unwrap_err();
    assert_eq!(dispatcher.context.get_calls(), 1);

    // Aborted on handler call by default
    let mut dispatcher = DispatcherBuilder::new()
        .middleware_error_strategy(ErrorStrategy::Ignore)
        .add_middleware(ErrorMiddleware)
        .add_middleware(ErrorMiddleware)
        .add_handler(Handler::update(handle_update_error))
        .build(MockContext::new());
    dispatcher.dispatch(update.clone()).wait().unwrap_err();
    assert_eq!(dispatcher.context.get_calls(), 3);

    // Ignore all errors
    let mut dispatcher = DispatcherBuilder::new()
        .middleware_error_strategy(ErrorStrategy::Ignore)
        .handler_error_strategy(ErrorStrategy::Ignore)
        .add_middleware(ErrorMiddleware)
        .add_middleware(ErrorMiddleware)
        .add_handler(Handler::update(handle_update_error))
        .build(MockContext::new());
    dispatcher.dispatch(update.clone()).wait().unwrap();
    assert_eq!(dispatcher.context.get_calls(), 5);
}
