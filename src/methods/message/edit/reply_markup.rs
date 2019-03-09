use crate::{
    methods::method::*,
    types::{ChatId, EditMessageResult, InlineKeyboardMarkup, Integer},
};
use failure::Error;
use serde::Serialize;

/// Edit only the reply markup of messages sent by the bot or via the bot (for inline bots)
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageReplyMarkup {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageReplyMarkup {
    /// Creates a new EditMessageReplyMarkup
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer) -> Self {
        EditMessageReplyMarkup {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            reply_markup: None,
        }
    }

    /// Creates a new EditMessageReplyMarkup
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S) -> Self {
        EditMessageReplyMarkup {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            reply_markup: None,
        }
    }

    /// Inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for EditMessageReplyMarkup {
    type Response = EditMessageResult;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("editMessageReplyMarkup", &self)
    }
}
