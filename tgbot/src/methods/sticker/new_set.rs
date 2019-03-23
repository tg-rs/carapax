use crate::{
    methods::method::*,
    types::{Integer, MaskPosition},
};
use failure::Error;
use serde::Serialize;

/// Create new sticker set owned by a user
///
/// The bot will be able to edit the created sticker set
#[derive(Clone, Debug, Serialize)]
pub struct CreateNewStickerSet {
    user_id: Integer,
    name: String,
    title: String,
    png_sticker: String,
    emojis: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    contains_masks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mask_position: Option<MaskPosition>,
}

impl CreateNewStickerSet {
    /// Creates a new CreateNewStickerSet with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * user_id - User identifier of created sticker set owner
    /// * name - Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g., animals)
    ///          Can contain only english letters, digits and underscores
    ///          Must begin with a letter, can't contain consecutive underscores
    ///          and must end in “_by_<bot username>”
    ///          <bot_username> is case insensitive
    ///          1-64 characters
    /// * title - Sticker set title, 1-64 characters
    /// * png_sticker - Png image with the sticker,
    ///                 must be up to 512 kilobytes in size, dimensions must not exceed 512px,
    ///                 and either width or height must be exactly 512px
    ///                 Pass a file_id as a String to send a file that already exists on the Telegram servers,
    ///                 pass an HTTP URL as a String for Telegram to get a file from the Internet,
    ///                 or upload a new one using multipart/form-data
    /// * emojis - One or more emoji corresponding to the sticker
    pub fn new<S: Into<String>>(user_id: Integer, name: S, title: S, png_sticker: S, emojis: S) -> Self {
        CreateNewStickerSet {
            user_id,
            name: name.into(),
            title: title.into(),
            png_sticker: png_sticker.into(),
            emojis: emojis.into(),
            contains_masks: None,
            mask_position: None,
        }
    }

    /// Pass True, if a set of mask stickers should be created
    pub fn contains_masks(mut self, contains_masks: bool) -> Self {
        self.contains_masks = Some(contains_masks);
        self
    }

    /// Position where the mask should be placed on faces
    pub fn mask_position(mut self, mask_position: MaskPosition) -> Self {
        self.mask_position = Some(mask_position);
        self
    }
}

impl Method for CreateNewStickerSet {
    type Response = bool;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("createNewStickerSet", &self)
    }
}
