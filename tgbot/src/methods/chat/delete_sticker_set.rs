use crate::{methods::Method, request::RequestBuilder, types::ChatId};
use failure::Error;
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

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("deleteChatStickerSet", &self)
    }
}
