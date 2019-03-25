use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, Integer, Message, ParseMode, ReplyMarkup},
};
use failure::Error;
use serde::Serialize;

/// Send audio files, if you want Telegram clients to display the file as a playable voice message
///
/// For this to work, your audio must be in an .ogg file encoded with OPUS
/// (other formats may be sent as Audio or Document)
/// Bots can currently send voice messages of up to 50 MB in size,
/// this limit may be changed in the future
#[derive(Clone, Debug, Serialize)]
pub struct SendVoice {
    chat_id: ChatId,
    voice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl SendVoice {
    /// Creates a new SendVoice with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * voice - Audio file to send
    ///           Pass a file_id as String to send a file that exists on the Telegram servers (recommended),
    ///           pass an HTTP URL as a String for Telegram to get a file from the Internet,
    ///           or upload a new one using multipart/form-data
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, voice: S) -> Self {
        SendVoice {
            chat_id: chat_id.into(),
            voice: voice.into(),
            caption: None,
            parse_mode: None,
            duration: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Voice message caption, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Sets a parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Duration of the voice message in seconds
    pub fn duration(mut self, duration: Integer) -> Self {
        self.duration = Some(duration);
        self
    }

    // Sends the message silently
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(mut self, reply_markup: R) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendVoice {
    type Response = Message;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("sendVoice", &self)
    }
}
