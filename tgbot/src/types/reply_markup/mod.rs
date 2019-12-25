use serde::Serialize;
use serde_json::Error as JsonError;
use std::{error::Error as StdError, fmt};

mod force_reply;
mod inline_keyboard;
mod reply_keyboard;

pub use self::{force_reply::*, inline_keyboard::*, reply_keyboard::*};

/// Reply markup
#[derive(Clone, Debug, derive_more::From, Serialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    /// Force reply
    ForceReply(ForceReply),
    /// Inline keyboard
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    /// A custom keyboard with reply options
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    /// Remove keyboard
    ReplyKeyboardRemove(ReplyKeyboardRemove),
}

impl ReplyMarkup {
    pub(crate) fn serialize(&self) -> Result<String, ReplyMarkupError> {
        serde_json::to_string(self).map_err(ReplyMarkupError::Serialize)
    }
}

impl From<Vec<Vec<InlineKeyboardButton>>> for ReplyMarkup {
    fn from(markup: Vec<Vec<InlineKeyboardButton>>) -> ReplyMarkup {
        ReplyMarkup::InlineKeyboardMarkup(markup.into())
    }
}

impl From<Vec<Vec<KeyboardButton>>> for ReplyMarkup {
    fn from(markup: Vec<Vec<KeyboardButton>>) -> ReplyMarkup {
        ReplyMarkup::ReplyKeyboardMarkup(markup.into())
    }
}

/// An error occurred with reply markup
#[derive(Debug)]
pub enum ReplyMarkupError {
    /// Can not serialize markup
    Serialize(JsonError),
}

impl StdError for ReplyMarkupError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            ReplyMarkupError::Serialize(err) => Some(err),
        }
    }
}

impl fmt::Display for ReplyMarkupError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReplyMarkupError::Serialize(err) => write!(out, "can not serialize reply markup: {}", err),
        }
    }
}
