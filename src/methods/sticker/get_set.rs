use crate::methods::method::*;
use crate::types::StickerSet;
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

    fn get_request(&self) -> Result<Request, RequestError> {
        Ok(Request {
            method: RequestMethod::Post,
            url: RequestUrl::new("getStickerSet"),
            body: RequestBody::json(&self)?,
        })
    }
}
