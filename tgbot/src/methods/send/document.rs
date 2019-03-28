use crate::{
    methods::Method,
    request::{Form, RequestBuilder},
    types::{ChatId, InputFile, Integer, Message, ParseMode, ReplyMarkup},
};
use failure::Error;

/// Send general files
///
/// Bots can currently send files of any type of up to 50 MB in size,
/// this limit may be changed in the future
#[derive(Debug)]
pub struct SendDocument {
    form: Form,
}

impl SendDocument {
    /// Creates a new SendDocument with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * document - File to send
    pub fn new<C: Into<ChatId>>(chat_id: C, document: InputFile) -> Self {
        let mut form = Form::new();
        form.set_field("chat_id", chat_id.into());
        form.set_field("document", document);
        SendDocument { form }
    }

    /// Thumbnail of the file sent
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size
    /// A thumbnail‘s width and height should not exceed 90
    /// Ignored if the file is not uploaded using multipart/form-data
    /// Thumbnails can’t be reused and can be only uploaded as a new file,
    /// so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data under <file_attach_name>
    pub fn thumb(mut self, value: InputFile) -> Self {
        self.form.set_field("thumb", value);
        self
    }

    /// Document caption
    ///
    /// May also be used when resending documents by file_id
    /// 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, value: S) -> Self {
        self.form.set_field("caption", value.into());
        self
    }

    /// Parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.form.set_field("parse_mode", parse_mode);
        self
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

impl Method for SendDocument {
    type Response = Message;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::form("sendDocument", self.form)
    }
}
