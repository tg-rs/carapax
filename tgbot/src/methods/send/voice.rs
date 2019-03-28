use crate::{
    methods::Method,
    request::{Form, RequestBuilder},
    types::{ChatId, InputFile, Integer, Message, ParseMode, ReplyMarkup},
};
use failure::Error;

/// Send audio files, if you want Telegram clients to display the file as a playable voice message
///
/// For this to work, your audio must be in an .ogg file encoded with OPUS
/// (other formats may be sent as Audio or Document)
/// Bots can currently send voice messages of up to 50 MB in size,
/// this limit may be changed in the future
#[derive(Debug)]
pub struct SendVoice {
    form: Form,
}

impl SendVoice {
    /// Creates a new SendVoice with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * voice - Audio file to send
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, voice: InputFile) -> Self {
        let mut form = Form::new();
        form.set_field("chat_id", chat_id.into());
        form.set_field("voice", voice);
        SendVoice { form }
    }

    /// Voice message caption, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, value: S) -> Self {
        self.form.set_field("caption", value.into());
        self
    }

    /// Sets a parse mode
    pub fn parse_mode(mut self, value: ParseMode) -> Self {
        self.form.set_field("parse_mode", value);
        self
    }

    /// Duration of the voice message in seconds
    pub fn duration(mut self, value: Integer) -> Self {
        self.form.set_field("duration", value);
        self
    }

    // Sends the message silently
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.form.set_field("disable_notification", value);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, value: Integer) -> Self {
        self.form.set_field("reply_to_message_id", value);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(mut self, value: R) -> Result<Self, Error> {
        let value = serde_json::to_string(&value.into())?;
        self.form.set_field("reply_markup", value);
        Ok(self)
    }
}

impl Method for SendVoice {
    type Response = Message;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::form("sendVoice", self.form)
    }
}
