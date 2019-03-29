use crate::{
    methods::Method,
    request::{Form, RequestBuilder},
    types::{ChatId, InputFile, Integer, Message, ParseMode, ReplyMarkup},
};
use failure::Error;

/// Send audio files
///
/// Your audio must be in the .mp3 format
/// Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future
///
/// For sending voice messages, use the sendVoice method instead
#[derive(Debug)]
pub struct SendAudio {
    form: Form,
}

impl SendAudio {
    /// Creates a new SendAudio with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * audio - Audio file to send
    pub fn new<C, A>(chat_id: C, audio: A) -> Self
    where
        C: Into<ChatId>,
        A: Into<InputFile>,
    {
        let mut form = Form::new();
        form.set_field("chat_id", chat_id.into());
        form.set_field("audio", audio.into());
        SendAudio { form }
    }

    /// Sets audio caption (0-1024 characters)
    pub fn caption<S: Into<String>>(mut self, value: S) -> Self {
        self.form.set_field("caption", value.into());
        self
    }

    /// Sets parse mode
    pub fn parse_mode(mut self, value: ParseMode) -> Self {
        self.form.set_field("parse_mode", value);
        self
    }

    /// Sets duration of the audio in seconds
    pub fn duration(mut self, value: Integer) -> Self {
        self.form.set_field("duration", value);
        self
    }

    /// Sets performer
    pub fn performer<S: Into<String>>(mut self, value: S) -> Self {
        self.form.set_field("performer", value.into());
        self
    }

    /// Sets track name
    pub fn title<S: Into<String>>(mut self, value: S) -> Self {
        self.form.set_field("title", value.into());
        self
    }

    /// Sets thumbnail of the file
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size
    /// A thumbnail‘s width and height should not exceed 90
    ///
    /// Ignored if the file is not uploaded using multipart/form-data
    /// Thumbnails can’t be reused and can be only uploaded as a new file,
    /// so you can pass “attach://<file_attach_name>” if the thumbnail
    /// was uploaded using multipart/form-data under <file_attach_name>
    pub fn thumb<V>(mut self, value: V) -> Self
    where
        V: Into<InputFile>,
    {
        self.form.set_field("thumb", value.into());
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

impl Method for SendAudio {
    type Response = Message;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::form("sendAudio", self.form)
    }
}
