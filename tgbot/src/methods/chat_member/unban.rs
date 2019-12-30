use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Integer},
};
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

    fn into_request(self) -> Request {
        Request::json("unbanChatMember", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn unban_chat_member() {
        let request = UnbanChatMember::new(1, 2).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/unbanChatMember"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["user_id"], 2);
        } else {
            panic!("Unexpected request body");
        }
    }
}
