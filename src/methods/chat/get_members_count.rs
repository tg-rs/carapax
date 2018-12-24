use crate::methods::method::*;
use crate::types::{ChatId, Integer};
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

    fn get_request(&self) -> Result<Request, RequestError> {
        Ok(Request {
            method: RequestMethod::Post,
            url: RequestUrl::new("getChatMembersCount"),
            body: RequestBody::json(&self)?,
        })
    }
}
