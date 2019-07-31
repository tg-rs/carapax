use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, ChatPermissions},
};
use failure::Error;
use serde::Serialize;

/// Set default chat permissions for all members
///
/// The bot must be an administrator in the group or a supergroup
/// for this to work and must have the can_restrict_members admin rights
///
/// Returns True on success
#[derive(Clone, Debug, Serialize)]
pub struct SetChatPermissions {
    chat_id: ChatId,
    permissions: ChatPermissions,
}

impl SetChatPermissions {
    /// Creates a new SetChatPermissions
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * permissions - New permissions
    pub fn new<C: Into<ChatId>>(chat_id: C, permissions: ChatPermissions) -> Self {
        SetChatPermissions {
            chat_id: chat_id.into(),
            permissions,
        }
    }
}

impl Method for SetChatPermissions {
    type Response = bool;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("setChatPermissions", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn set_chat_permissions() {
        let request = SetChatPermissions::new(1, ChatPermissions::default().with_send_messages(true))
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/setChatPermissions");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["permissions"], serde_json::json!({"can_send_messages": true}));
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
