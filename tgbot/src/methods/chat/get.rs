use crate::{
    methods::Method,
    request::Request,
    types::{Chat, ChatId},
};
use serde::Serialize;

/// Get up to date information about the chat
#[derive(Clone, Debug, Serialize)]
pub struct GetChat {
    chat_id: ChatId,
}

impl GetChat {
    /// Creates a new GetChat
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        GetChat {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for GetChat {
    type Response = Chat;

    fn into_request(self) -> Request {
        Request::json("getChat", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn get_chat() {
        let request = GetChat::new(1).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/getChat");
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
        } else {
            panic!("Unexpected request body");
        }
    }
}
