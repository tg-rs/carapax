use crate::types::inline_mode::message_content::InputMessageContent;
use crate::types::primitive::{Integer, ParseMode};
use crate::types::reply_markup::InlineKeyboardMarkup;

/// Link to a photo
///
/// By default, this photo will be sent by the user with optional caption
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the photo
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultPhoto {
    id: String,
    photo_url: String,
    thumb_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultPhoto {
    /// Creates a new InlineQueryResultPhoto with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * photo_url - A valid URL of the photo, must be in jpeg format, size must not exceed 5MB
    /// * thumb_url - URL of the thumbnail for the photo
    pub fn new<S: Into<String>>(id: S, photo_url: S, thumb_url: S) -> Self {
        InlineQueryResultPhoto {
            id: id.into(),
            photo_url: photo_url.into(),
            thumb_url: thumb_url.into(),
            photo_width: None,
            photo_height: None,
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// Width of the photo
    pub fn photo_width(&mut self, photo_width: Integer) -> &mut Self {
        self.photo_width = Some(photo_width);
        self
    }

    /// Height of the photo
    pub fn photo_height(&mut self, photo_height: Integer) -> &mut Self {
        self.photo_height = Some(photo_height);
        self
    }

    /// Title for the result
    pub fn title<S: Into<String>>(&mut self, title: S) -> &mut Self {
        self.title = Some(title.into());
        self
    }

    /// Short description of the result
    pub fn description<S: Into<String>>(&mut self, description: S) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    /// Caption of the photo to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(&mut self, caption: S) -> &mut Self {
        self.caption = Some(caption.into());
        self
    }

    /// Parse mode
    pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup(&mut self, reply_markup: InlineKeyboardMarkup) -> &mut Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    /// Content of the message to be sent instead of the photo
    pub fn input_message_content(
        &mut self,
        input_message_content: InputMessageContent,
    ) -> &mut Self {
        self.input_message_content = Some(input_message_content);
        self
    }
}
