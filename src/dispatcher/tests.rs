use super::*;
use crate::api::Api;
use crate::types::{
    CallbackQuery, ChosenInlineResult, InlineQuery, Message, PreCheckoutQuery, ShippingQuery,
    Update,
};
use tokio::runtime::current_thread::block_on_all;

struct MockHandler {
    result: HandlerResult,
}

impl Default for MockHandler {
    fn default() -> Self {
        MockHandler {
            result: HandlerResult::Continue,
        }
    }
}

impl MessageHandler for MockHandler {
    fn handle(&mut self, _api: &Api, _message: &Message) -> HandlerFuture {
        self.result.into()
    }
}

impl InlineQueryHandler for MockHandler {
    fn handle(&mut self, _api: &Api, _query: &InlineQuery) -> HandlerFuture {
        self.result.into()
    }
}

impl ChosenInlineResultHandler for MockHandler {
    fn handle(&mut self, _api: &Api, _result: &ChosenInlineResult) -> HandlerFuture {
        self.result.into()
    }
}

impl CallbackQueryHandler for MockHandler {
    fn handle(&mut self, _api: &Api, _query: &CallbackQuery) -> HandlerFuture {
        self.result.into()
    }
}

impl ShippingQueryHandler for MockHandler {
    fn handle(&mut self, _api: &Api, _query: &ShippingQuery) -> HandlerFuture {
        self.result.into()
    }
}

impl PreCheckoutQueryHandler for MockHandler {
    fn handle(&mut self, _api: &Api, _query: &PreCheckoutQuery) -> HandlerFuture {
        self.result.into()
    }
}

fn create_dispatcher() -> Dispatcher {
    Dispatcher::new(Api::create("test-dispatcher").expect("failed to create api"))
}

fn parse_update(data: &str) -> Update {
    serde_json::from_str(data).unwrap()
}

fn get_dispatch_result(f: DispatcherFuture) -> usize {
    block_on_all(f).unwrap()
}

#[test]
fn test_dispatch_message() {
    let mut dispatcher = create_dispatcher().add_message_handler(MockHandler::default());
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
        assert_eq!(get_dispatch_result(dispatcher.dispatch(&update)), 1);
    }
}

#[test]
fn test_dispatch_command() {
    let handler = CommandHandler::new("/testcommand", MockHandler::default());
    let mut dispatcher = create_dispatcher().add_command_handler(handler);
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
        assert_eq!(get_dispatch_result(dispatcher.dispatch(&update)), 1);
    }

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
    assert_eq!(get_dispatch_result(dispatcher.dispatch(&update)), 0);
}

#[test]
fn test_dispatch_inline_query() {
    let mut dispatcher = create_dispatcher().add_inline_query_handler(MockHandler::default());
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
    assert_eq!(get_dispatch_result(dispatcher.dispatch(&update)), 1);
}

#[test]
fn test_dispatch_chosen_inline_result() {
    let mut dispatcher =
        create_dispatcher().add_chosen_inline_result_handler(MockHandler::default());
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
    assert_eq!(get_dispatch_result(dispatcher.dispatch(&update)), 1);
}

#[test]
fn test_dispatch_callback_query() {
    let mut dispatcher = create_dispatcher().add_callback_query_handler(MockHandler::default());
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
    assert_eq!(get_dispatch_result(dispatcher.dispatch(&update)), 1);
}

#[test]
fn test_dispatch_shipping_query() {
    let mut dispatcher = create_dispatcher().add_shipping_query_handler(MockHandler::default());
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
    assert_eq!(get_dispatch_result(dispatcher.dispatch(&update)), 1);
}

#[test]
fn test_dispatch_pre_checkout_query() {
    let mut dispatcher = create_dispatcher().add_pre_checkout_query_handler(MockHandler::default());
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
    assert_eq!(get_dispatch_result(dispatcher.dispatch(&update)), 1);
}

#[test]
fn test_stop_handler() {
    use self::HandlerResult::*;
    for (executed_num, results) in &[
        (2, &[Continue, Stop, Continue, Continue]),
        (1, &[Stop, Continue, Continue, Continue]),
        (3, &[Continue, Continue, Stop, Continue]),
    ] {
        let mut dispatcher = create_dispatcher();
        for result in *results {
            dispatcher = dispatcher.add_message_handler(MockHandler { result: *result });
        }
        let f = dispatcher.dispatch(&parse_update(
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
        ));
        assert_eq!(get_dispatch_result(f), *executed_num);
    }
}

struct MockMiddleware {
    before_result: MiddlewareResult,
    after_result: MiddlewareResult,
}

impl Middleware for MockMiddleware {
    fn before(&mut self, _api: &Api, _update: &Update) -> MiddlewareFuture {
        self.before_result.into()
    }

    fn after(&mut self, _api: &Api, _update: &Update) -> MiddlewareFuture {
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

    let mut dispatcher = create_dispatcher()
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
        .add_message_handler(MockHandler {
            result: HandlerResult::Continue,
        });
    let f = dispatcher.dispatch(&update);
    assert_eq!(get_dispatch_result(f), 5);

    let mut dispatcher = create_dispatcher()
        .add_middleware(MockMiddleware {
            before_result: MiddlewareResult::Continue,
            after_result: MiddlewareResult::Stop,
        })
        .add_middleware(MockMiddleware {
            before_result: MiddlewareResult::Continue,
            after_result: MiddlewareResult::Continue,
        })
        .add_message_handler(MockHandler {
            result: HandlerResult::Stop,
        })
        .add_message_handler(MockHandler {
            result: HandlerResult::Continue,
        });
    let f = dispatcher.dispatch(&update);
    assert_eq!(get_dispatch_result(f), 4);
}
