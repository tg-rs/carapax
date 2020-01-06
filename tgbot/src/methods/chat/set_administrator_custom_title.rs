use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Integer},
};
use serde::Serialize;

/// Set a custom title for an administrator in a supergroup promoted by the bot
///
/// Returns True on success
#[derive(Clone, Debug, Serialize)]
pub struct SetChatAdministratorCustomTitle {
    chat_id: ChatId,
    user_id: Integer,
    custom_title: String,
}

impl SetChatAdministratorCustomTitle {
    /// Creates a new SetChatAdministratorCustomTitle
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    /// * custom_title - New custom title for the administrator; 0-16 characters, emoji are not allowed
    pub fn new<C, T>(chat_id: C, user_id: Integer, custom_title: T) -> Self
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        Self {
            chat_id: chat_id.into(),
            user_id,
            custom_title: custom_title.into(),
        }
    }
}

impl Method for SetChatAdministratorCustomTitle {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("setChatAdministratorCustomTitle", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn set_chat_administrator_custom_title() {
        let request = SetChatAdministratorCustomTitle::new(1, 1, "title").into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/setChatAdministratorCustomTitle"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["user_id"], 1);
            assert_eq!(data["custom_title"], "title");
        } else {
            panic!("Unexpected request body");
        }
    }
}
