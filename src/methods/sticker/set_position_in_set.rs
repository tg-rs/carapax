use crate::{methods::method::*, types::Integer};
use failure::Error;
use serde::Serialize;

/// Move a sticker in a set created by the bot to a specific position
#[derive(Clone, Debug, Serialize)]
pub struct SetStickerPositionInSet {
    sticker: String,
    position: Integer,
}

impl SetStickerPositionInSet {
    /// Creates a new SetStickerPositionInSet
    ///
    /// # Arguments
    ///
    /// * sticker - File identifier of the sticker
    /// * position - New sticker position in the set, zero-based
    pub fn new<S: Into<String>>(sticker: S, position: Integer) -> Self {
        SetStickerPositionInSet {
            sticker: sticker.into(),
            position,
        }
    }
}

impl Method for SetStickerPositionInSet {
    type Response = bool;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("setStickerPositionInSet", &self)
    }
}
