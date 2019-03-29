use crate::{
    methods::Method,
    request::{Form, RequestBuilder},
    types::{InputFile, Integer, MaskPosition},
};
use failure::Error;

/// Create new sticker set owned by a user
///
/// The bot will be able to edit the created sticker set
#[derive(Debug)]
pub struct CreateNewStickerSet {
    form: Form,
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
    /// * emojis - One or more emoji corresponding to the sticker
    pub fn new<N, T, E>(user_id: Integer, name: N, title: T, png_sticker: InputFile, emojis: E) -> Self
    where
        N: Into<String>,
        T: Into<String>,
        E: Into<String>,
    {
        let mut form = Form::new();
        form.set_field("user_id", user_id);
        form.set_field("name", name.into());
        form.set_field("title", title.into());
        form.set_field("png_sticker", png_sticker);
        form.set_field("emojis", emojis.into());
        CreateNewStickerSet { form }
    }

    /// Pass True, if a set of mask stickers should be created
    pub fn contains_masks(mut self, value: bool) -> Self {
        self.form.set_field("contains_masks", value);
        self
    }

    /// Position where the mask should be placed on faces
    pub fn mask_position(mut self, value: MaskPosition) -> Result<Self, Error> {
        let value = serde_json::to_string(&value)?;
        self.form.set_field("mask_position", value);
        Ok(self)
    }
}

impl Method for CreateNewStickerSet {
    type Response = bool;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::form("createNewStickerSet", self.form)
    }
}
