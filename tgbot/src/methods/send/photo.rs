use crate::{
    methods::Method,
    request::{Form, RequestBuilder},
    types::{ChatId, InputFile, Integer, Message, ParseMode, ReplyMarkup},
};
use failure::Error;

/// Send photo
#[derive(Debug)]
pub struct SendPhoto {
    form: Form,
}

impl SendPhoto {
    /// Creates a new SendPhoto with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * photo - Photo to send
    pub fn new<C: Into<ChatId>>(chat_id: C, photo: InputFile) -> Self {
        let mut form = Form::new();
        form.set_field("chat_id", chat_id.into());
        form.set_field("photo", photo);
        Self { form }
    }

    /// Photo caption
    ///
    /// May also be used when resending photos by file_id
    /// 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, value: S) -> Self {
        self.form.set_field("caption", value.into());
        self
    }

    /// Sets a parse mode
    pub fn parse_mode(mut self, value: ParseMode) -> Self {
        self.form.set_field("parse_mode", value);
        self
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.form.set_field("disable_notification", value.to_string());
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, value: Integer) -> Self {
        self.form.set_field("reply_to_message_id", value.to_string());
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(mut self, value: R) -> Result<Self, Error> {
        let value = serde_json::to_string(&value.into())?;
        self.form.set_field("reply_markup", value);
        Ok(self)
    }
}

impl Method for SendPhoto {
    type Response = Message;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::form("sendPhoto", self.form)
    }
}
