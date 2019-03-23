use crate::{
    methods::method::*,
    types::{ChatId, Integer, Message, ReplyMarkup},
};
use failure::Error;
use serde::Serialize;

/// Send .webp sticker
#[derive(Clone, Debug, Serialize)]
pub struct SendSticker {
    chat_id: ChatId,
    sticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
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
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, sticker: S) -> Self {
        SendSticker {
            chat_id: chat_id.into(),
            sticker: sticker.into(),
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    // Sends the message silently
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(mut self, reply_markup: R) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendSticker {
    type Response = Message;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("sendSticker", &self)
    }
}
