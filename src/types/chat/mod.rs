use crate::types::chat::raw::{RawChat, RawChatKind};
use crate::types::message::Message;
use crate::types::primitive::Integer;
use serde::de::{Deserialize, Deserializer, Error};

mod member;
mod photo;
mod raw;
#[cfg(test)]
mod tests;

pub use self::member::{
    ChatMember, ChatMemberAdministrator, ChatMemberKicked, ChatMemberRestricted,
};
pub use self::photo::ChatPhoto;

/// Chat
#[derive(Clone, Debug)]
pub enum Chat {
    /// Channel
    Channel(ChannelChat),
    /// Group
    Group(GroupChat),
    /// Private chat
    Private(PrivateChat),
    /// Supergroup
    Supergroup(SupergroupChat),
}

impl<'de> Deserialize<'de> for Chat {
    fn deserialize<D>(deserializer: D) -> Result<Chat, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_chat: RawChat = Deserialize::deserialize(deserializer)?;
        macro_rules! required {
            ($name:ident) => {{
                match raw_chat.$name {
                    Some(val) => val,
                    None => return Err(D::Error::missing_field(stringify!($name))),
                }
            }};
        };
        Ok(match raw_chat.kind {
            RawChatKind::Channel => Chat::Channel(ChannelChat {
                id: raw_chat.id,
                username: raw_chat.username,
                title: required!(title),
                description: raw_chat.description,
                photo: raw_chat.photo,
                pinned_message: raw_chat.pinned_message,
                invite_link: raw_chat.invite_link,
            }),
            RawChatKind::Group => Chat::Group(GroupChat {
                id: raw_chat.id,
                title: required!(title),
                all_members_are_administrators: required!(all_members_are_administrators),
                photo: raw_chat.photo,
                pinned_message: raw_chat.pinned_message,
                invite_link: raw_chat.invite_link,
            }),
            RawChatKind::Private => Chat::Private(PrivateChat {
                id: raw_chat.id,
                username: raw_chat.username,
                first_name: required!(first_name),
                last_name: raw_chat.last_name,
                photo: raw_chat.photo,
            }),
            RawChatKind::Supergroup => Chat::Supergroup(SupergroupChat {
                id: raw_chat.id,
                title: required!(title),
                username: raw_chat.username,
                description: raw_chat.description,
                photo: raw_chat.photo,
                pinned_message: raw_chat.pinned_message,
                invite_link: raw_chat.invite_link,
                sticker_set_name: raw_chat.sticker_set_name,
                can_set_sticker_set: raw_chat.can_set_sticker_set,
            }),
        })
    }
}

/// Channel chat
#[derive(Clone, Debug)]
pub struct ChannelChat {
    /// Unique identifier for this chat
    pub id: Integer,
    /// Title
    pub title: String,
    /// Username of a channel
    pub username: Option<String>,
    /// Chat photo
    /// Returned only in getChat
    pub photo: Option<ChatPhoto>,
    /// Description of a channel
    /// Returned only in getChat.
    pub description: Option<String>,
    /// Invite link
    /// Returned only in getChat
    pub invite_link: Option<String>,
    /// Pinned message
    /// Returned only in getChat
    pub pinned_message: Option<Box<Message>>,
}

/// Group chat
#[derive(Clone, Debug)]
pub struct GroupChat {
    /// Unique identifier for this chat
    pub id: Integer,
    /// Title
    pub title: String,
    /// True if a group has ‘All Members Are Admins’ enabled
    pub all_members_are_administrators: bool,
    /// Chat photo
    /// Returned only in getChat
    pub photo: Option<ChatPhoto>,
    /// Invite link
    /// Returned only in getChat
    pub invite_link: Option<String>,
    /// Pinned message
    /// Returned only in getChat
    pub pinned_message: Option<Box<Message>>,
}

/// Private chat
#[derive(Clone, Debug)]
pub struct PrivateChat {
    /// Unique identifier for this chat
    pub id: Integer,
    /// First name of the other party
    pub first_name: String,
    /// Last name of the other party
    pub last_name: Option<String>,
    /// Username of a chat
    pub username: Option<String>,
    /// Chat photo
    /// Returned only in getChat
    pub photo: Option<ChatPhoto>,
}

/// Supergroup chat
#[derive(Clone, Debug)]
pub struct SupergroupChat {
    /// Unique identifier for this chat
    pub id: Integer,
    /// Title
    pub title: String,
    /// Username of a supergroup
    pub username: Option<String>,
    /// Photo of a supergroup
    /// Returned only in getChat
    pub photo: Option<ChatPhoto>,
    /// Description of a supergroup
    /// Returned only in getChat
    pub description: Option<String>,
    /// Invite link
    /// Returned only in getChat
    pub invite_link: Option<String>,
    /// Pinned message
    /// Returned only in getChat
    pub pinned_message: Option<Box<Message>>,
    /// For supergroups, name of group sticker set
    /// Returned only in getChat
    pub sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set
    /// Returned only in getChat
    pub can_set_sticker_set: Option<bool>,
}
