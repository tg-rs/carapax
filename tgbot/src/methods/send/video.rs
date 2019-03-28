use crate::{
    methods::Method,
    request::{Form, RequestBuilder},
    types::{ChatId, InputFile, Integer, Message, ParseMode, ReplyMarkup},
};
use failure::Error;

/// Send video file
///
/// Telegram clients support mp4 videos (other formats may be sent as Document)
/// Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future
#[derive(Debug)]
pub struct SendVideo {
    form: Form,
}

impl SendVideo {
    /// Creates a new SendVideo with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * video - Video to send
    pub fn new<C: Into<ChatId>>(chat_id: C, video: InputFile) -> Self {
        let mut form = Form::new();
        form.set_field("chat_id", chat_id.into());
        form.set_field("video", video);
        SendVideo { form }
    }

    /// Duration of sent video in seconds
    pub fn duration(mut self, value: Integer) -> Self {
        self.form.set_field("duration", value);
        self
    }

    /// Video width
    pub fn width(mut self, value: Integer) -> Self {
        self.form.set_field("width", value);
        self
    }

    /// Video height
    pub fn height(mut self, value: Integer) -> Self {
        self.form.set_field("height", value);
        self
    }

    /// Thumbnail of the file sent
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size
    /// A thumbnail‘s width and height should not exceed 90
    /// Ignored if the file is not uploaded using multipart/form-data
    /// Thumbnails can’t be reused and can be only uploaded as a new file,
    /// so you can pass “attach://<file_attach_name>” if the thumbnail was
    /// uploaded using multipart/form-data under <file_attach_name>
    pub fn thumb(mut self, value: InputFile) -> Self {
        self.form.set_field("thumb", value);
        self
    }

    /// Video caption
    ///
    /// May also be used when resending videos by file_id
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

    /// Pass True, if the uploaded video is suitable for streaming
    pub fn supports_streaming(mut self, value: bool) -> Self {
        self.form.set_field("supports_streaming", value);
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

impl Method for SendVideo {
    type Response = Message;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::form("sendVideo", self.form)
    }
}
