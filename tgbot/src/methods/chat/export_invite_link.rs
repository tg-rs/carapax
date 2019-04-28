use crate::{methods::Method, request::RequestBuilder, types::ChatId};
use failure::Error;
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

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("exportChatInviteLink", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn export_chat_invite_link() {
        let request = ExportChatInviteLink::new(1)
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/exportChatInviteLink");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
