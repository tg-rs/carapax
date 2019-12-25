use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Integer},
};
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
    pub fn until_date(mut self, until_date: Integer) -> Self {
        self.until_date = Some(until_date);
        self
    }
}

impl Method for KickChatMember {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("kickChatMember", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn kick_chat_member() {
        let request = KickChatMember::new(1, 2).until_date(3).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/kickChatMember"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["user_id"], 2);
            assert_eq!(data["until_date"], 3);
        } else {
            panic!("Unexpected request body");
        }
    }
}
