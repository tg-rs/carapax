use crate::types::{
    inline_mode::message_content::InputMessageContent, primitive::ParseMode, reply_markup::InlineKeyboardMarkup,
};
use serde::Serialize;

/// Link to an animated GIF file stored on the Telegram servers
///
/// By default, this animated GIF file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content to send
/// a message with specified content instead of the animation
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultCachedGif {
    id: String,
    gif_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedGif {
    /// Creates a new InlineQueryResultCachedGif with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * gif_file_id - A valid file identifier for the GIF file
    pub fn new<I, F>(id: I, gif_file_id: F) -> Self
    where
        I: Into<String>,
        F: Into<String>,
    {
        InlineQueryResultCachedGif {
            id: id.into(),
            gif_file_id: gif_file_id.into(),
            title: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// Title for the result
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Caption of the GIF file to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the GIF animation
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }
}
