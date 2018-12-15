use crate::types::primitive::{Float, Integer};

/// This object represents the content of a message to be sent as a result of an inline query.
#[derive(Debug)]
pub enum InputMessageContent {
    /// Represents the content of a text message to be sent as the result of an inline query.
    Text(InputTextMessageContent),
    /// Represents the content of a location message to be sent as the result of an inline query.
    Location(InputLocationMessageContent),
    /// Represents the content of a venue message to be sent as the result of an inline query.
    Venue(InputVenueMessageContent),
    /// Represents the content of a contact message to be sent as the result of an inline query.
    Contact(InputContactMessageContent),
}

/// Represents the content of a text message to be sent as the result of an inline query.
#[derive(Debug)]
pub struct InputTextMessageContent {
    /// Text of the message to be sent, 1-4096 characters
    pub message_text: String,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text or
    /// inline URLs in your bot's message.
    pub parse_mode: Option<String>,
    ///  Disables link previews for links in the sent message
    pub disable_web_page_preview: Option<bool>,
}

/// Represents the content of a location message to be sent as the result of an inline query.
#[derive(Debug)]
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees
    pub latitude: Float,
    /// Longitude of the location in degrees
    pub longitude: Float,
    /// Period in seconds for which the location can be updated, should be between 60 and 86400.
    pub live_period: Option<Integer>,
}

/// Represents the content of a venue message to be sent as the result of an inline query.
#[derive(Debug)]
pub struct InputVenueMessageContent {
    /// Latitude of the venue in degrees
    pub latitude: Float,
    /// Longitude of the venue in degrees
    pub longitude: Float,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue, if known
    pub foursquare_id: Option<String>,
    ///  Foursquare type of the venue, if known.
    /// (For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
}

/// Represents the content of a contact message to be sent as the result of an inline query.
#[derive(Debug)]
pub struct InputContactMessageContent {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub vcard: Option<String>,
}
