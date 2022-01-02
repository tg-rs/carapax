use crate::core::{handler::Handler, result::HandlerResult};
use async_trait::async_trait;
use std::{
    convert::{Infallible, TryFrom},
    error::Error,
};
use tgbot::types::{
    CallbackQuery, ChosenInlineResult, Command, CommandError, InlineQuery, Message, Poll, PollAnswer, PreCheckoutQuery,
    ShippingQuery, Update, UpdateKind,
};

pub(super) struct ConvertHandler<H>(H);

impl<H> ConvertHandler<H> {
    pub(super) fn boxed(handler: H) -> Box<Self> {
        Box::new(Self(handler))
    }
}

#[async_trait]
impl<C, H, I> Handler<C> for ConvertHandler<H>
where
    C: Send + Sync,
    H: Handler<C, Input = I> + Send,
    I: TryFromUpdate + Send + Sync + 'static,
{
    type Input = Update;
    type Output = HandlerResult;

    async fn handle(&mut self, context: &C, input: Self::Input) -> Self::Output {
        match TryFromUpdate::try_from_update(input) {
            Ok(Some(input)) => self.0.handle(context, input).await.into(),
            Ok(None) => HandlerResult::Continue,
            Err(err) => HandlerResult::error(err),
        }
    }
}

/// Allows to create an input for a handler from given update
pub trait TryFromUpdate: Sized {
    /// An error when converting update
    type Error: Error + Send + Sync;

    /// Returns a handler input
    ///
    /// Handler will not run if None or Error returned
    fn try_from_update(update: Update) -> Result<Option<Self>, Self::Error>;
}

impl TryFromUpdate for Update {
    type Error = Infallible;

    fn try_from_update(update: Update) -> Result<Option<Self>, Self::Error> {
        Ok(Some(update))
    }
}

impl TryFromUpdate for Message {
    type Error = Infallible;

    fn try_from_update(update: Update) -> Result<Option<Self>, Self::Error> {
        Ok(match update.kind {
            UpdateKind::Message(msg)
            | UpdateKind::EditedMessage(msg)
            | UpdateKind::ChannelPost(msg)
            | UpdateKind::EditedChannelPost(msg) => Some(msg),
            _ => None,
        })
    }
}

impl TryFromUpdate for Command {
    type Error = CommandError;

    fn try_from_update(update: Update) -> Result<Option<Self>, Self::Error> {
        // Should never panic as the error type is Infallible
        let message: Option<Message> =
            TryFromUpdate::try_from_update(update).expect("Could not convert update to message");
        if let Some(message) = message {
            match Command::try_from(message) {
                Ok(command) => Ok(Some(command)),
                Err(CommandError::NotFound) => Ok(None),
                Err(err) => Err(err),
            }
        } else {
            Ok(None)
        }
    }
}

impl TryFromUpdate for InlineQuery {
    type Error = Infallible;

    fn try_from_update(update: Update) -> Result<Option<Self>, Self::Error> {
        Ok(match update.kind {
            UpdateKind::InlineQuery(query) => Some(query),
            _ => None,
        })
    }
}

impl TryFromUpdate for ChosenInlineResult {
    type Error = Infallible;

    fn try_from_update(update: Update) -> Result<Option<Self>, Self::Error> {
        Ok(match update.kind {
            UpdateKind::ChosenInlineResult(result) => Some(result),
            _ => None,
        })
    }
}

impl TryFromUpdate for CallbackQuery {
    type Error = Infallible;

    fn try_from_update(update: Update) -> Result<Option<Self>, Self::Error> {
        Ok(match update.kind {
            UpdateKind::CallbackQuery(query) => Some(query),
            _ => None,
        })
    }
}

impl TryFromUpdate for ShippingQuery {
    type Error = Infallible;

    fn try_from_update(update: Update) -> Result<Option<Self>, Self::Error> {
        Ok(match update.kind {
            UpdateKind::ShippingQuery(query) => Some(query),
            _ => None,
        })
    }
}

impl TryFromUpdate for PreCheckoutQuery {
    type Error = Infallible;

    fn try_from_update(update: Update) -> Result<Option<Self>, Self::Error> {
        Ok(match update.kind {
            UpdateKind::PreCheckoutQuery(query) => Some(query),
            _ => None,
        })
    }
}

impl TryFromUpdate for Poll {
    type Error = Infallible;

    fn try_from_update(update: Update) -> Result<Option<Self>, Self::Error> {
        Ok(match update.kind {
            UpdateKind::Poll(poll) => Some(poll),
            _ => None,
        })
    }
}

impl TryFromUpdate for PollAnswer {
    type Error = Infallible;

    fn try_from_update(update: Update) -> Result<Option<Self>, Self::Error> {
        Ok(match update.kind {
            UpdateKind::PollAnswer(poll_answer) => Some(poll_answer),
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message() {
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
            assert!(Update::try_from_update(update.clone()).unwrap().is_some());
            assert!(Message::try_from_update(update).unwrap().is_some());
        }
    }

    #[test]
    fn command() {
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
        assert!(Update::try_from_update(update.clone()).unwrap().is_some());
        assert!(Command::try_from_update(update).unwrap().is_some());
    }

    #[test]
    fn inline_query() {
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
        assert!(Update::try_from_update(update.clone()).unwrap().is_some());
        assert!(InlineQuery::try_from_update(update).unwrap().is_some());
    }

    #[test]
    fn chosen_inline_result() {
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
        assert!(Update::try_from_update(update.clone()).unwrap().is_some());
        assert!(ChosenInlineResult::try_from_update(update).unwrap().is_some());
    }

    #[test]
    fn callback_query() {
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
        assert!(Update::try_from_update(update.clone()).unwrap().is_some());
        assert!(CallbackQuery::try_from_update(update).unwrap().is_some());
    }

    #[test]
    fn shipping_query() {
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
        assert!(Update::try_from_update(update.clone()).unwrap().is_some());
        assert!(ShippingQuery::try_from_update(update).unwrap().is_some());
    }

    #[test]
    fn pre_checkout_query() {
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
        assert!(Update::try_from_update(update.clone()).unwrap().is_some());
        assert!(PreCheckoutQuery::try_from_update(update).unwrap().is_some());
    }

    #[test]
    fn poll() {
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
        assert!(Update::try_from_update(update.clone()).unwrap().is_some());
        assert!(Poll::try_from_update(update).unwrap().is_some());
    }

    #[test]
    fn poll_answer() {
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
        assert!(Update::try_from_update(update.clone()).unwrap().is_some());
        assert!(PollAnswer::try_from_update(update).unwrap().is_some());
    }
}
