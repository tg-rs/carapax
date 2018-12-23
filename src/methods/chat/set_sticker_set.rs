use crate::methods::method::*;
use crate::types::ChatId;

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

    fn get_request(&self) -> Result<Request, RequestError> {
        Ok(Request {
            method: RequestMethod::Post,
            url: RequestUrl::new("setChatStickerSet"),
            body: RequestBody::json(&self)?,
        })
    }
}
