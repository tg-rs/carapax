use crate::methods::method::*;
use crate::types::{ChatId, Integer, Message, ParseMode, ReplyMarkup};
use failure::Error;
use serde::Serialize;

/// Send text messages
#[derive(Clone, Debug, Serialize)]
pub struct SendMessage {
    chat_id: ChatId,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl SendMessage {
    /// Creates a new SendMessage with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * text - Text of the message to be sent
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, text: S) -> Self {
        SendMessage {
            chat_id: chat_id.into(),
            text: text.into(),
            parse_mode: None,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Sets parse mode
    pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Disables link previews for links in this message
    pub fn disable_web_page_preview(&mut self, disable_web_page_preview: bool) -> &mut Self {
        self.disable_web_page_preview = Some(disable_web_page_preview);
        self
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(&mut self, reply_to_message_id: Integer) -> &mut Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(&mut self, reply_markup: R) -> &mut Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendMessage {
    type Response = Message;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("sendMessage", &self)
    }
}
