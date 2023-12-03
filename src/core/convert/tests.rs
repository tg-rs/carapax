use std::sync::Arc;

use crate::core::context::Context;

use super::*;

#[tokio::test]
async fn empty_tuple() {
    let update: Update = serde_json::from_value(serde_json::json!(
        {
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test",
            }
        }
    ))
    .unwrap();
    let input = HandlerInput::from(update);
    assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(matches!(<()>::try_from_input(input).await, Ok(Some(()))));
}

#[tokio::test]
async fn context_ref() {
    let mut context = Context::default();
    context.insert(3usize);
    let update: Update = serde_json::from_value(serde_json::json!(
        {
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test",
            }
        }
    ))
    .unwrap();
    let input = HandlerInput {
        update,
        context: Arc::new(context),
    };
    assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
    assert_eq!(
        <Ref<usize>>::try_from_input(input.clone()).await.unwrap().as_deref(),
        Some(&3)
    );
    assert!(matches!(
        <Ref<()>>::try_from_input(input.clone()).await,
        Err(ConvertInputError::Context(_))
    ));
}

#[tokio::test]
async fn chat_id() {
    let update: Update = serde_json::from_value(serde_json::json!(
        {
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test",
            }
        }
    ))
    .unwrap();
    let input = HandlerInput::from(update);
    assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
    assert_eq!(ChatPeerId::try_from_input(input).await, Ok(Some(1.into())));
}

#[tokio::test]
async fn user() {
    let update: Update = serde_json::from_value(serde_json::json!(
        {
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test",
            }
        }
    ))
    .unwrap();
    let input = HandlerInput::from(update);
    assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(User::try_from_input(input).await.unwrap().is_some());
}

#[tokio::test]
async fn message() {
    for data in [
        serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test message from private chat"
            }
        }),
        serde_json::json!({
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
        serde_json::json!({
            "update_id": 1,
            "channel_post": {
                "message_id": 1111,
                "date": 0,
                "author_signature": "test",
                "chat": {"id": 1, "type": "channel", "title": "channel title", "username": "channel_username"},
                "text": "test message from channel"
            }
        }),
        serde_json::json!({
            "update_id": 1,
            "edited_channel_post": {
                "message_id": 1111,
                "date": 0,
                "chat": {"id": 1, "type": "channel", "title": "channel title", "username": "channel_username"},
                "text": "test edited message from channel",
                "edit_date": 1213
            }
        }),
    ] {
        let update: Update = serde_json::from_value(data).unwrap();
        let input = HandlerInput::from(update);
        assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
        assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
        assert!(Message::try_from_input(input).await.unwrap().is_some());
    }
}

#[tokio::test]
async fn command() {
    let update: Update = serde_json::from_value(serde_json::json!(
        {
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "/test",
                "entities": [
                    {"type": "bot_command", "offset": 0, "length": 5}
                ]
            }
        }
    ))
    .unwrap();
    let input = HandlerInput::from(update);
    assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Command::try_from_input(input).await.unwrap().is_some());
}

#[tokio::test]
async fn inline_query() {
    let update: Update = serde_json::from_value(serde_json::json!(
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
    let input = HandlerInput::from(update);
    assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(InlineQuery::try_from_input(input).await.unwrap().is_some());
}

#[tokio::test]
async fn chosen_inline_result() {
    let update: Update = serde_json::from_value(serde_json::json!(
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
    let input = HandlerInput::from(update);
    assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(ChosenInlineResult::try_from_input(input).await.unwrap().is_some());
}

#[tokio::test]
async fn callback_query() {
    let update: Update = serde_json::from_value(serde_json::json!(
        {
            "update_id": 1,
            "callback_query": {
                "id": "id",
                "from": {"id": 1, "is_bot": false, "first_name": "test"}
            }
        }
    ))
    .unwrap();
    let input = HandlerInput::from(update);
    assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(CallbackQuery::try_from_input(input).await.unwrap().is_some());
}

#[tokio::test]
async fn shipping_query() {
    let update: Update = serde_json::from_value(serde_json::json!(
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
    let input = HandlerInput::from(update);
    assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(ShippingQuery::try_from_input(input).await.unwrap().is_some());
}

#[tokio::test]
async fn pre_checkout_query() {
    let update: Update = serde_json::from_value(serde_json::json!(
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
    let input = HandlerInput::from(update);
    assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(PreCheckoutQuery::try_from_input(input).await.unwrap().is_some());
}

#[tokio::test]
async fn poll() {
    let update: Update = serde_json::from_value(serde_json::json!(
        {
            "update_id": 1,
            "poll": {
                "id": "id",
                "question": "test poll",
                "options": [
                    {"text": "opt 1", "voter_count": 1},
                    {"text": "opt 2", "voter_count": 2}
                ],
                "is_closed": false,
                "total_voter_count": 3,
                "is_anonymous": true,
                "type": "regular",
                "allows_multiple_answers": false
            }
        }
    ))
    .unwrap();
    let input = HandlerInput::from(update);
    assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Poll::try_from_input(input).await.unwrap().is_some());
}

#[tokio::test]
async fn poll_answer() {
    let update: Update = serde_json::from_value(serde_json::json!(
        {
            "update_id": 1,
            "poll_answer": {
                "poll_id": "poll-id",
                "user": {
                    "id": 1,
                    "first_name": "Jamie",
                    "is_bot": false
                },
                "option_ids": [0],
            }
        }
    ))
    .unwrap();
    let input = HandlerInput::from(update);
    assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(PollAnswer::try_from_input(input).await.unwrap().is_some());
}

#[tokio::test]
async fn bot_status() {
    let update: Update = serde_json::from_value(serde_json::json!(
        {
            "update_id": 1,
            "my_chat_member": {
                "chat": {
                    "id": 1,
                    "type": "group",
                    "title": "group title"
                },
                "from": {
                    "id": 1,
                    "is_bot": true,
                    "first_name": "firstname"
                },
                "date": 0,
                "old_chat_member": {
                    "status": "member",
                    "user": {
                        "id": 2,
                        "is_bot": true,
                        "first_name": "firstname"
                    }
                },
                "new_chat_member": {
                    "status": "kicked",
                    "user": {
                        "id": 2,
                        "is_bot": true,
                        "first_name": "firstname",
                    },
                    "until_date": 0
                }
            }
        }
    ))
    .unwrap();
    let input = HandlerInput::from(update);
    assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(ChatMemberUpdated::try_from_input(input).await.unwrap().is_some());
}

#[tokio::test]
async fn user_status() {
    let update: Update = serde_json::from_value(serde_json::json!(
        {
            "update_id": 1,
            "chat_member": {
                "chat": {
                    "id": 1,
                    "type": "group",
                    "title": "group title"
                },
                "from": {
                    "id": 1,
                    "is_bot": true,
                    "first_name": "firstname"
                },
                "date": 0,
                "old_chat_member": {
                    "status": "member",
                    "user": {
                        "id": 2,
                        "is_bot": false,
                        "first_name": "firstname"
                    }
                },
                "new_chat_member": {
                    "status": "kicked",
                    "user": {
                        "id": 2,
                        "is_bot": false,
                        "first_name": "firstname",
                    },
                    "until_date": 0
                }
            }
        }
    ))
    .unwrap();
    let input = HandlerInput::from(update);
    assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(ChatMemberUpdated::try_from_input(input).await.unwrap().is_some());
}

#[tokio::test]
async fn chat_join_request() {
    let update: Update = serde_json::from_value(serde_json::json!(
        {
            "update_id": 1,
            "chat_join_request": {
                "chat": {
                    "id": 1,
                    "type": "group",
                    "title": "group title"
                },
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "firstname"
                },
                "date": 0
            }
        }
    ))
    .unwrap();
    let input = HandlerInput::from(update);
    assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(ChatJoinRequest::try_from_input(input).await.unwrap().is_some());
}

#[tokio::test]
async fn tuple() {
    let mut context = Context::default();
    context.insert(3usize);
    let update: Update = serde_json::from_value(serde_json::json!(
        {
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test",
            }
        }
    ))
    .unwrap();
    let input = HandlerInput {
        update,
        context: Arc::new(context),
    };
    assert!(HandlerInput::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(Update::try_from_input(input.clone()).await.unwrap().is_some());
    assert!(<(Ref<usize>, Update, User, Message)>::try_from_input(input.clone())
        .await
        .unwrap()
        .is_some());
}
