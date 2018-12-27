use crate::methods::method::*;
use crate::types::{ChatId, Integer, MediaGroupItem, Message};
use serde::Serialize;

/// Send a group of photos or videos as an album
#[derive(Clone, Debug, Serialize)]
pub struct SendMediaGroup {
    chat_id: ChatId,
    media: Vec<MediaGroupItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
}

impl SendMediaGroup {
    /// Creates a new SendMediaGroup with empty optional parameters
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * media - Photos and videos to be sent, must include 2–10 items
    pub fn new<C: Into<ChatId>>(chat_id: C, media: Vec<MediaGroupItem>) -> Self {
        SendMediaGroup {
            chat_id: chat_id.into(),
            media,
            disable_notification: None,
            reply_to_message_id: None,
        }
    }

    /// Sends the messages silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// If the messages are a reply, ID of the original message
    pub fn reply_to_message_id(&mut self, reply_to_message_id: Integer) -> &mut Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
}

impl Method for SendMediaGroup {
    type Response = Vec<Message>;

    fn get_request(&self) -> Result<RequestBuilder, RequestError> {
        RequestBuilder::json("sendMediaGroup", &self)
    }
}
