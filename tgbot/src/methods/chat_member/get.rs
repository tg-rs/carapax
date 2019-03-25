use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, ChatMember, Integer},
};
use failure::Error;
use serde::Serialize;

/// Get information about a member of a chat
#[derive(Clone, Debug, Serialize)]
pub struct GetChatMember {
    chat_id: ChatId,
    user_id: Integer,
}

impl GetChatMember {
    /// Creates a new GetChatMember
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
        GetChatMember {
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl Method for GetChatMember {
    type Response = ChatMember;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("getChatMember", &self)
    }
}
