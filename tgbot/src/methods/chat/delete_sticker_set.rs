use crate::{methods::Method, request::Request, types::ChatId};
use serde::Serialize;

/// Delete a group sticker set from a supergroup
///
/// The bot must be an administrator in the chat
/// for this to work and must have the appropriate admin rights
/// Use the field can_set_sticker_set optionally returned
/// in getChat requests to check if the bot can use this method
#[derive(Clone, Debug, Serialize)]
pub struct DeleteChatStickerSet {
    chat_id: ChatId,
}

impl DeleteChatStickerSet {
    /// Creates a new DeleteChatStickerSet
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        DeleteChatStickerSet {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for DeleteChatStickerSet {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("deleteChatStickerSet", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn delete_chat_sticker_set() {
        let request = DeleteChatStickerSet::new(1).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/deleteChatStickerSet"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
        } else {
            panic!("Unexpected request body");
        }
    }
}
