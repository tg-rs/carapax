use crate::methods::method::*;
use crate::types::{File, Integer};
use failure::Error;
use serde::Serialize;

/// Upload a .png file with a sticker for later use in createNewStickerSet and addStickerToSet methods
#[derive(Clone, Debug, Serialize)]
pub struct UploadStickerFile {
    user_id: Integer,
    png_sticker: String,
}

impl UploadStickerFile {
    /// Creates a new UploadStickerFile
    ///
    /// # Arguments
    ///
    /// * user_id - User identifier of sticker file owner
    /// * png_sticker - Png image with the sticker, must be up to 512 kilobytes in size,
    ///                 dimensions must not exceed 512px, and either width or height must be exactly 512px
    pub fn new<S: Into<String>>(user_id: Integer, png_sticker: S) -> Self {
        UploadStickerFile {
            user_id,
            png_sticker: png_sticker.into(),
        }
    }
}

impl Method for UploadStickerFile {
    type Response = File;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("uploadStickerFile", &self)
    }
}
