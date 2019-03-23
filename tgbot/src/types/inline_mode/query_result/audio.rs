use crate::types::{
    inline_mode::message_content::InputMessageContent,
    primitive::{Integer, ParseMode},
    reply_markup::InlineKeyboardMarkup,
};
use serde::Serialize;

/// Link to an mp3 audio file
///
/// By default, this audio file will be sent by the user
/// Alternatively, you can use input_message_content to send
/// a message with the specified content instead of the audio
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultAudio {
    id: String,
    audio_url: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultAudio {
    /// Creates a new InlineQueryResultAudio with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * audio_url - A valid URL for the audio file
    /// * title - Title
    pub fn new<S: Into<String>>(id: S, audio_url: S, title: S) -> Self {
        InlineQueryResultAudio {
            id: id.into(),
            audio_url: audio_url.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            performer: None,
            audio_duration: None,
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

    /// Performer
    pub fn performer<S: Into<String>>(mut self, performer: S) -> Self {
        self.performer = Some(performer.into());
        self
    }

    /// Audio duration in seconds
    pub fn audio_duration(mut self, audio_duration: Integer) -> Self {
        self.audio_duration = Some(audio_duration);
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
