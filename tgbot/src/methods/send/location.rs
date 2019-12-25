use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Float, Integer, Message, ReplyMarkup},
};
use serde::Serialize;

/// Send point on the map
#[derive(Clone, Debug, Serialize)]
pub struct SendLocation {
    chat_id: ChatId,
    latitude: Float,
    longitude: Float,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_period: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl SendLocation {
    /// Creates a new SendLocation with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * latitude - Latitude of the location
    /// * longitude - Longitude of the location
    pub fn new<C: Into<ChatId>>(chat_id: C, latitude: Float, longitude: Float) -> Self {
        SendLocation {
            chat_id: chat_id.into(),
            latitude,
            longitude,
            live_period: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Period in seconds for which the location will be updated
    ///
    /// Should be between 60 and 86400
    pub fn live_period(mut self, live_period: Integer) -> Self {
        self.live_period = Some(live_period);
        self
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(mut self, reply_markup: R) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendLocation {
    type Response = Message;

    fn into_request(self) -> Request {
        Request::json("sendLocation", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::ForceReply,
    };
    use serde_json::Value;

    #[allow(clippy::float_cmp)]
    #[test]
    fn send_location() {
        let request = SendLocation::new(1, 2.0, 3.0)
            .live_period(100)
            .disable_notification(true)
            .reply_to_message_id(1)
            .reply_markup(ForceReply::new(true))
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendLocation");
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["latitude"], 2.0);
            assert_eq!(data["longitude"], 3.0);
            assert_eq!(data["live_period"], 100);
            assert_eq!(data["disable_notification"], true);
            assert_eq!(data["reply_to_message_id"], 1);
            assert_eq!(data["reply_markup"]["force_reply"], true);
        } else {
            panic!("Unexpected request body");
        }
    }
}
