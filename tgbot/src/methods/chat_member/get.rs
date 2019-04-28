use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, ChatMember, Integer},
};
use failure::Error;
use serde::Serialize;

/// Get information about a member of a chat
#[derive(Clone, Debug, Serialize)]
pub struct GetChatMember {
    chat_id: ChatId,
    user_id: Integer,
}

impl GetChatMember {
    /// Creates a new GetChatMember
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
        GetChatMember {
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl Method for GetChatMember {
    type Response = ChatMember;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("getChatMember", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn get_chat_member() {
        let request = GetChatMember::new(1, 2)
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/getChatMember");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["user_id"], 2);
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
