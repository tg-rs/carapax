use tgbot::types::{
    CallbackQuery, ChosenInlineResult, InlineQuery, Message, Poll, PreCheckoutQuery, ShippingQuery, Update, UpdateKind,
};

/// Allows to create an input for a handler from given update
pub trait FromUpdate: Sized {
    /// Returns a handler input
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
            assert!(Update::from_update(update.clone()).is_some());
            assert!(Message::from_update(update).is_some());
        }
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
        assert!(Update::from_update(update.clone()).is_some());
        assert!(InlineQuery::from_update(update).is_some());
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
        assert!(Update::from_update(update.clone()).is_some());
        assert!(ChosenInlineResult::from_update(update).is_some());
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
        assert!(Update::from_update(update.clone()).is_some());
        assert!(CallbackQuery::from_update(update).is_some());
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
        assert!(Update::from_update(update.clone()).is_some());
        assert!(ShippingQuery::from_update(update).is_some());
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
        assert!(Update::from_update(update.clone()).is_some());
        assert!(PreCheckoutQuery::from_update(update).is_some());
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
                    "is_closed": false
                }
            }
        ))
        .unwrap();
        assert!(Update::from_update(update.clone()).is_some());
        assert!(Poll::from_update(update).is_some());
    }
}
