use crate::{methods::Method, request::Request, types::ChatId};
use serde::Serialize;

/// Set a new group sticker set for a supergroup
///
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights
///
/// Use the field can_set_sticker_set optionally returned in getChat requests
/// to check if the bot can use this method
#[derive(Clone, Debug, Serialize)]
pub struct SetChatStickerSet {
    chat_id: ChatId,
    sticker_set_name: String,
}

impl SetChatStickerSet {
    /// Creates a new SetChatStickerSet
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * sticker_set_name - Name of the sticker set to be set as the group sticker set
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, sticker_set_name: S) -> Self {
        SetChatStickerSet {
            chat_id: chat_id.into(),
            sticker_set_name: sticker_set_name.into(),
        }
    }
}

impl Method for SetChatStickerSet {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("setChatStickerSet", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn set_chat_sticker_set() {
        let request = SetChatStickerSet::new(1, "name").into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/setChatStickerSet"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["sticker_set_name"], "name");
        } else {
            panic!("Unexpected request body");
        }
    }
}
