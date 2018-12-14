use crate::types::user::User;

/// This object represents one special entity in a text message.
/// For example, hashtags, usernames, URLs, etc.
#[derive(Debug)]
pub struct MessageEntity {
    /// Type of the entity.
    /// Can be mention (@username),
    /// hashtag, cashtag, bot_command, url, email, phone_number,
    /// bold (bold text), italic (italic text), code (monowidth string),
    /// pre (monowidth block),
    /// text_link (for clickable text URLs),
    /// text_mention (for users without usernames)
    pub kind: String, // TODO: enum, type
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i64,
    /// Length of the entity in UTF-16 code units
    pub length: i64,
    /// For “text_link” only, url that will be opened after user taps on the text
    pub url: Option<String>,
    /// For “text_mention” only, the mentioned user
    pub user: Option<User>,
}
