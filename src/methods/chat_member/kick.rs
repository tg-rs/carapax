use crate::methods::method::*;
use crate::types::{ChatId, Integer};
use serde::Serialize;

/// Kick a user from a group, a supergroup or a channel
///
/// In the case of supergroups and channels, the user will not be able to return
/// to the group on their own using invite links, etc., unless unbanned first
///
/// The bot must be an administrator in the chat
/// for this to work and must have the appropriate admin rights
///
/// Note: In regular groups (non-supergroups), this method
/// will only work if the ‘All Members Are Admins’
/// setting is off in the target group
/// Otherwise members may only be removed
/// by the group's creator or by the member that added them
#[derive(Clone, Debug, Serialize)]
pub struct KickChatMember {
    chat_id: ChatId,
    user_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<Integer>,
}

impl KickChatMember {
    /// Creates a new KickChatMember
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
        KickChatMember {
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
        }
    }

    /// Date when the user will be unbanned, unix time
    ///
    /// If user is banned for more than 366 days or less than 30 seconds
    /// from the current time they are considered to be banned forever
    pub fn until_date(&mut self, until_date: Integer) -> &mut Self {
        self.until_date = Some(until_date);
        self
    }
}

impl Method for KickChatMember {
    type Response = bool;

    fn get_request(&self) -> Result<Request, RequestError> {
        Ok(Request {
            method: RequestMethod::Post,
            url: RequestUrl::new("kickChatMember"),
            body: RequestBody::json(&self)?,
        })
    }
}
