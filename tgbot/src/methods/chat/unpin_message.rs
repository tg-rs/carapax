use crate::{methods::Method, request::Request, types::ChatId};
use serde::Serialize;

/// Unpin a message in a group, supergroup or a channel
///
/// The bot must be an administrator in the chat
/// for this to work and must have
/// the ‘can_pin_messages’ admin right in the supergroup
/// or ‘can_edit_messages’ admin right in the channel
#[derive(Clone, Debug, Serialize)]
pub struct UnpinChatMessage {
    chat_id: ChatId,
}

impl UnpinChatMessage {
    /// Creates a new UnpinChatMessage
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        UnpinChatMessage {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for UnpinChatMessage {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("unpinChatMessage", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn unpin_chat_message() {
        let request = UnpinChatMessage::new(1).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/unpinChatMessage"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
        } else {
            panic!("Unexpected request body");
        }
    }
}
