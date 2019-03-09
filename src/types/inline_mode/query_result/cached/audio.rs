use crate::types::{
    inline_mode::message_content::InputMessageContent, primitive::ParseMode, reply_markup::InlineKeyboardMarkup,
};
use serde::Serialize;

/// Link to an mp3 audio file stored on the Telegram servers
///
/// By default, this audio file will be sent by the user
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the audio
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultCachedAudio {
    id: String,
    audio_file_id: String,
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedAudio {
    /// Creates a new InlineQueryResultCachedAudio with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * audio_file_id - A valid file identifier for the audio file
    pub fn new<S: Into<String>>(id: S, audio_file_id: S) -> Self {
        InlineQueryResultCachedAudio {
            id: id.into(),
            audio_file_id: audio_file_id.into(),
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

    /// Content of the message to be sent instead of the audio
    pub fn input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }
}
