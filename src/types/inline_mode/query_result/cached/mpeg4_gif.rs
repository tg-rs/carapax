use crate::types::inline_mode::message_content::InputMessageContent;
use crate::types::primitive::ParseMode;
use crate::types::reply_markup::InlineKeyboardMarkup;
use serde::Serialize;

/// Link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers
///
/// By default, this animated MPEG-4 file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content
/// to send a message with the specified content
/// instead of the animation
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultCachedMpeg4Gif {
    id: String,
    mpeg4_file_id: String,
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

impl InlineQueryResultCachedMpeg4Gif {
    /// Creates a new InlineQueryResultCachedMpeg4Gif with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * mpeg4_file_id - A valid file identifier for the MP4 file
    pub fn new<S: Into<String>>(id: S, mpeg4_file_id: S) -> Self {
        InlineQueryResultCachedMpeg4Gif {
            id: id.into(),
            mpeg4_file_id: mpeg4_file_id.into(),
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
    pub fn input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }
}
