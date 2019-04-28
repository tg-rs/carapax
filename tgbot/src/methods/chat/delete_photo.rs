use crate::{methods::Method, request::RequestBuilder, types::ChatId};
use failure::Error;
use serde::Serialize;

/// Delete a chat photo
///
/// Photos can't be changed for private chats
/// The bot must be an administrator in the chat for this
/// to work and must have the appropriate admin rights
/// Note: In regular groups (non-supergroups), this method
/// will only work if the ‘All Members Are Admins’
/// setting is off in the target group
#[derive(Clone, Debug, Serialize)]
pub struct DeleteChatPhoto {
    chat_id: ChatId,
}

impl DeleteChatPhoto {
    /// Creates a new DeleteChatPhoto
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        DeleteChatPhoto {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for DeleteChatPhoto {
    type Response = bool;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("deleteChatPhoto", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn delete_chat_photo() {
        let request = DeleteChatPhoto::new(1)
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/deleteChatPhoto");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
