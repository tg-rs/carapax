use crate::types::{
    inline_mode::message_content::InputMessageContent,
    primitive::{Integer, ParseMode},
    reply_markup::InlineKeyboardMarkup,
};
use serde::Serialize;

/// Link to a voice recording in an .ogg container encoded with OPUS
///
/// By default, this voice recording will be sent by the user
/// Alternatively, you can use input_message_content to send
/// a message with the specified content instead of the the voice message
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultVoice {
    id: String,
    voice_url: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    voice_duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultVoice {
    /// Creates a new InlineQueryResultVoice with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * voice_url - A valid URL for the voice recording
    /// * title - Recording title
    pub fn new<S: Into<String>>(id: S, voice_url: S, title: S) -> Self {
        InlineQueryResultVoice {
            id: id.into(),
            voice_url: voice_url.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            voice_duration: None,
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

    /// Recording duration in seconds
    pub fn voice_duration(mut self, voice_duration: Integer) -> Self {
        self.voice_duration = Some(voice_duration);
        self
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the voice recording
    pub fn input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }
}
