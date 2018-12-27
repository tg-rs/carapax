use crate::methods::method::*;
use crate::types::{ChatId, EditMessageResult, InlineKeyboardMarkup, InputMedia, Integer};
use serde::Serialize;

/// Edit audio, document, photo, or video messages
///
/// If a message is a part of a message album, then it can be edited only to a photo or a video
/// Otherwise, message type can be changed arbitrarily
/// When inline message is edited, new file can't be uploaded
/// Use previously uploaded file via its file_id or specify a URL
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageMedia {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    media: InputMedia,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
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
        EditMessageMedia {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            media,
            reply_markup: None,
        }
    }

    /// Creates a new EditMessageMedia
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    /// * media - New media content of the message
    pub fn with_inline_message_id<S: Into<String>>(
        inline_message_id: S,
        media: InputMedia,
    ) -> Self {
        EditMessageMedia {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            media,
            reply_markup: None,
        }
    }

    /// New inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(&mut self, reply_markup: I) -> &mut Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for EditMessageMedia {
    type Response = EditMessageResult;

    fn get_request(&self) -> Result<RequestBuilder, RequestError> {
        RequestBuilder::json("editMessageMedia", &self)
    }
}
