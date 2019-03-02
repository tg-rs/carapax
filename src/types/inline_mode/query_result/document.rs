use crate::types::inline_mode::message_content::InputMessageContent;
use crate::types::primitive::{Integer, ParseMode};
use crate::types::reply_markup::InlineKeyboardMarkup;
use serde::Serialize;

/// Link to a file
///
/// By default, this file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the file
/// Currently, only .PDF and .ZIP files can be sent using this method
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultDocument {
    id: String,
    title: String,
    document_url: String,
    mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
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

impl InlineQueryResultDocument {
    /// Creates a new InlineQueryResultDocument with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * title - Title for the result
    /// * document_url - A valid URL for the file
    /// * mime_type - Mime type of the content of the file, either “application/pdf” or “application/zip”
    pub fn new<S: Into<String>>(id: S, title: S, document_url: S, mime_type: S) -> Self {
        InlineQueryResultDocument {
            id: id.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            document_url: document_url.into(),
            mime_type: mime_type.into(),
            description: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }

    /// Caption of the document to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Short description of the result
    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the file
    pub fn input_message_content(
        mut self,
        input_message_content: InputMessageContent,
    ) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }

    /// URL of the thumbnail (jpeg only) for the file
    pub fn thumb_url<S: Into<String>>(mut self, thumb_url: S) -> Self {
        self.thumb_url = Some(thumb_url.into());
        self
    }

    /// Thumbnail width
    pub fn thumb_width(mut self, thumb_width: Integer) -> Self {
        self.thumb_width = Some(thumb_width);
        self
    }

    /// Thumbnail height
    pub fn thumb_height(mut self, thumb_height: Integer) -> Self {
        self.thumb_height = Some(thumb_height);
        self
    }
}
