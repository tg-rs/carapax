use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, Integer, Message},
};
use failure::Error;
use serde::Serialize;

/// Forward message of any kind
#[derive(Clone, Debug, Serialize)]
pub struct ForwardMessage {
    chat_id: ChatId,
    from_chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    message_id: Integer,
}

impl ForwardMessage {
    /// Creates a new ForwardMessage with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * from_chat_id - Unique identifier for the chat where the original message was sent
    /// * message_id - Message identifier in the chat specified in from_chat_id
    pub fn new<C: Into<ChatId>>(chat_id: C, from_chat_id: C, message_id: Integer) -> Self {
        ForwardMessage {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            disable_notification: None,
        }
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
}

impl Method for ForwardMessage {
    type Response = Message;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("forwardMessage", &self)
    }
}
