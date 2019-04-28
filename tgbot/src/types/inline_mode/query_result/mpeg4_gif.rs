use crate::types::{
    inline_mode::message_content::InputMessageContent,
    primitive::{Integer, ParseMode},
    reply_markup::InlineKeyboardMarkup,
};
use serde::Serialize;

/// Link to a video animation (H.264/MPEG-4 AVC video without sound)
///
/// By default, this animated MPEG-4 file will be sent by the user with optional caption
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the animation
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultMpeg4Gif {
    id: String,
    mpeg4_url: String,
    thumb_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    mpeg4_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mpeg4_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mpeg4_duration: Option<Integer>,
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

impl InlineQueryResultMpeg4Gif {
    /// Creates a new InlineQueryResultMpeg4Gif with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * mpeg4_url - A valid URL for the MP4 file. File size must not exceed 1MB
    /// * thumb_url - URL of the static thumbnail (jpeg or gif) for the result
    pub fn new<I, U, T>(id: I, mpeg4_url: U, thumb_url: T) -> Self
    where
        I: Into<String>,
        U: Into<String>,
        T: Into<String>,
    {
        InlineQueryResultMpeg4Gif {
            id: id.into(),
            mpeg4_url: mpeg4_url.into(),
            mpeg4_width: None,
            mpeg4_height: None,
            mpeg4_duration: None,
            thumb_url: thumb_url.into(),
            title: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// Video width
    pub fn mpeg4_width(mut self, mpeg4_width: Integer) -> Self {
        self.mpeg4_width = Some(mpeg4_width);
        self
    }

    /// Video height
    pub fn mpeg4_height(mut self, mpeg4_height: Integer) -> Self {
        self.mpeg4_height = Some(mpeg4_height);
        self
    }

    /// Video duration
    pub fn mpeg4_duration(mut self, mpeg4_duration: Integer) -> Self {
        self.mpeg4_duration = Some(mpeg4_duration);
        self
    }

    /// Title for the result
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Caption of the MPEG-4 file to be sent, 0-1024 characters
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

    /// Content of the message to be sent instead of the video animation
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }
}
