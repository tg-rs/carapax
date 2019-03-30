use crate::{
    methods::Method,
    request::{Form, RequestBuilder},
    types::{ChatId, EditMessageResult, InlineKeyboardMarkup, InputMedia, Integer},
};
use failure::Error;

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
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Result<Self, Error> {
        let reply_markup = serde_json::to_string(&reply_markup.into())?;
        self.form.insert_field("reply_markup", reply_markup);
        Ok(self)
    }
}

impl Method for EditMessageMedia {
    type Response = EditMessageResult;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::form("editMessageMedia", self.form)
    }
}
