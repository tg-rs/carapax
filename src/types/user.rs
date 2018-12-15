use crate::types::primitive::Integer;

/// This object represents a Telegram user or bot.
#[derive(Debug)]
pub struct User {
    /// Unique identifier for this user or bot
    pub id: Integer,
    /// True, if this user is a bot
    pub is_bot: bool,
    /// User‘s or bot’s first name
    pub first_name: String,
    /// User‘s or bot’s last name
    pub last_name: Option<String>,
    /// User‘s or bot’s username
    pub username: Option<String>,
    /// IETF language tag of the user's language
    pub language_code: Option<String>,
}
