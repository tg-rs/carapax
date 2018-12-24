use crate::types::photo_size::PhotoSize;
use crate::types::primitive::Integer;
use serde::Deserialize;

/// Telegram user or bot
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd)]
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

/// User's profile pictures
#[derive(Clone, Debug, Deserialize)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: Integer,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<Vec<PhotoSize>>,
}
