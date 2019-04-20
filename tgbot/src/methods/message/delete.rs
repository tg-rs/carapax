use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, Integer},
};
use failure::Error;
use serde::Serialize;

/// Delete a message, including service messages
///
/// Limitations:
///
/// * A message can only be deleted if it was sent less than 48 hours ago.
/// * Bots can delete outgoing messages in private chats, groups, and supergroups.
/// * Bots can delete incoming messages in private chats.
/// * Bots granted can_post_messages permissions can delete outgoing messages in channels.
/// * If the bot is an administrator of a group, it can delete any message there.
/// * If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.
#[derive(Clone, Debug, Serialize)]
pub struct DeleteMessage {
    chat_id: ChatId,
    message_id: Integer,
}

impl DeleteMessage {
    /// Creates a new DeleteMessage
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the message to delete
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer) -> Self {
        DeleteMessage {
            chat_id: chat_id.into(),
            message_id,
        }
    }
}

impl Method for DeleteMessage {
    type Response = bool;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("deleteMessage", &self)
    }
}
