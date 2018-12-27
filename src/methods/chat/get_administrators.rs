use crate::methods::method::*;
use crate::types::{ChatId, ChatMember};
use serde::Serialize;

/// Get a list of administrators in a chat
///
/// On success, returns an Array of ChatMember objects that contains
/// information about all chat administrators except other bots
/// If the chat is a group or a supergroup and no administrators
/// were appointed, only the creator will be returned
#[derive(Clone, Debug, Serialize)]
pub struct GetChatAdministrators {
    chat_id: ChatId,
}

impl GetChatAdministrators {
    /// Creates a new GetChatAdministrators
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        GetChatAdministrators {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for GetChatAdministrators {
    type Response = Vec<ChatMember>;

    fn get_request(&self) -> Result<RequestBuilder, RequestError> {
        RequestBuilder::json("getChatAdministrators", &self)
    }
}
