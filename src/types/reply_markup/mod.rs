use serde::Serialize;

mod force_reply;
mod inline_keyboard;
mod reply_keyboard;
#[cfg(test)]
mod tests;

pub use self::{force_reply::*, inline_keyboard::*, reply_keyboard::*};

/// Reply markup
#[derive(Clone, Debug, From, Serialize)]
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
