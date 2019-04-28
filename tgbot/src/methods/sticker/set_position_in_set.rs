use crate::{methods::Method, request::RequestBuilder, types::Integer};
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

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("setStickerPositionInSet", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn set_sticker_position_in_set() {
        let request = SetStickerPositionInSet::new("sticker", 1)
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/setStickerPositionInSet");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["sticker"], "sticker");
            assert_eq!(data["position"], 1);
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
