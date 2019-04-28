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

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("getStickerSet", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn get_sticker_set() {
        let request = GetStickerSet::new("name")
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/getStickerSet");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["name"], "name");
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
