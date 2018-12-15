use crate::types::chat_photo::ChatPhoto;
use crate::types::message::Message;
use crate::types::primitive::Integer;

/// This object represents a chat.
#[derive(Debug)]
pub struct Chat {
    /// Unique identifier for this chat.
    pub id: Integer,
    /// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    pub kind: String, // TODO: type, enum
    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// True if a group has ‘All Members Are Admins’ enabled.
    pub all_members_are_administrators: Option<bool>,
    /// Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
    /// Description, for supergroups and channel chats. Returned only in getChat.
    pub description: Option<String>,
    /// Chat invite link, for supergroups and channel chats. Returned only in getChat.
    pub invite_link: Option<String>,
    /// Pinned message, for supergroups and channel chats. Returned only in getChat.
    pub pinned_message: Option<Box<Message>>,
    /// For supergroups, name of group sticker set. Returned only in getChat.
    pub sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set. Returned only in getChat.
    pub can_set_sticker_set: Option<bool>,
}
