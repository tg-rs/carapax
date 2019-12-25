use crate::{methods::Method, request::Request};
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

    fn into_request(self) -> Request {
        Request::json("deleteStickerFromSet", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn delete_sticker_from_set() {
        let request = DeleteStickerFromSet::new("sticker").into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/deleteStickerFromSet"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["sticker"], "sticker");
        } else {
            panic!("Unexpected request body");
        }
    }
}
