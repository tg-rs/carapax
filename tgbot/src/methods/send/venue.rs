use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, Float, Integer, Message, ReplyMarkup},
};
use failure::Error;
use serde::Serialize;

/// Send information about a venue
#[derive(Clone, Debug, Serialize)]
pub struct SendVenue {
    chat_id: ChatId,
    latitude: Float,
    longitude: Float,
    title: String,
    address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl SendVenue {
    /// Creates a new SendVenue with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * latitude - Latitude of the venue
    /// * longitude - Longitude of the venue
    /// * title - Name of the venue
    /// * address - Address of the venue
    pub fn new<C, T, A>(chat_id: C, latitude: Float, longitude: Float, title: T, address: A) -> Self
    where
        C: Into<ChatId>,
        T: Into<String>,
        A: Into<String>,
    {
        SendVenue {
            chat_id: chat_id.into(),
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Foursquare identifier of the venue
    pub fn foursquare_id<S: Into<String>>(mut self, foursquare_id: S) -> Self {
        self.foursquare_id = Some(foursquare_id.into());
        self
    }

    /// Foursquare type of the venue, if known
    ///
    /// For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”
    pub fn foursquare_type<S: Into<String>>(mut self, foursquare_type: S) -> Self {
        self.foursquare_type = Some(foursquare_type.into());
        self
    }

    // Sends the message silently
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

impl Method for SendVenue {
    type Response = Message;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("sendVenue", &self)
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

    #[test]
    fn send_venue() {
        let request = SendVenue::new(1, 2.0, 3.0, "title", "addr")
            .foursquare_id("f-id")
            .foursquare_type("f-type")
            .disable_notification(true)
            .reply_to_message_id(1)
            .reply_markup(ForceReply::new(true))
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/sendVenue");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(
                data,
                serde_json::json!({
                    "chat_id": 1,
                    "latitude": 2.0,
                    "longitude": 3.0,
                    "title": "title",
                    "address": "addr",
                    "foursquare_id": "f-id",
                    "foursquare_type": "f-type",
                    "disable_notification": true,
                    "reply_to_message_id": 1,
                    "reply_markup": {"force_reply": true}
                })
            );
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
