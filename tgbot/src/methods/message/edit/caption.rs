use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, EditMessageResult, InlineKeyboardMarkup, Integer, ParseMode},
};
use failure::Error;
use serde::Serialize;

/// Edit caption of message sent by the bot or via the bot (for inline bots)
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageCaption {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageCaption {
    /// Creates a new EditMessageCaption
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer) -> Self {
        EditMessageCaption {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
        }
    }

    /// Creates a new EditMessageCaption
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S) -> Self {
        EditMessageCaption {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            caption: None,
            parse_mode: None,
            reply_markup: None,
        }
    }

    /// New caption of the message
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for EditMessageCaption {
    type Response = EditMessageResult;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("editMessageCaption", &self)
    }
}
