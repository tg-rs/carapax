use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, ChatPermissions, Integer},
};
use serde::Serialize;

/// Restrict a user in a supergroup
///
/// The bot must be an administrator in the supergroup
/// for this to work and must have the appropriate admin rights.
///
/// Pass True for all boolean parameters to lift restrictions from a user
#[derive(Clone, Debug, Serialize)]
pub struct RestrictChatMember {
    chat_id: ChatId,
    user_id: Integer,
    permissions: ChatPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<Integer>,
}

impl RestrictChatMember {
    /// Creates a new RestrictChatMember with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
        RestrictChatMember {
            chat_id: chat_id.into(),
            user_id,
            permissions: ChatPermissions::default(),
            until_date: None,
        }
    }

    /// Replace current permissions with the new one
    pub fn with_permissions(mut self, permissions: ChatPermissions) -> Self {
        self.permissions = permissions;
        self
    }

    /// Restrict everything
    pub fn restrict_all(mut self) -> Self {
        self.permissions = ChatPermissions::restricted();
        self
    }

    /// Allow everything
    pub fn allow_all(mut self) -> Self {
        self.permissions = ChatPermissions::allowed();
        self
    }

    /// Date when restrictions will be lifted for the user, unix time
    ///
    /// If user is restricted for more than 366 days or less than 30 seconds
    /// from the current time, they are considered to be restricted forever
    pub fn until_date(mut self, until_date: Integer) -> Self {
        self.until_date = Some(until_date);
        self
    }
}

impl Method for RestrictChatMember {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("restrictChatMember", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn restrict_chat_member_restrict_all() {
        let request = RestrictChatMember::new(1, 2)
            .restrict_all()
            .until_date(100)
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/restrictChatMember"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["user_id"], 2);
            assert_eq!(data["until_date"], 100);
            assert_eq!(
                data["permissions"],
                serde_json::json!({
                    "can_send_messages": false,
                    "can_send_media_messages": false,
                    "can_send_polls": false,
                    "can_send_other_messages": false,
                    "can_add_web_page_previews": false,
                    "can_change_info": false,
                    "can_invite_users": false,
                    "can_pin_messages": false,
                })
            );
        } else {
            panic!("Unexpected request body");
        }
    }

    #[test]
    fn restrict_chat_member_allow_all() {
        let request = RestrictChatMember::new(1, 2).allow_all().until_date(100).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/restrictChatMember"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["user_id"], 2);
            assert_eq!(data["until_date"], 100);
            assert_eq!(
                data["permissions"],
                serde_json::json!({
                    "can_send_messages": true,
                    "can_send_media_messages": true,
                    "can_send_polls": true,
                    "can_send_other_messages": true,
                    "can_add_web_page_previews": true,
                    "can_change_info": true,
                    "can_invite_users": true,
                    "can_pin_messages": true,
                })
            );
        } else {
            panic!("Unexpected request body");
        }
    }

    #[test]
    fn restrict_chat_member_custom() {
        let request = RestrictChatMember::new(1, 2)
            .with_permissions(
                ChatPermissions::default()
                    .with_send_messages(true)
                    .with_send_media_messages(false)
                    .with_send_other_messages(true)
                    .with_add_web_page_previews(false),
            )
            .until_date(100)
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/restrictChatMember"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["user_id"], 2);
            assert_eq!(data["until_date"], 100);
            assert_eq!(
                data["permissions"],
                serde_json::json!({
                    "can_send_messages": true,
                    "can_send_media_messages": false,
                    "can_send_other_messages": true,
                    "can_add_web_page_previews": false
                })
            );
        } else {
            panic!("Unexpected request body");
        }
    }
}
