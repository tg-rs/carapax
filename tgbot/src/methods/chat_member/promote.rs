use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, Integer},
};
use failure::Error;
use serde::Serialize;

/// Promote or demote a user in a supergroup or a channel
///
/// The bot must be an administrator in the chat
/// for this to work and must have the appropriate admin rights
/// Pass False for all boolean parameters to demote a user
#[derive(Clone, Debug, Serialize)]
pub struct PromoteChatMember {
    chat_id: ChatId,
    user_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_change_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_post_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_delete_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_invite_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_restrict_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_promote_members: Option<bool>,
}

impl PromoteChatMember {
    /// Creates a new PromoteChatMember with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
        PromoteChatMember {
            chat_id: chat_id.into(),
            user_id,
            can_change_info: None,
            can_post_messages: None,
            can_edit_messages: None,
            can_delete_messages: None,
            can_invite_users: None,
            can_restrict_members: None,
            can_pin_messages: None,
            can_promote_members: None,
        }
    }

    /// Promote all privileges
    pub fn promote_all(mut self) -> Self {
        self.can_change_info = Some(true);
        self.can_post_messages = Some(true);
        self.can_edit_messages = Some(true);
        self.can_delete_messages = Some(true);
        self.can_invite_users = Some(true);
        self.can_restrict_members = Some(true);
        self.can_pin_messages = Some(true);
        self.can_promote_members = Some(true);
        self
    }

    /// Demote all privileges
    pub fn demote_all(mut self) -> Self {
        self.can_change_info = Some(false);
        self.can_post_messages = Some(false);
        self.can_edit_messages = Some(false);
        self.can_delete_messages = Some(false);
        self.can_invite_users = Some(false);
        self.can_restrict_members = Some(false);
        self.can_pin_messages = Some(false);
        self.can_promote_members = Some(false);
        self
    }

    /// Administrator can change chat title, photo and other settings
    pub fn can_change_info(mut self, can_change_info: bool) -> Self {
        self.can_change_info = Some(can_change_info);
        self
    }

    /// Administrator can create channel posts, channels only
    pub fn can_post_messages(mut self, can_post_messages: bool) -> Self {
        self.can_post_messages = Some(can_post_messages);
        self
    }

    /// Administrator can edit messages of other users and can pin messages, channels only
    pub fn can_edit_messages(mut self, can_edit_messages: bool) -> Self {
        self.can_edit_messages = Some(can_edit_messages);
        self
    }

    /// Administrator can delete messages of other users
    pub fn can_delete_messages(mut self, can_delete_messages: bool) -> Self {
        self.can_delete_messages = Some(can_delete_messages);
        self
    }

    /// Administrator can invite new users to the chat
    pub fn can_invite_users(mut self, can_invite_users: bool) -> Self {
        self.can_invite_users = Some(can_invite_users);
        self
    }

    /// Administrator can restrict, ban or unban chat members
    pub fn can_restrict_members(mut self, can_restrict_members: bool) -> Self {
        self.can_restrict_members = Some(can_restrict_members);
        self
    }

    /// Administrator can pin messages, supergroups only
    pub fn can_pin_messages(mut self, can_pin_messages: bool) -> Self {
        self.can_pin_messages = Some(can_pin_messages);
        self
    }

    /// Administrator can add new administrators with a subset of his own privileges or demote administrators
    /// that he has promoted, directly or indirectly (promoted by administrators that were appointed by him)
    pub fn can_promote_members(mut self, can_promote_members: bool) -> Self {
        self.can_promote_members = Some(can_promote_members);
        self
    }
}

impl Method for PromoteChatMember {
    type Response = bool;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("promoteChatMember", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn promote_chat_member() {
        let request = PromoteChatMember::new(1, 2)
            .promote_all()
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/promoteChatMember");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["user_id"], 2);
            assert_eq!(data["can_change_info"], true);
            assert_eq!(data["can_post_messages"], true);
            assert_eq!(data["can_edit_messages"], true);
            assert_eq!(data["can_delete_messages"], true);
            assert_eq!(data["can_invite_users"], true);
            assert_eq!(data["can_restrict_members"], true);
            assert_eq!(data["can_pin_messages"], true);
            assert_eq!(data["can_promote_members"], true);
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }

        let request = PromoteChatMember::new(1, 2)
            .demote_all()
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/promoteChatMember");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["user_id"], 2);
            assert_eq!(data["can_change_info"], false);
            assert_eq!(data["can_post_messages"], false);
            assert_eq!(data["can_edit_messages"], false);
            assert_eq!(data["can_delete_messages"], false);
            assert_eq!(data["can_invite_users"], false);
            assert_eq!(data["can_restrict_members"], false);
            assert_eq!(data["can_pin_messages"], false);
            assert_eq!(data["can_promote_members"], false);
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }

        let request = PromoteChatMember::new(1, 2)
            .can_change_info(true)
            .can_post_messages(false)
            .can_edit_messages(true)
            .can_delete_messages(false)
            .can_invite_users(true)
            .can_restrict_members(false)
            .can_pin_messages(true)
            .can_promote_members(false)
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/promoteChatMember");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["user_id"], 2);
            assert_eq!(data["can_change_info"], true);
            assert_eq!(data["can_post_messages"], false);
            assert_eq!(data["can_edit_messages"], true);
            assert_eq!(data["can_delete_messages"], false);
            assert_eq!(data["can_invite_users"], true);
            assert_eq!(data["can_restrict_members"], false);
            assert_eq!(data["can_pin_messages"], true);
            assert_eq!(data["can_promote_members"], false);
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
