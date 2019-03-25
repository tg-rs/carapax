use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, Integer},
};
use failure::Error;
use serde::Serialize;

/// Get the number of members in a chat
#[derive(Clone, Debug, Serialize)]
pub struct GetChatMembersCount {
    chat_id: ChatId,
}

impl GetChatMembersCount {
    /// Creates a new GetChatMembersCount
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        GetChatMembersCount {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for GetChatMembersCount {
    type Response = Integer;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("getChatMembersCount", &self)
    }
}
