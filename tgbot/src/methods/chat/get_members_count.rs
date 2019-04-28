use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, Integer},
};
use failure::Error;
use serde::Serialize;

/// Get the number of members in a chat
#[derive(Clone, Debug, Serialize)]
pub struct GetChatMembersCount {
    chat_id: ChatId,
}

impl GetChatMembersCount {
    /// Creates a new GetChatMembersCount
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        GetChatMembersCount {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for GetChatMembersCount {
    type Response = Integer;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("getChatMembersCount", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn get_chat_members_count() {
        let request = GetChatMembersCount::new(1)
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/getChatMembersCount");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
