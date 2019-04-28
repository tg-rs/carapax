use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, EditMessageResult, Float, InlineKeyboardMarkup, Integer},
};
use failure::Error;
use serde::Serialize;

/// Edit live location messages sent by the bot or via the bot (for inline bots)
///
/// A location can be edited until its live_period expires or editing
/// is explicitly disabled by a call to stopMessageLiveLocation
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageLiveLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    latitude: Float,
    longitude: Float,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageLiveLocation {
    /// Creates a new EditMessageLiveLocation
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    /// * latitude - Latitude of new location
    /// * longitude Longitude of new location
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer, latitude: Float, longitude: Float) -> Self {
        EditMessageLiveLocation {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            latitude,
            longitude,
            reply_markup: None,
        }
    }

    /// Creates a new EditMessageLiveLocation
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    /// * latitude - Latitude of new location
    /// * longitude Longitude of new location
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S, latitude: Float, longitude: Float) -> Self {
        EditMessageLiveLocation {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            latitude,
            longitude,
            reply_markup: None,
        }
    }

    /// New inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for EditMessageLiveLocation {
    type Response = EditMessageResult;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("editMessageLiveLocation", &self)
    }
}

/// Stop updating a live location message
/// sent by the bot or via the bot (for inline bots)
/// before live_period expires
#[derive(Clone, Debug, Serialize)]
pub struct StopMessageLiveLocation {
    chat_id: Option<ChatId>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl StopMessageLiveLocation {
    /// Creates a new StopMessageLiveLocation
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer) -> Self {
        StopMessageLiveLocation {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            reply_markup: None,
        }
    }

    /// Creates a new StopMessageLiveLocation
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S) -> Self {
        StopMessageLiveLocation {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            reply_markup: None,
        }
    }

    /// New inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for StopMessageLiveLocation {
    type Response = EditMessageResult;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("stopMessageLiveLocation", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::InlineKeyboardButton,
    };
    use serde_json::Value;

    #[allow(clippy::float_cmp)]
    #[test]
    fn edit_message_live_location() {
        let request = EditMessageLiveLocation::new(1, 2, 3.0, 4.0)
            .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/editMessageLiveLocation");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["message_id"], 2);
            assert_eq!(data["latitude"], 3.0);
            assert_eq!(data["longitude"], 4.0);
            assert_eq!(data["reply_markup"]["inline_keyboard"][0][0]["text"], "text");
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }

        let request = EditMessageLiveLocation::with_inline_message_id("msg-id", 3.0, 4.0)
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/editMessageLiveLocation");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["inline_message_id"], "msg-id");
            assert_eq!(data["latitude"], 3.0);
            assert_eq!(data["longitude"], 4.0);
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }

    #[test]
    fn stop_message_live_location() {
        let request = StopMessageLiveLocation::new(1, 2)
            .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/stopMessageLiveLocation");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["message_id"], 2);
            assert_eq!(data["reply_markup"]["inline_keyboard"][0][0]["text"], "text");
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }

        let request = StopMessageLiveLocation::with_inline_message_id("msg-id")
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/stopMessageLiveLocation");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["inline_message_id"], "msg-id");
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
