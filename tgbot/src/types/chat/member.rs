use crate::types::{
    chat::raw::{RawChatMember, RawChatMemberStatus},
    primitive::Integer,
    user::User,
};
use serde::de::{Deserialize, Deserializer, Error};

/// Information about one member of a chat
#[derive(Clone, Debug)]
pub enum ChatMember {
    /// Chat admin
    Administrator(ChatMemberAdministrator),
    /// Chat creator
    Creator(User),
    /// Kicked user
    Kicked(ChatMemberKicked),
    /// Left user
    Left(User),
    /// Chat member
    Member(User),
    /// Restricted user
    Restricted(ChatMemberRestricted),
}

impl ChatMember {
    /// Returns a user object
    pub fn user(&self) -> &User {
        use self::ChatMember::*;
        match self {
            Administrator(ref admin) => &admin.user,
            Creator(ref user) => user,
            Kicked(ref kicked) => &kicked.user,
            Left(ref user) => user,
            Member(ref user) => user,
            Restricted(ref restricted) => &restricted.user,
        }
    }
}

impl<'de> Deserialize<'de> for ChatMember {
    fn deserialize<D>(deserializer: D) -> Result<ChatMember, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawChatMember = Deserialize::deserialize(deserializer)?;
        macro_rules! required {
            ($name:ident) => {{
                match raw.$name {
                    Some(val) => val,
                    None => return Err(D::Error::missing_field(stringify!($name))),
                }
            }};
        };
        Ok(match raw.status {
            RawChatMemberStatus::Administrator => ChatMember::Administrator(ChatMemberAdministrator {
                user: raw.user,
                can_be_edited: required!(can_be_edited),
                can_change_info: required!(can_change_info),
                can_post_messages: required!(can_post_messages),
                can_edit_messages: required!(can_edit_messages),
                can_delete_messages: required!(can_delete_messages),
                can_invite_users: required!(can_invite_users),
                can_restrict_members: required!(can_restrict_members),
                can_pin_messages: required!(can_pin_messages),
                can_promote_members: required!(can_promote_members),
            }),
            RawChatMemberStatus::Creator => ChatMember::Creator(raw.user),
            RawChatMemberStatus::Kicked => ChatMember::Kicked(ChatMemberKicked {
                user: raw.user,
                until_date: required!(until_date),
            }),
            RawChatMemberStatus::Left => ChatMember::Left(raw.user),
            RawChatMemberStatus::Member => ChatMember::Member(raw.user),
            RawChatMemberStatus::Restricted => ChatMember::Restricted(ChatMemberRestricted {
                user: raw.user,
                until_date: required!(until_date),
                can_send_messages: required!(can_send_messages),
                can_send_media_messages: required!(can_send_media_messages),
                can_send_other_messages: required!(can_send_other_messages),
                can_add_web_page_previews: required!(can_add_web_page_previews),
                is_member: required!(is_member),
            }),
        })
    }
}

/// Chat admin
#[derive(Clone, Debug)]
pub struct ChatMemberAdministrator {
    /// Information about the user
    pub user: User,
    /// True, if the bot is allowed
    /// to edit administrator privileges of that user
    pub can_be_edited: bool,
    /// True, if the administrator can change
    /// the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the administrator can post
    /// in the channel, channels only
    pub can_post_messages: bool,
    /// True, if the administrator can edit messages
    /// of other users and can pin messages, channels only
    pub can_edit_messages: bool,
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// True, if the administrator can invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: bool,
    /// True, if the administrator can pin messages, supergroups only
    pub can_pin_messages: bool,
    /// True, if the administrator can
    /// add new administrators with a subset
    /// of his own privileges or
    /// demote administrators that he has promoted,
    /// directly or indirectly
    /// (promoted by administrators that were appointed by the user)
    pub can_promote_members: bool,
}

/// Kicked user
#[derive(Clone, Debug)]
pub struct ChatMemberKicked {
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user, unix time
    pub until_date: Integer,
}

/// Restricted user
#[derive(Clone, Debug)]
pub struct ChatMemberRestricted {
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user, unix time
    pub until_date: Integer,
    /// True, if the user can send
    /// text messages, contacts, locations and venues
    pub can_send_messages: bool,
    /// True, if the user can send
    /// audios, documents, photos, videos,
    /// video notes and voice notes, implies can_send_messages
    pub can_send_media_messages: bool,
    /// True, if the user can send
    /// animations, games, stickers
    /// and use inline bots, implies can_send_media_messages
    pub can_send_other_messages: bool,
    /// True, if user may add web page previews
    /// to his messages, implies can_send_media_messages
    pub can_add_web_page_previews: bool,
    /// True, if the user is a member
    /// of the chat at the moment of the request
    pub is_member: bool,
}
