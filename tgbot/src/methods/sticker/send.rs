use crate::{
    methods::Method,
    request::{Form, RequestBuilder},
    types::{ChatId, InputFile, Integer, Message, ReplyMarkup},
};
use failure::Error;

/// Send .webp sticker
#[derive(Debug)]
pub struct SendSticker {
    form: Form,
}

impl SendSticker {
    /// Creates a new SendSticker with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * sticker - Sticker to send
    ///             Pass a file_id as String to send a file that exists on the Telegram servers (recommended),
    ///             pass an HTTP URL as a String for Telegram to get a .webp file from the Internet,
    ///             or upload a new one using multipart/form-data
    pub fn new<C, S>(chat_id: C, sticker: S) -> Self
    where
        C: Into<ChatId>,
        S: Into<InputFile>,
    {
        let mut form = Form::new();
        form.set_field("chat_id", chat_id.into());
        form.set_field("sticker", sticker.into());
        SendSticker { form }
    }

    /// Sends the message silently
    ///
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

impl Method for SendSticker {
    type Response = Message;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::form("sendSticker", self.form)
    }
}
