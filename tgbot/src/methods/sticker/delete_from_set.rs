use crate::{methods::Method, request::RequestBuilder};
use failure::Error;
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

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("deleteStickerFromSet", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn delete_sticker_from_set() {
        let request = DeleteStickerFromSet::new("sticker")
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/deleteStickerFromSet");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["sticker"], "sticker");
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
