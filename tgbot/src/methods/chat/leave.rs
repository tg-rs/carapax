use crate::{methods::Method, request::RequestBuilder, types::ChatId};
use failure::Error;
use serde::Serialize;

/// Leave a group, supergroup or channel
#[derive(Clone, Debug, Serialize)]
pub struct LeaveChat {
    chat_id: ChatId,
}

impl LeaveChat {
    /// Creates a new LeaveChat
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        LeaveChat {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for LeaveChat {
    type Response = bool;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("leaveChat", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn leave_chat() {
        let request = LeaveChat::new(1).into_request().unwrap().build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/leaveChat");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
