use crate::types::{ChatId, InlineKeyboardMarkup, Integer, Message, Poll, ReplyMarkup};
use crate::{methods::Method, request::RequestBuilder};
use failure::Error;
use serde::Serialize;

/// Use this method to send a native poll
///
/// A native poll can't be sent to a private chat
/// On success, the sent Message is returned
#[derive(Clone, Debug, Serialize)]
pub struct SendPoll {
    chat_id: ChatId,
    question: String,
    options: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl SendPoll {
    /// Creates a new SendPoll
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * question - Poll question, 1-255 characters
    pub fn new<C, Q>(chat_id: C, question: Q) -> Self
    where
        C: Into<ChatId>,
        Q: Into<String>,
    {
        Self {
            chat_id: chat_id.into(),
            question: question.into(),
            options: vec![],
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Adds an answer option (1-100 characters)
    pub fn option<O>(mut self, option: O) -> Self
    where
        O: Into<String>,
    {
        self.options.push(option.into());
        self
    }

    /// Sends the message silently
    ///
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
    pub fn reply_markup<R>(mut self, reply_markup: R) -> Self
    where
        R: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendPoll {
    type Response = Message;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("sendPoll", &self)
    }
}

/// Use this method to stop a poll which was sent by the bot
///
/// On success, the stopped Poll with the final results is returned
#[derive(Clone, Debug, Serialize)]
pub struct StopPoll {
    chat_id: ChatId,
    message_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl StopPoll {
    /// Creates a new StopPoll
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the original message with the poll
    pub fn new<C>(chat_id: C, message_id: Integer) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            message_id,
            reply_markup: None,
        }
    }

    /// A JSON-serialized object for a new message inline keyboard
    pub fn reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
}

impl Method for StopPoll {
    type Response = Poll;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("stopPoll", &self)
    }
}
