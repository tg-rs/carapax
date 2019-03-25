use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{Integer, MaskPosition},
};
use failure::Error;
use serde::Serialize;

/// Add a new sticker to a set created by the bot
#[derive(Clone, Debug, Serialize)]
pub struct AddStickerToSet {
    user_id: Integer,
    name: String,
    png_sticker: String,
    emojis: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    mask_position: Option<MaskPosition>,
}

impl AddStickerToSet {
    /// Creates a new AddStickerToSet
    ///
    /// # Arguments
    ///
    /// * user_id - User identifier of sticker set owner
    /// * name - Sticker set name
    /// * png_sticker - Png image with the sticker, must be up to 512 kilobytes in size,
    ///                 dimensions must not exceed 512px,
    ///                 and either width or height must be exactly 512px
    ///                 Pass a file_id as a String to send a file that already exists on the Telegram servers,
    ///                 pass an HTTP URL as a String for Telegram to get a file from the Internet,
    ///                 or upload a new one using multipart/form-data
    /// * emojis - One or more emoji corresponding to the sticker
    pub fn new<S: Into<String>>(user_id: Integer, name: S, png_sticker: S, emojis: S) -> Self {
        AddStickerToSet {
            user_id,
            name: name.into(),
            png_sticker: png_sticker.into(),
            emojis: emojis.into(),
            mask_position: None,
        }
    }

    /// Position where the mask should be placed on faces
    pub fn mask_position(mut self, mask_position: MaskPosition) -> Self {
        self.mask_position = Some(mask_position);
        self
    }
}

impl Method for AddStickerToSet {
    type Response = bool;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("addStickerToSet", &self)
    }
}
