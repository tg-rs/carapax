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

macro_rules! impl_reply_markup_from {
    ($to:ident($from:ident)) => {
        impl From<$from> for ReplyMarkup {
            fn from(obj: $from) -> ReplyMarkup {
                ReplyMarkup::$to(obj)
            }
        }
    };
}

impl_reply_markup_from!(ForceReply(ForceReply));
impl_reply_markup_from!(InlineKeyboardMarkup(InlineKeyboardMarkup));
impl_reply_markup_from!(ReplyKeyboardMarkup(ReplyKeyboardMarkup));
impl_reply_markup_from!(ReplyKeyboardRemove(ReplyKeyboardRemove));
