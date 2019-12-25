use crate::{
    methods::Method,
    request::{Form, Request},
    types::{ChatId, InputFile, Integer, Message, ReplyMarkup, ReplyMarkupError},
};

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
        form.insert_field("chat_id", chat_id.into());
        form.insert_field("sticker", sticker.into());
        SendSticker { form }
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
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

impl Method for SendSticker {
    type Response = Message;

    fn into_request(self) -> Request {
        Request::form("sendSticker", self.form)
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
    fn send_sticker() {
        let request = SendSticker::new(1, InputFile::file_id("sticker-id"))
            .disable_notification(true)
            .reply_to_message_id(1)
            .reply_markup(ForceReply::new(true))
            .unwrap()
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendSticker");
        if let RequestBody::Form(form) = request.into_body() {
            assert_eq!(form.fields["chat_id"].get_text().unwrap(), "1");
            assert!(form.fields["sticker"].get_file().is_some());
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
