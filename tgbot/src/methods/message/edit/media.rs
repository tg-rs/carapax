use crate::{
    methods::Method,
    request::{Form, Request},
    types::{ChatId, EditMessageResult, InlineKeyboardError, InlineKeyboardMarkup, InputMedia, Integer},
};

/// Edit audio, document, photo, or video messages
///
/// If a message is a part of a message album, then it can be edited only to a photo or a video
/// Otherwise, message type can be changed arbitrarily
/// When inline message is edited, new file can't be uploaded
/// Use previously uploaded file via its file_id or specify a URL
#[derive(Debug)]
pub struct EditMessageMedia {
    form: Form,
}

impl EditMessageMedia {
    /// Creates a new EditMessageMedia
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    /// * media - New media content of the message
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer, media: InputMedia) -> Self {
        let mut form = Form::new();
        form.insert_field("chat_id", chat_id.into());
        form.insert_field("message_id", message_id);
        for (k, v) in media.into_form() {
            form.insert_field(k, v);
        }
        EditMessageMedia { form }
    }

    /// Creates a new EditMessageMedia
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    /// * media - New media content of the message
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S, media: InputMedia) -> Self {
        let mut form = Form::new();
        form.insert_field("inline_message_id", inline_message_id.into());
        for (k, v) in media.into_form() {
            form.insert_field(k, v);
        }
        EditMessageMedia { form }
    }

    /// New inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Result<Self, InlineKeyboardError> {
        let reply_markup = reply_markup.into().serialize()?;
        self.form.insert_field("reply_markup", reply_markup);
        Ok(self)
    }
}

impl Method for EditMessageMedia {
    type Response = EditMessageResult;

    fn into_request(self) -> Request {
        Request::form("editMessageMedia", self.form)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::{InlineKeyboardButton, InputFile, InputMediaPhoto},
    };

    #[test]
    fn edit_message_media() {
        let request = EditMessageMedia::new(
            1,
            2,
            InputMedia::new(InputFile::file_id("file-id"), InputMediaPhoto::default()).unwrap(),
        )
        .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
        .unwrap()
        .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/editMessageMedia"
        );
        if let RequestBody::Form(form) = request.into_body() {
            assert_eq!(form.fields["chat_id"].get_text().unwrap(), "1");
            assert_eq!(form.fields["message_id"].get_text().unwrap(), "2");
            assert!(form.fields.get("media").is_some());
            assert!(form.fields.get("reply_markup").is_some());
        } else {
            panic!("Unexpected request body");
        }

        let request = EditMessageMedia::with_inline_message_id(
            "msg-id",
            InputMedia::new(InputFile::file_id("file-id"), InputMediaPhoto::default()).unwrap(),
        )
        .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/editMessageMedia"
        );
        if let RequestBody::Form(form) = request.into_body() {
            assert_eq!(form.fields["inline_message_id"].get_text().unwrap(), "msg-id");
            assert!(form.fields.get("media").is_some());
        } else {
            panic!("Unexpected request body");
        }
    }
}
