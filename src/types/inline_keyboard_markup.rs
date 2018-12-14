use crate::types::inline_keyboard_button::InlineKeyboardButton;

/// This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Debug)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of InlineKeyboardButton objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
