use crate::types::{
    inline_mode::message_content::InputMessageContent,
    primitive::{Integer, ParseMode},
    reply_markup::InlineKeyboardMarkup,
};
use serde::Serialize;

/// Link to an animated GIF file
///
/// By default, this animated GIF file
/// will be sent by the user with optional caption
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the animation
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultGif {
    id: String,
    gif_url: String,
    thumb_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    gif_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gif_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gif_duration: Option<Integer>,
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

impl InlineQueryResultGif {
    /// Creates a new InlineQueryResultGif with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * gif_url - A valid URL for the GIF file. File size must not exceed 1MB
    /// * thumb_url - URL of the static thumbnail for the result (jpeg or gif)
    pub fn new<I, U, T>(id: I, gif_url: U, thumb_url: T) -> Self
    where
        I: Into<String>,
        U: Into<String>,
        T: Into<String>,
    {
        InlineQueryResultGif {
            id: id.into(),
            gif_url: gif_url.into(),
            gif_width: None,
            gif_height: None,
            gif_duration: None,
            thumb_url: thumb_url.into(),
            title: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// Width of the GIF
    pub fn gif_width(mut self, gif_width: Integer) -> Self {
        self.gif_width = Some(gif_width);
        self
    }

    /// Height of the GIF
    pub fn gif_height(mut self, gif_height: Integer) -> Self {
        self.gif_height = Some(gif_height);
        self
    }

    /// Duration of the GIF
    pub fn gif_duration(mut self, gif_duration: Integer) -> Self {
        self.gif_duration = Some(gif_duration);
        self
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
