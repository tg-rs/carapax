use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, ChatMember},
};
use failure::Error;
use serde::Serialize;

/// Get a list of administrators in a chat
///
/// On success, returns an Array of ChatMember objects that contains
/// information about all chat administrators except other bots
/// If the chat is a group or a supergroup and no administrators
/// were appointed, only the creator will be returned
#[derive(Clone, Debug, Serialize)]
pub struct GetChatAdministrators {
    chat_id: ChatId,
}

impl GetChatAdministrators {
    /// Creates a new GetChatAdministrators
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        GetChatAdministrators {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for GetChatAdministrators {
    type Response = Vec<ChatMember>;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("getChatAdministrators", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn get_chat_administrators() {
        let request = GetChatAdministrators::new(1)
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/getChatAdministrators");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
