use crate::{methods::Method, request::RequestBuilder, types::StickerSet};
use failure::Error;
use serde::Serialize;

/// Get a sticker set
#[derive(Clone, Debug, Serialize)]
pub struct GetStickerSet {
    name: String,
}

impl GetStickerSet {
    /// Creates a new GetStickerSet
    ///
    /// # Arguments
    ///
    /// * name - Name of the sticker set
    pub fn new<S: Into<String>>(name: S) -> Self {
        GetStickerSet { name: name.into() }
    }
}

impl Method for GetStickerSet {
    type Response = StickerSet;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("getStickerSet", &self)
    }
}
