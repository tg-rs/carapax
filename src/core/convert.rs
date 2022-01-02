use crate::{
    core::{context::Ref, handler::HandlerInput},
    types::{
        CallbackQuery, ChatId, ChatJoinRequest, ChatMemberUpdated, ChosenInlineResult, Command, CommandError, Message,
        Poll, PollAnswer, PreCheckoutQuery, ShippingQuery, Text, Update, UpdateKind, User,
    },
};
use futures_util::future::{ok, BoxFuture, Ready};
use std::{convert::Infallible, error::Error, fmt, future::Future};
use tgbot::types::InlineQuery;

/// Allows to create a specific handler input
pub trait TryFromInput: Send + Sized {
    /// A future returned by `try_from_input` method
    type Future: Future<Output = Result<Option<Self>, Self::Error>> + Send;

    /// An error when conversion failed
    type Error: Error + Send;

    /// Performs conversion
    ///
    /// # Arguments
    ///
    /// * input - A value to convert from
    fn try_from_input(input: HandlerInput) -> Self::Future;
}

impl TryFromInput for HandlerInput {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(Some(input))
    }
}

impl TryFromInput for () {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(_input: HandlerInput) -> Self::Future {
        ok(Some(()))
    }
}

impl<T> TryFromInput for Ref<T>
where
    T: Clone + Send + 'static,
{
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.context.get::<T>().cloned().map(Ref::new))
    }
}

impl TryFromInput for Update {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(Some(input.update))
    }
}

impl TryFromInput for ChatId {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.get_chat_id().map(ChatId::Id))
    }
}

impl TryFromInput for User {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.get_user().cloned())
    }
}

impl TryFromInput for Text {
    type Future = BoxFuture<'static, Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        Box::pin(async move {
            match Message::try_from_input(input).await {
                Ok(Some(message)) => Ok(message.get_text().cloned()),
                Ok(None) => Ok(None),
                Err(err) => Err(err),
            }
        })
    }
}

impl TryFromInput for Message {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(match input.update.kind {
            UpdateKind::Message(msg)
            | UpdateKind::EditedMessage(msg)
            | UpdateKind::ChannelPost(msg)
            | UpdateKind::EditedChannelPost(msg) => Some(msg),
            _ => None,
        })
    }
}

impl TryFromInput for Command {
    type Future = BoxFuture<'static, Result<Option<Self>, Self::Error>>;
    type Error = CommandError;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        Box::pin(async move {
            match Message::try_from_input(input).await {
                Ok(Some(message)) => match Command::try_from(message) {
                    Ok(command) => Ok(Some(command)),
                    Err(CommandError::NotFound) => Ok(None),
                    Err(err) => Err(err),
                },
                Ok(None) | Err(_) => Ok(None),
            }
        })
    }
}

impl TryFromInput for InlineQuery {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(match input.update.kind {
            UpdateKind::InlineQuery(query) => Some(query),
            _ => None,
        })
    }
}

impl TryFromInput for ChosenInlineResult {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(match input.update.kind {
            UpdateKind::ChosenInlineResult(result) => Some(result),
            _ => None,
        })
    }
}

impl TryFromInput for CallbackQuery {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(match input.update.kind {
            UpdateKind::CallbackQuery(query) => Some(query),
            _ => None,
        })
    }
}

impl TryFromInput for ShippingQuery {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(match input.update.kind {
            UpdateKind::ShippingQuery(query) => Some(query),
            _ => None,
        })
    }
}

impl TryFromInput for PreCheckoutQuery {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(match input.update.kind {
            UpdateKind::PreCheckoutQuery(query) => Some(query),
            _ => None,
        })
    }
}

impl TryFromInput for Poll {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(match input.update.kind {
            UpdateKind::Poll(poll) => Some(poll),
            _ => None,
        })
    }
}

impl TryFromInput for PollAnswer {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(match input.update.kind {
            UpdateKind::PollAnswer(poll_answer) => Some(poll_answer),
            _ => None,
        })
    }
}

impl TryFromInput for ChatMemberUpdated {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(match input.update.kind {
            UpdateKind::BotStatus(status) | UpdateKind::UserStatus(status) => Some(status),
            _ => None,
        })
    }
}

impl TryFromInput for ChatJoinRequest {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(match input.update.kind {
            UpdateKind::ChatJoinRequest(request) => Some(request),
            _ => None,
        })
    }
}

macro_rules! convert_tuple {
    ($($T:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($T),+> TryFromInput for ($($T,)+)
        where
            $(
                $T: TryFromInput,
                $T::Error: 'static,
            )+
        {
            type Future = BoxFuture<'static, Result<Option<Self>, Self::Error>>;
            type Error = ConvertUpdateError;

            fn try_from_input(input: HandlerInput) -> Self::Future {
                Box::pin(async move {
                    $(
                        let $T = match <$T>::try_from_input(
                            input.clone()
                        ).await.map_err(ConvertUpdateError::new)? {
                            Some(v) => v,
                            None => return Ok(None)
                        };
                    )+
                    Ok(Some(($($T,)+)))
                })
            }
        }
    };
}

convert_tuple!(A);
convert_tuple!(A, B);
convert_tuple!(A, B, C);
convert_tuple!(A, B, C, D);
convert_tuple!(A, B, C, D, E);
convert_tuple!(A, B, C, D, E, F);
convert_tuple!(A, B, C, D, E, F, G);
convert_tuple!(A, B, C, D, E, F, G, H);
convert_tuple!(A, B, C, D, E, F, G, H, I);
convert_tuple!(A, B, C, D, E, F, G, H, I, J);

/// An error when converting an update from a tuple
#[derive(Debug)]
pub struct ConvertUpdateError(Box<dyn Error + Send>);

impl ConvertUpdateError {
    fn new<E: Error + Send + 'static>(err: E) -> Self {
        Self(Box::new(err))
    }
}

impl Error for ConvertUpdateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}

impl fmt::Display for ConvertUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::context::Context;
    use std::sync::Arc;

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
        assert!(matches!(ChatId::try_from_input(input).await, Ok(Some(ChatId::Id(1)))));
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
        for data in vec![
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
                    "chat": {"id": 1, "type": "channel", "title": "channeltitle", "username": "channelusername"},
                    "text": "test message from channel"
                }
            }),
            serde_json::json!({
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
                        "title": "grouptitle"
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
                        "title": "grouptitle"
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
                        "title": "grouptitle"
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
}
