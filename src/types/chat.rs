use crate::types::message::Message;
use crate::types::primitive::Integer;
use crate::types::user::User;

/// This object represents a chat.
#[derive(Debug, Deserialize, Serialize)]
pub struct Chat {
    /// Unique identifier for this chat.
    pub id: Integer,
    /// Type of chat
    pub kind: ChatType, // TODO: rename to type
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

/// Type of chat
/// Can be either “private”, “group”, “supergroup” or “channel”
#[derive(Debug, Deserialize, Serialize)]
pub enum ChatType {
    /// Private chat
    Private,
    /// Group chat
    Group,
    /// Supergroup chat
    Supergroup,
    /// Channel
    Channel,
}

/// This object contains information about one member of a chat.
#[derive(Debug, Deserialize, Serialize)]
pub struct ChatMember {
    /// Information about the user
    pub user: User,
    /// The member's status in the chat.
    pub status: ChatMemberStatus,
    /// Restricted and kicked only.
    /// Date when restrictions will be lifted for this user, unix time
    pub until_date: Option<Integer>,
    /// Administrators only.
    /// True, if the bot is allowed
    /// to edit administrator privileges of that user
    pub can_be_edited: Option<bool>,
    /// Administrators only.
    /// True, if the administrator can change
    /// the chat title, photo and other settings
    pub can_change_info: Option<bool>,
    /// Administrators only.
    /// True, if the administrator can post in the channel, channels only
    pub can_post_messages: Option<bool>,
    /// Administrators only.
    /// True, if the administrator can edit messages
    /// of other users and can pin messages, channels only
    pub can_edit_messages: Option<bool>,
    /// Administrators only.
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: Option<bool>,
    /// Administrators only.
    /// True, if the administrator can invite new users to the chat
    pub can_invite_users: Option<bool>,
    /// Administrators only.
    /// True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: Option<bool>,
    /// Administrators only.
    /// True, if the administrator can pin messages, supergroups only
    pub can_pin_messages: Option<bool>,
    /// Administrators only.
    /// True, if the administrator can
    /// add new administrators with a subset
    /// of his own privileges or
    /// demote administrators that he has promoted,
    /// directly or indirectly
    /// (promoted by administrators that were appointed by the user)
    pub can_promote_members: Option<bool>,
    /// Restricted only.
    /// True, if the user can send
    /// text messages, contacts, locations and venues
    pub can_send_messages: Option<bool>,
    /// Restricted only.
    /// True, if the user can send
    /// audios, documents, photos, videos,
    /// video notes and voice notes, implies can_send_messages
    pub can_send_media_messages: Option<bool>,
    /// Restricted only.
    /// True, if the user can send
    /// animations, games, stickers
    /// and use inline bots, implies can_send_media_messages
    pub can_send_other_messages: Option<bool>,
    /// Restricted only.
    /// True, if user may add web page previews
    /// to his messages, implies can_send_media_messages
    pub can_add_web_page_previews: Option<bool>,
}

/// Status of a chat member
/// Can be “creator”, “administrator”, “member”, “restricted”, “left” or “kicked”
#[derive(Debug, Deserialize, Serialize)]
pub enum ChatMemberStatus {
    /// User is admin in a chat
    Administrator,
    /// User has created a chat
    Creator,
    /// User has kicked from a chat
    Kicked,
    /// User has left a chat
    Left,
    /// User is a member of chat
    Member,
    /// User is in restricted list
    Restricted,
}

/// This object represents a chat photo.
#[derive(Debug, Deserialize, Serialize)]
pub struct ChatPhoto {
    /// Unique file identifier of small (160x160) chat photo.
    /// This file_id can be used only for photo download.
    pub small_file_id: String,
    /// Unique file identifier of big (640x640) chat photo.
    /// This file_id can be used only for photo download.
    pub big_file_id: String,
}
