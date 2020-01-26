use crate::types::{
    callback_query::CallbackQuery,
    inline_mode::{ChosenInlineResult, InlineQuery},
    message::Message,
    payments::{PreCheckoutQuery, ShippingQuery},
    poll::{Poll, PollAnswer},
    primitive::Integer,
    user::User,
};
use serde::{de::Error, Deserialize, Deserializer, Serialize};

/// Incoming update
#[derive(Clone, Debug)]
pub struct Update {
    /// The update‘s unique identifier
    ///
    /// Update identifiers start from a certain positive number and increase sequentially
    /// This ID becomes especially handy if you’re using Webhooks, since it allows you to
    /// ignore repeated updates or to restore the correct update sequence, should they get out of order
    /// If there are no new updates for at least a week, then identifier
    /// of the next update will be chosen randomly instead of sequentially
    pub id: Integer,
    /// Kind of update
    pub kind: UpdateKind,
}

impl Update {
    /// Returns a chat ID from update
    pub fn get_chat_id(&self) -> Option<Integer> {
        self.get_message().map(|msg| msg.get_chat_id())
    }

    /// Returns a chat username from update
    pub fn get_chat_username(&self) -> Option<&str> {
        self.get_message().and_then(|msg| msg.get_chat_username())
    }

    /// Returns a user ID from update
    pub fn get_user(&self) -> Option<&User> {
        Some(match self.kind {
            UpdateKind::Message(ref msg)
            | UpdateKind::EditedMessage(ref msg)
            | UpdateKind::ChannelPost(ref msg)
            | UpdateKind::EditedChannelPost(ref msg) => return msg.get_user(),
            UpdateKind::InlineQuery(ref query) => &query.from,
            UpdateKind::ChosenInlineResult(ref result) => &result.from,
            UpdateKind::CallbackQuery(ref query) => &query.from,
            UpdateKind::ShippingQuery(ref query) => &query.from,
            UpdateKind::PreCheckoutQuery(ref query) => &query.from,
            UpdateKind::Poll(_) => return None,
            UpdateKind::PollAnswer(ref answer) => &answer.user,
        })
    }

    /// Returns a message from update
    pub fn get_message(&self) -> Option<&Message> {
        match self.kind {
            UpdateKind::Message(ref msg)
            | UpdateKind::EditedMessage(ref msg)
            | UpdateKind::ChannelPost(ref msg)
            | UpdateKind::EditedChannelPost(ref msg) => Some(msg),
            _ => None,
        }
    }
}

/// Kind of update
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum UpdateKind {
    /// New incoming message of any kind — text, photo, sticker, etc
    Message(Message),
    /// New version of a message that is known to the bot and was edited
    EditedMessage(Message),
    /// New incoming channel post of any kind — text, photo, sticker, etc
    ChannelPost(Message),
    /// New version of a channel post that is known to the bot and was edited
    EditedChannelPost(Message),
    /// New incoming inline query
    InlineQuery(InlineQuery),
    /// The result of an inline query that was chosen by a user and sent to their chat partner
    ///
    /// Please see our documentation on the feedback collecting
    /// for details on how to enable these updates for your bot
    ChosenInlineResult(ChosenInlineResult),
    /// New incoming callback query
    CallbackQuery(CallbackQuery),
    /// New incoming shipping query
    ///
    /// Only for invoices with flexible price
    ShippingQuery(ShippingQuery),
    /// New incoming pre-checkout query
    ///
    /// Contains full information about checkout
    PreCheckoutQuery(PreCheckoutQuery),
    /// New poll state
    ///
    /// Bots receive only updates about polls, which are sent or stopped by the bot
    Poll(Poll),
    /// A user changed their answer in a non-anonymous poll
    ///
    /// Bots receive new votes only in polls that were sent by the bot itself
    PollAnswer(PollAnswer),
}

impl<'de> Deserialize<'de> for Update {
    fn deserialize<D>(deserializer: D) -> Result<Update, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawUpdate = Deserialize::deserialize(deserializer)?;
        Ok(Update {
            id: raw.update_id,
            kind: if let Some(data) = raw.message {
                UpdateKind::Message(data)
            } else if let Some(data) = raw.edited_message {
                UpdateKind::EditedMessage(data)
            } else if let Some(data) = raw.channel_post {
                UpdateKind::ChannelPost(data)
            } else if let Some(data) = raw.edited_channel_post {
                UpdateKind::EditedChannelPost(data)
            } else if let Some(data) = raw.inline_query {
                UpdateKind::InlineQuery(data)
            } else if let Some(data) = raw.chosen_inline_result {
                UpdateKind::ChosenInlineResult(data)
            } else if let Some(data) = raw.callback_query {
                UpdateKind::CallbackQuery(data)
            } else if let Some(data) = raw.shipping_query {
                UpdateKind::ShippingQuery(data)
            } else if let Some(data) = raw.pre_checkout_query {
                UpdateKind::PreCheckoutQuery(data)
            } else if let Some(data) = raw.poll {
                UpdateKind::Poll(data)
            } else if let Some(data) = raw.poll_answer {
                UpdateKind::PollAnswer(data)
            } else {
                return Err(D::Error::custom("Can not detect update kind"));
            },
        })
    }
}

/// Information about the current status of a webhook
#[derive(Clone, Debug, Deserialize)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub url: String,
    /// True, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    pub pending_update_count: Integer,
    ///  Unix time for the most recent error that happened when trying to deliver an update via webhook
    pub last_error_date: Option<Integer>,
    /// Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    pub last_error_message: Option<String>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    pub max_connections: Option<Integer>,
    /// A list of update types the bot is subscribed to
    /// Defaults to all update types
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

/// Type of update to receive
#[derive(Debug, Deserialize, Eq, Clone, Copy, Hash, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AllowedUpdate {
    /// Message
    Message,
    /// Edited message
    EditedMessage,
    /// Channel post
    ChannelPost,
    /// Edited channel post
    EditedChannelPost,
    /// Inline query
    InlineQuery,
    /// Chosen inline result
    ChosenInlineResult,
    /// Callback query
    CallbackQuery,
    /// Shipping query
    ShippingQuery,
    /// Pre checkout query
    PreCheckoutQuery,
    /// Poll
    Poll,
    /// Poll answer
    PollAnswer,
}

#[derive(Debug, Deserialize)]
struct RawUpdate {
    update_id: Integer,
    message: Option<Message>,
    edited_message: Option<Message>,
    channel_post: Option<Message>,
    edited_channel_post: Option<Message>,
    inline_query: Option<InlineQuery>,
    chosen_inline_result: Option<ChosenInlineResult>,
    callback_query: Option<CallbackQuery>,
    shipping_query: Option<ShippingQuery>,
    pre_checkout_query: Option<PreCheckoutQuery>,
    poll: Option<Poll>,
    poll_answer: Option<PollAnswer>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_update_message() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 0,
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test"
                },
                "chat": {
                    "id": 1,
                    "type": "private",
                    "first_name": "test"
                },
                "text": "test"
            }
        }))
        .unwrap();
        assert_eq!(update.get_chat_id().unwrap(), 1);
        assert!(update.get_chat_username().is_none());
        assert_eq!(update.get_user().map(|u| u.id).unwrap(), 1);
        if let Update {
            id,
            kind: UpdateKind::Message(data),
        } = update
        {
            assert_eq!(id, 1);
            assert_eq!(data.id, 1);
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }

    #[test]
    fn deserialize_update_edited_message() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "edited_message": {
                "date": 1441,
                "chat": {
                    "id": 1111,
                    "first_name": "Test Firstname",
                    "last_name": "Test Lastname",
                    "username": "Testusername",
                    "type": "private",
                },
                "message_id": 1365,
                "from": {
                    "id": 1111,
                    "first_name": "Test Firstname",
                    "last_name": "Test Lastname",
                    "username": "Testusername",
                    "is_bot": false
                },
                "text": "Edited text",
                "edit_date": 1441
            }
        }))
        .unwrap();
        assert_eq!(update.get_chat_id().unwrap(), 1111);
        assert_eq!(update.get_chat_username().unwrap(), "Testusername");
        assert_eq!(update.get_user().map(|u| u.id).unwrap(), 1111);
        if let Update {
            id,
            kind: UpdateKind::EditedMessage(data),
        } = update
        {
            assert_eq!(id, 1);
            assert_eq!(data.id, 1365);
            assert!(data.is_edited());
            assert_eq!(data.edit_date.unwrap(), 1441);
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }

    #[test]
    fn deserialize_update_channel_post() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "channel_post": {
                "message_id": 1111,
                "date": 0,
                "author_signature": "test",
                "chat": {
                    "id": 1,
                    "type": "channel",
                    "title": "channeltitle",
                    "username": "channelusername"
                },
                "text": "test message from channel"
            }
        }))
        .unwrap();
        assert_eq!(update.get_chat_id().unwrap(), 1);
        assert_eq!(update.get_chat_username().unwrap(), "channelusername");
        assert!(update.get_user().is_none());
        if let Update {
            id,
            kind: UpdateKind::ChannelPost(data),
        } = update
        {
            assert_eq!(id, 1);
            assert_eq!(data.id, 1111);
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }

    #[test]
    fn deserialize_update_edited_channel_post() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "edited_channel_post": {
                "message_id": 1111,
                "date": 0,
                "author_signature": "test",
                "chat": {
                    "id": 1,
                    "type": "channel",
                    "title": "channeltitle",
                    "username": "channelusername"
                },
                "text": "test message from channel"
            }
        }))
        .unwrap();
        assert_eq!(update.get_chat_id().unwrap(), 1);
        assert_eq!(update.get_chat_username().unwrap(), "channelusername");
        assert!(update.get_user().is_none());
        if let Update {
            id,
            kind: UpdateKind::EditedChannelPost(data),
        } = update
        {
            assert_eq!(id, 1);
            assert_eq!(data.id, 1111);
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }

    #[test]
    fn deserialize_update_inline_query() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "inline_query": {
                "id": "query id",
                "from": {
                    "id": 1111,
                    "first_name": "Test Firstname",
                    "is_bot": false
                },
                "query": "query text",
                "offset": "query offset"
            }
        }))
        .unwrap();
        assert!(update.get_chat_id().is_none());
        assert!(update.get_chat_username().is_none());
        assert_eq!(update.get_user().map(|u| u.id).unwrap(), 1111);
        if let Update {
            id,
            kind: UpdateKind::InlineQuery(data),
        } = update
        {
            assert_eq!(id, 1);
            assert_eq!(data.id, "query id");
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }

    #[test]
    fn deserialize_update_chosen_inline_result() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "chosen_inline_result": {
                "result_id": "result id",
                "from": {
                    "id": 1111,
                    "first_name": "Test Firstname",
                    "is_bot": false
                },
                "query": "q",
            }
        }))
        .unwrap();
        assert!(update.get_chat_id().is_none());
        assert!(update.get_chat_username().is_none());
        assert_eq!(update.get_user().map(|u| u.id).unwrap(), 1111);
        if let Update {
            id,
            kind: UpdateKind::ChosenInlineResult(data),
        } = update
        {
            assert_eq!(id, 1);
            assert_eq!(data.result_id, "result id");
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }

    #[test]
    fn deserialize_update_callback_query() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "callback_query": {
                "id": "test",
                "from": {
                    "id": 1,
                    "first_name": "test",
                    "is_bot": false
                }
            }
        }))
        .unwrap();
        assert!(update.get_chat_id().is_none());
        assert!(update.get_chat_username().is_none());
        assert_eq!(update.get_user().map(|u| u.id).unwrap(), 1);
        if let Update {
            id,
            kind: UpdateKind::CallbackQuery(data),
        } = update
        {
            assert_eq!(id, 1);
            assert_eq!(data.id, "test");
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }

    #[test]
    fn deserialize_update_shipping_query() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "shipping_query": {
                "id": "query-id",
                "from": {
                    "id": 1,
                    "first_name": "test",
                    "is_bot": false
                },
                "invoice_payload": "payload",
                "shipping_address": {
                    "country_code": "RU",
                    "state": "Chechen Republic",
                    "city": "Gudermes",
                    "street_line1": "Nuradilov st., 12",
                    "street_line2": "",
                    "post_code": "366200",
                }
            }
        }))
        .unwrap();
        assert!(update.get_chat_id().is_none());
        assert!(update.get_chat_username().is_none());
        assert_eq!(update.get_user().map(|u| u.id).unwrap(), 1);
        if let Update {
            id,
            kind: UpdateKind::ShippingQuery(data),
        } = update
        {
            assert_eq!(id, 1);
            assert_eq!(data.id, "query-id");
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }

    #[test]
    fn deserialize_update_pre_checkout_query() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "pre_checkout_query": {
                "id": "query id",
                "from": {
                    "id": 1,
                    "first_name": "test",
                    "is_bot": false
                },
                "currency": "GEL",
                "total_amount": 100,
                "invoice_payload": "invoice payload"
            }
        }))
        .unwrap();
        assert!(update.get_chat_id().is_none());
        assert!(update.get_chat_username().is_none());
        assert_eq!(update.get_user().map(|u| u.id).unwrap(), 1);
        if let Update {
            id,
            kind: UpdateKind::PreCheckoutQuery(data),
        } = update
        {
            assert_eq!(id, 1);
            assert_eq!(data.id, "query id");
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }

    #[test]
    fn deserialize_update_poll() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "poll": {
                "id": "poll-id",
                "question": "Rust?",
                "options": [
                    {"text": "Yes", "voter_count": 1000},
                    {"text": "No", "voter_count": 0}
                ],
                "is_closed": true,
                "total_voter_count": 100,
                "is_anonymous": true,
                "type": "regular",
                "allows_multiple_answers": false
            }
        }))
        .unwrap();
        assert!(update.get_chat_id().is_none());
        assert!(update.get_chat_username().is_none());
        assert!(update.get_user().is_none());
        if let Update {
            id,
            kind: UpdateKind::Poll(data),
        } = update
        {
            assert_eq!(id, 1);
            assert_eq!(data.id, "poll-id");
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }

    #[test]
    fn deserialize_update_poll_answer() {
        let update: Update = serde_json::from_value(serde_json::json!({
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
        }))
        .unwrap();
        assert!(update.get_chat_id().is_none());
        assert!(update.get_chat_username().is_none());
        assert!(update.get_user().is_some());
        if let Update {
            id,
            kind: UpdateKind::PollAnswer(data),
        } = update
        {
            assert_eq!(id, 1);
            assert_eq!(data.poll_id, "poll-id");
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }

    #[test]
    fn allowed_update() {
        assert_eq!(serde_json::to_string(&AllowedUpdate::Message).unwrap(), r#""message""#);
        assert_eq!(
            serde_json::to_string(&AllowedUpdate::EditedMessage).unwrap(),
            r#""edited_message""#
        );
        assert_eq!(
            serde_json::to_string(&AllowedUpdate::ChannelPost).unwrap(),
            r#""channel_post""#
        );
        assert_eq!(
            serde_json::to_string(&AllowedUpdate::EditedChannelPost).unwrap(),
            r#""edited_channel_post""#
        );
        assert_eq!(
            serde_json::to_string(&AllowedUpdate::InlineQuery).unwrap(),
            r#""inline_query""#
        );
        assert_eq!(
            serde_json::to_string(&AllowedUpdate::ChosenInlineResult).unwrap(),
            r#""chosen_inline_result""#
        );
        assert_eq!(
            serde_json::to_string(&AllowedUpdate::CallbackQuery).unwrap(),
            r#""callback_query""#
        );
        assert_eq!(
            serde_json::to_string(&AllowedUpdate::ShippingQuery).unwrap(),
            r#""shipping_query""#
        );
        assert_eq!(
            serde_json::to_string(&AllowedUpdate::PreCheckoutQuery).unwrap(),
            r#""pre_checkout_query""#
        );
        assert_eq!(serde_json::to_string(&AllowedUpdate::Poll).unwrap(), r#""poll""#);
        assert_eq!(
            serde_json::to_string(&AllowedUpdate::PollAnswer).unwrap(),
            r#""poll_answer""#
        );

        assert_eq!(
            AllowedUpdate::Message,
            serde_json::from_str::<AllowedUpdate>(r#""message""#).unwrap()
        );
        assert_eq!(
            AllowedUpdate::EditedMessage,
            serde_json::from_str::<AllowedUpdate>(r#""edited_message""#).unwrap()
        );
        assert_eq!(
            AllowedUpdate::ChannelPost,
            serde_json::from_str::<AllowedUpdate>(r#""channel_post""#).unwrap()
        );
        assert_eq!(
            AllowedUpdate::EditedChannelPost,
            serde_json::from_str::<AllowedUpdate>(r#""edited_channel_post""#).unwrap()
        );
        assert_eq!(
            AllowedUpdate::InlineQuery,
            serde_json::from_str::<AllowedUpdate>(r#""inline_query""#).unwrap()
        );
        assert_eq!(
            AllowedUpdate::ChosenInlineResult,
            serde_json::from_str::<AllowedUpdate>(r#""chosen_inline_result""#).unwrap()
        );
        assert_eq!(
            AllowedUpdate::CallbackQuery,
            serde_json::from_str::<AllowedUpdate>(r#""callback_query""#).unwrap()
        );
        assert_eq!(
            AllowedUpdate::ShippingQuery,
            serde_json::from_str::<AllowedUpdate>(r#""shipping_query""#).unwrap()
        );
        assert_eq!(
            AllowedUpdate::PreCheckoutQuery,
            serde_json::from_str::<AllowedUpdate>(r#""pre_checkout_query""#).unwrap()
        );
        assert_eq!(
            AllowedUpdate::Poll,
            serde_json::from_str::<AllowedUpdate>(r#""poll""#).unwrap()
        );
        assert_eq!(
            AllowedUpdate::PollAnswer,
            serde_json::from_str::<AllowedUpdate>(r#""poll_answer""#).unwrap()
        );
    }

    #[test]
    fn deserialize_webhook_info_full() {
        let data: WebhookInfo = serde_json::from_value(serde_json::json!({
            "url": "https://example.com/tg-webhook",
            "has_custom_certificate": true,
            "pending_update_count": 1,
            "last_error_date": 0,
            "last_error_message": "error",
            "max_connections": 10,
            "allowed_updates": ["message", "poll"]
        }))
        .unwrap();
        assert_eq!(data.url, "https://example.com/tg-webhook");
        assert!(data.has_custom_certificate);
        assert_eq!(data.pending_update_count, 1);
        assert_eq!(data.last_error_date.unwrap(), 0);
        assert_eq!(data.last_error_message.unwrap(), "error");
        assert_eq!(data.max_connections.unwrap(), 10);
        let allowed = data.allowed_updates.unwrap();
        assert_eq!(allowed.len(), 2);
        assert_eq!(&allowed[0], &AllowedUpdate::Message);
        assert_eq!(&allowed[1], &AllowedUpdate::Poll);
    }

    #[test]
    fn deserialize_webhook_info_partial() {
        let data: WebhookInfo = serde_json::from_value(serde_json::json!({
            "url": "https://example.com/tg-webhook",
            "has_custom_certificate": true,
            "pending_update_count": 1
        }))
        .unwrap();
        assert_eq!(data.url, "https://example.com/tg-webhook");
        assert!(data.has_custom_certificate);
        assert_eq!(data.pending_update_count, 1);
        assert!(data.last_error_date.is_none());
        assert!(data.last_error_message.is_none());
        assert!(data.max_connections.is_none());
        assert!(data.allowed_updates.is_none());
    }
}
