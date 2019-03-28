use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, Integer},
};
use failure::Error;
use serde::Serialize;

/// Unban a previously kicked user in a supergroup or channel
///
/// The user will not return to the group or channel
/// automatically, but will be able to join via link, etc.
///
/// The bot must be an administrator for this to work
#[derive(Clone, Debug, Serialize)]
pub struct UnbanChatMember {
    chat_id: ChatId,
    user_id: Integer,
}

impl UnbanChatMember {
    /// Creates a new UnbanChatMember
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
        UnbanChatMember {
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl Method for UnbanChatMember {
    type Response = bool;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("unbanChatMember", &self)
    }
}
