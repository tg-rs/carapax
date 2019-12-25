use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Integer, Message},
};
use serde::Serialize;

/// Forward message of any kind
#[derive(Clone, Debug, Serialize)]
pub struct ForwardMessage {
    chat_id: ChatId,
    from_chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    message_id: Integer,
}

impl ForwardMessage {
    /// Creates a new ForwardMessage with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * from_chat_id - Unique identifier for the chat where the original message was sent
    /// * message_id - Message identifier in the chat specified in from_chat_id
    pub fn new<C: Into<ChatId>>(chat_id: C, from_chat_id: C, message_id: Integer) -> Self {
        ForwardMessage {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            disable_notification: None,
        }
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
}

impl Method for ForwardMessage {
    type Response = Message;

    fn into_request(self) -> Request {
        Request::json("forwardMessage", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn forward_message() {
        let request = ForwardMessage::new(1, 2, 3).disable_notification(true).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/forwardMessage"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["from_chat_id"], 2);
            assert_eq!(data["message_id"], 3);
            assert_eq!(data["disable_notification"], true);
        } else {
            panic!("Unexpected request body");
        }
    }
}
