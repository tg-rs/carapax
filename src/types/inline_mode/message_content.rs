use crate::types::primitive::{Float, Integer, ParseMode};
use serde::Serialize;

/// Content of a message to be sent as a result of an inline query
#[derive(Clone, Debug, From, Serialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    /// Contact message
    Contact(InputMessageContentContact),
    /// Location message
    Location(InputMessageContentLocation),
    /// Text message
    Text(InputMessageContentText),
    /// Venue message
    Venue(InputMessageContentVenue),
}

/// Contact message to be sent as the result of an inline query
#[derive(Clone, Debug, Serialize)]
pub struct InputMessageContentContact {
    phone_number: String,
    first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<String>,
}

impl InputMessageContentContact {
    /// Creates a new InputMessageContentContact with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * phone_numer - Contact's phone number
    /// * first_name - Contact's first name
    pub fn new<S: Into<String>>(phone_number: S, first_name: S) -> Self {
        InputMessageContentContact {
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
        }
    }

    /// Contact's last name
    pub fn last_name<S: Into<String>>(&mut self, last_name: S) -> &mut Self {
        self.last_name = Some(last_name.into());
        self
    }

    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub fn vcard<S: Into<String>>(&mut self, vcard: S) -> &mut Self {
        self.vcard = Some(vcard.into());
        self
    }
}

/// Location message to be sent as the result of an inline query
#[derive(Clone, Debug, Serialize)]
pub struct InputMessageContentLocation {
    latitude: Float,
    longitude: Float,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_period: Option<Integer>,
}

impl InputMessageContentLocation {
    /// Creates a new InputMessageContentLocation with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * latitude - Latitude of the location in degrees
    /// * longitude - Longitude of the location in degrees
    pub fn new(latitude: Float, longitude: Float) -> Self {
        InputMessageContentLocation {
            latitude,
            longitude,
            live_period: None,
        }
    }

    /// Period in seconds for which the location can be updated, should be between 60 and 86400
    pub fn live_period(&mut self, live_period: Integer) -> &mut Self {
        self.live_period = Some(live_period);
        self
    }
}

/// Text message to be sent as the result of an inline query
#[derive(Clone, Debug, Serialize)]
pub struct InputMessageContentText {
    message_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
}

impl InputMessageContentText {
    /// Creates a new InputMessageContentText with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * message_text - Text of the message to be sent, 1-4096 characters
    pub fn new<S: Into<String>>(message_text: S) -> Self {
        InputMessageContentText {
            message_text: message_text.into(),
            parse_mode: None,
            disable_web_page_preview: None,
        }
    }

    /// Parse mode
    pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Disables link previews for links in the sent message
    pub fn disable_web_page_preview(&mut self, disable_web_page_preview: bool) -> &mut Self {
        self.disable_web_page_preview = Some(disable_web_page_preview);
        self
    }
}

/// Venue message to be sent as the result of an inline query
#[derive(Clone, Debug, Serialize)]
pub struct InputMessageContentVenue {
    latitude: Float,
    longitude: Float,
    title: String,
    address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<String>,
}

impl InputMessageContentVenue {
    /// Creates a new InputMessageContentVenue with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * latitude - Latitude of the venue in degrees
    /// * longitude - Longitude of the venue in degrees
    /// * title - Name of the venue
    /// * address - Address of the venue
    pub fn new<S: Into<String>>(latitude: Float, longitude: Float, title: S, address: S) -> Self {
        InputMessageContentVenue {
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
        }
    }

    /// Foursquare identifier of the venue, if known
    pub fn foursquare_id<S: Into<String>>(&mut self, foursquare_id: S) -> &mut Self {
        self.foursquare_id = Some(foursquare_id.into());
        self
    }

    /// Foursquare type of the venue, if known
    ///
    /// For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”
    pub fn foursquare_type<S: Into<String>>(&mut self, foursquare_type: S) -> &mut Self {
        self.foursquare_type = Some(foursquare_type.into());
        self
    }
}
