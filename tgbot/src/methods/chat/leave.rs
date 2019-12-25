use crate::{methods::Method, request::Request, types::ChatId};
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

    fn into_request(self) -> Request {
        Request::json("leaveChat", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn leave_chat() {
        let request = LeaveChat::new(1).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/leaveChat");
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
        } else {
            panic!("Unexpected request body");
        }
    }
}
