use crate::methods::method::*;
use serde::Serialize;

/// Delete a sticker from a set created by the bot
#[derive(Clone, Debug, Serialize)]
pub struct DeleteStickerFromSet {
    sticker: String,
}

impl DeleteStickerFromSet {
    /// Creates a new DeleteStickerFromSet
    ///
    /// # Arguments
    ///
    /// * sticker - File identifier of the sticker
    pub fn new<S: Into<String>>(sticker: S) -> Self {
        DeleteStickerFromSet {
            sticker: sticker.into(),
        }
    }
}

impl Method for DeleteStickerFromSet {
    type Response = bool;

    fn get_request(&self) -> Result<RequestBuilder, RequestError> {
        RequestBuilder::json("deleteStickerFromSet", &self)
    }
}
