use crate::{methods::Method, request::Request, types::ChatId};
use serde::Serialize;

/// Generate a new invite link for a chat
///
/// Any previously generated link is revoked
/// The bot must be an administrator in the chat for this to work and must have the appropriate admin rights
/// Returns the new invite link as String on success
#[derive(Clone, Debug, Serialize)]
pub struct ExportChatInviteLink {
    chat_id: ChatId,
}

impl ExportChatInviteLink {
    /// Creates a new ExportChatInviteLink
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        ExportChatInviteLink {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for ExportChatInviteLink {
    type Response = String;

    fn into_request(self) -> Request {
        Request::json("exportChatInviteLink", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn export_chat_invite_link() {
        let request = ExportChatInviteLink::new(1).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/exportChatInviteLink"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
        } else {
            panic!("Unexpected request body");
        }
    }
}
