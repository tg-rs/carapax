use crate::types::primitive::{Float, Integer, ParseMode};

/// Content of a message to be sent as a result of an inline query
#[derive(Clone, Debug, Serialize)]
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

macro_rules! impl_input_message_content_from {
    ($to:ident($from:ident)) => {
        impl From<$from> for InputMessageContent {
            fn from(obj: $from) -> InputMessageContent {
                InputMessageContent::$to(obj)
            }
        }
    };
}

impl_input_message_content_from!(Contact(InputMessageContentContact));
impl_input_message_content_from!(Location(InputMessageContentLocation));
impl_input_message_content_from!(Text(InputMessageContentText));
impl_input_message_content_from!(Venue(InputMessageContentVenue));

/// Contact message to be sent as the result of an inline query
#[derive(Clone, Debug, Serialize)]
pub struct InputMessageContentContact {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

/// Location message to be sent as the result of an inline query
#[derive(Clone, Debug, Serialize)]
pub struct InputMessageContentLocation {
    /// Latitude of the location in degrees
    pub latitude: Float,
    /// Longitude of the location in degrees
    pub longitude: Float,
    /// Period in seconds for which the location can be updated, should be between 60 and 86400
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<Integer>,
}

/// Text message to be sent as the result of an inline query
#[derive(Clone, Debug, Serialize)]
pub struct InputMessageContentText {
    /// Text of the message to be sent, 1-4096 characters
    pub message_text: String,
    /// Parse mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Disables link previews for links in the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
}

/// Venue message to be sent as the result of an inline query
#[derive(Clone, Debug, Serialize)]
pub struct InputMessageContentVenue {
    /// Latitude of the venue in degrees
    pub latitude: Float,
    /// Longitude of the venue in degrees
    pub longitude: Float,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue, if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known.
    /// (For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
}
