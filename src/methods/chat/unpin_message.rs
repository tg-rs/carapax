use crate::methods::method::*;
use crate::types::ChatId;

/// Unpin a message in a supergroup or a channel
///
/// The bot must be an administrator in the chat
/// for this to work and must have
/// the ‘can_pin_messages’ admin right in the supergroup
/// or ‘can_edit_messages’ admin right in the channel
#[derive(Clone, Debug, Serialize)]
pub struct UnpinChatMessage {
    chat_id: ChatId,
}

impl UnpinChatMessage {
    /// Creates a new UnpinChatMessage
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        UnpinChatMessage {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for UnpinChatMessage {
    type Response = bool;

    fn get_request(&self) -> Result<Request, RequestError> {
        Ok(Request {
            method: RequestMethod::Post,
            url: RequestUrl::new("unpinChatMessage"),
            body: RequestBody::json(&self)?,
        })
    }
}
