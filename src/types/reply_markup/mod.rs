use crate::macros::impl_enum_from;
use serde::Serialize;

mod force_reply;
mod inline_keyboard;
mod reply_keyboard;
#[cfg(test)]
mod tests;

pub use self::force_reply::*;
pub use self::inline_keyboard::*;
pub use self::reply_keyboard::*;

/// Reply markup
#[derive(Clone, Debug, Serialize)]
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

impl_enum_from!(
    ReplyMarkup {
        ForceReply(ForceReply),
        InlineKeyboardMarkup(InlineKeyboardMarkup),
        InlineKeyboardMarkup(Vec<Vec<InlineKeyboardButton>>),
        ReplyKeyboardMarkup(ReplyKeyboardMarkup),
        ReplyKeyboardMarkup(Vec<Vec<KeyboardButton>>),
        ReplyKeyboardRemove(ReplyKeyboardRemove)
    }
);
