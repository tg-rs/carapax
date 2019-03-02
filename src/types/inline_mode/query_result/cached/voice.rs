use crate::types::inline_mode::message_content::InputMessageContent;
use crate::types::primitive::ParseMode;
use crate::types::reply_markup::InlineKeyboardMarkup;
use serde::Serialize;

/// Link to a voice message stored on the Telegram servers
///
/// By default, this voice message will be sent by the user
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the voice message
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultCachedVoice {
    id: String,
    voice_file_id: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedVoice {
    /// Creates a new InlineQueryResultCachedVoice with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * voice_file_id - A valid file identifier for the voice message
    /// * title - Title for the result
    pub fn new<S: Into<String>>(id: S, voice_file_id: S, title: S) -> Self {
        InlineQueryResultCachedVoice {
            id: id.into(),
            voice_file_id: voice_file_id.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// Caption, 0-1024 characters
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

    /// Content of the message to be sent instead of the voice message
    pub fn input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }
}
