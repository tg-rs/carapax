use crate::types::inline_mode::message_content::InputMessageContent;
use crate::types::primitive::{Float, Integer};
use crate::types::reply_markup::InlineKeyboardMarkup;

/// Location on a map
///
/// By default, the location will be sent by the user
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the location
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultLocation {
    id: String,
    latitude: Float,
    longitude: Float,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_period: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_height: Option<Integer>,
}

impl InlineQueryResultLocation {
    /// Creates a new InlineQueryResultLocation with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * latitude - Location latitude in degrees
    /// * longitude - Location longitude in degrees
    /// * title - Location title
    pub fn new<S: Into<String>>(id: S, latitude: Float, longitude: Float, title: S) -> Self {
        InlineQueryResultLocation {
            id: id.into(),
            latitude,
            longitude,
            title: title.into(),
            live_period: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }

    /// Period in seconds for which the location can be updated, should be between 60 and 86400
    pub fn live_period(&mut self, live_period: Integer) -> &mut Self {
        self.live_period = Some(live_period);
        self
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(&mut self, reply_markup: I) -> &mut Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the location
    pub fn input_message_content(
        &mut self,
        input_message_content: InputMessageContent,
    ) -> &mut Self {
        self.input_message_content = Some(input_message_content);
        self
    }

    /// Url of the thumbnail for the result
    pub fn thumb_url<S: Into<String>>(&mut self, thumb_url: S) -> &mut Self {
        self.thumb_url = Some(thumb_url.into());
        self
    }

    /// Thumbnail width
    pub fn thumb_width(&mut self, thumb_width: Integer) -> &mut Self {
        self.thumb_width = Some(thumb_width);
        self
    }

    /// Thumbnail height
    pub fn thumb_height(&mut self, thumb_height: Integer) -> &mut Self {
        self.thumb_height = Some(thumb_height);
        self
    }
}
