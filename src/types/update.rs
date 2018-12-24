use crate::types::callback_query::CallbackQuery;
use crate::types::inline_mode::{ChosenInlineResult, InlineQuery};
use crate::types::message::Message;
use crate::types::payments::{PreCheckoutQuery, ShippingQuery};
use crate::types::primitive::Integer;
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
    /// Please see our documentation on the feedback collecting
    /// for details on how to enable these updates for your bot
    ChosenInlineResult(ChosenInlineResult),
    /// New incoming callback query
    CallbackQuery(CallbackQuery),
    /// New incoming shipping query
    /// Only for invoices with flexible price
    ShippingQuery(ShippingQuery),
    /// New incoming pre-checkout query. Contains full information about checkout
    PreCheckoutQuery(PreCheckoutQuery),
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
    pub allowed_updates: Vec<AllowedUpdate>,
}

/// Type of update to receive
#[derive(Debug, Deserialize, Eq, Clone, Copy, Hash, PartialEq, PartialOrd, Serialize)]
pub enum AllowedUpdate {
    /// Message
    #[serde(rename = "message")]
    Message,
    /// Edited message
    #[serde(rename = "edited_message")]
    EditedMessage,
    /// Channel post
    #[serde(rename = "channel_post")]
    ChannelPost,
    /// Edited channel post
    #[serde(rename = "edited_channel_post")]
    EditedChannelPost,
    /// Inline query
    #[serde(rename = "inline_query")]
    InlineQuery,
    /// Chosen inline result
    #[serde(rename = "chosen_inline_result")]
    ChosenInlineResult,
    /// Callback query
    #[serde(rename = "callback_query")]
    CallbackQuery,
    /// Shipping query
    #[serde(rename = "shipping_query")]
    ShippingQuery,
    /// Pre checkout query
    #[serde(rename = "pre_checkout_query")]
    PreCheckoutQuery,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let input = r#"{
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
        }"#;
        let update: Update = serde_json::from_str(input).unwrap();
        if let Update {
            id,
            kind: UpdateKind::Message(msg),
        } = update
        {
            assert_eq!(id, 1);
            assert_eq!(msg.id, 1);
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }
}
