use crate::{
    methods::Method,
    request::{Form, Request},
    types::{ChatId, InputFile, Integer, Message, ParseMode, ReplyMarkup, ReplyMarkupError},
};

/// Send animation files (GIF or H.264/MPEG-4 AVC video without sound)
///
/// Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future
#[derive(Debug)]
pub struct SendAnimation {
    form: Form,
}

impl SendAnimation {
    /// Creates a new SendAnimation with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * animation - Animation to send
    pub fn new<C, A>(chat_id: C, animation: A) -> Self
    where
        C: Into<ChatId>,
        A: Into<InputFile>,
    {
        let mut form = Form::new();
        form.insert_field("chat_id", chat_id.into());
        form.insert_field("animation", animation.into());
        SendAnimation { form }
    }

    /// Sets duration of sent animation in seconds
    pub fn duration(mut self, value: Integer) -> Self {
        self.form.insert_field("duration", value);
        self
    }

    /// Sets animation width
    pub fn width(mut self, value: Integer) -> Self {
        self.form.insert_field("width", value);
        self
    }

    /// Sets animation height
    pub fn height(mut self, value: Integer) -> Self {
        self.form.insert_field("height", value);
        self
    }

    /// Sets thumbnail of the file sent
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size
    /// A thumbnail‘s width and height should not exceed 320
    /// Ignored if the file is not uploaded using multipart/form-data
    /// Thumbnails can’t be reused and can be only uploaded as a new file,
    /// so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data under <file_attach_name>
    pub fn thumb<V>(mut self, value: V) -> Self
    where
        V: Into<InputFile>,
    {
        self.form.insert_field("thumb", value.into());
        self
    }

    /// Sets animation caption
    ///
    /// May also be used when resending animation by file_id
    /// 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, value: S) -> Self {
        self.form.insert_field("caption", value.into());
        self
    }

    /// Sets parse mode
    pub fn parse_mode(mut self, value: ParseMode) -> Self {
        self.form.insert_field("parse_mode", value);
        self
    }

    /// Whether to send the message silently
    ///
    /// Users will receive a notification with no sound if enabled
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.form.insert_field("disable_notification", value);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, value: Integer) -> Self {
        self.form.insert_field("reply_to_message_id", value);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(mut self, value: R) -> Result<Self, ReplyMarkupError> {
        let value = value.into();
        self.form.insert_field("reply_markup", value.serialize()?);
        Ok(self)
    }
}

impl Method for SendAnimation {
    type Response = Message;

    fn into_request(self) -> Request {
        Request::form("sendAnimation", self.form)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::ForceReply,
    };

    #[test]
    fn send_animation() {
        let request = SendAnimation::new(1, InputFile::file_id("file-id"))
            .duration(100)
            .width(200)
            .height(300)
            .thumb(InputFile::file_id("thumb-id"))
            .caption("caption")
            .parse_mode(ParseMode::Markdown)
            .disable_notification(true)
            .reply_to_message_id(1)
            .reply_markup(ForceReply::new(true))
            .unwrap()
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/sendAnimation"
        );
        if let RequestBody::Form(form) = request.into_body() {
            assert_eq!(form.fields["chat_id"].get_text().unwrap(), "1");
            assert!(form.fields["animation"].get_file().is_some());
            assert_eq!(form.fields["duration"].get_text().unwrap(), "100");
            assert_eq!(form.fields["width"].get_text().unwrap(), "200");
            assert_eq!(form.fields["height"].get_text().unwrap(), "300");
            assert!(form.fields["thumb"].get_file().is_some());
            assert_eq!(form.fields["caption"].get_text().unwrap(), "caption");
            assert_eq!(form.fields["parse_mode"].get_text().unwrap(), "Markdown");
            assert_eq!(form.fields["disable_notification"].get_text().unwrap(), "true");
            assert_eq!(form.fields["reply_to_message_id"].get_text().unwrap(), "1");
            assert_eq!(
                form.fields["reply_markup"].get_text().unwrap(),
                r#"{"force_reply":true}"#
            );
        } else {
            panic!("Unexpected request body");
        }
    }
}
