use crate::types::message::Message;
use crate::types::primitive::Integer;
use crate::types::user::User;
use serde::de::{Deserialize, Deserializer, Error};

/// This object represents a chat
#[derive(Debug)]
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
                username: required!(username),
                title: required!(title),
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

/// This object represents a channel
#[derive(Debug)]
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

/// This object represents a group
#[derive(Debug)]
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

/// This object represents a private chat
#[derive(Debug)]
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

/// This object represents a supergroup
#[derive(Debug)]
pub struct SupergroupChat {
    /// Unique identifier for this chat
    pub id: Integer,
    /// Title
    pub title: String,
    /// Username of a supergroup
    pub username: String,
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

#[derive(Debug, Deserialize)]
struct RawChat {
    id: Integer,
    #[serde(rename = "type")]
    kind: RawChatKind,
    title: Option<String>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    all_members_are_administrators: Option<bool>,
    photo: Option<ChatPhoto>,
    description: Option<String>,
    invite_link: Option<String>,
    pinned_message: Option<Box<Message>>,
    sticker_set_name: Option<String>,
    can_set_sticker_set: Option<bool>,
}

#[derive(Debug, Deserialize)]
enum RawChatKind {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "supergroup")]
    Supergroup,
    #[serde(rename = "channel")]
    Channel,
}

/// This object contains information about one member of a chat.
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
pub enum ChatMemberStatus {
    /// User is admin in a chat
    #[serde(rename = "administrator")]
    Administrator,
    /// User has created a chat
    #[serde(rename = "creator")]
    Creator,
    /// User has kicked from a chat
    #[serde(rename = "kicked")]
    Kicked,
    /// User has left a chat
    #[serde(rename = "left")]
    Left,
    /// User is a member of chat
    #[serde(rename = "member")]
    Member,
    /// User is in restricted list
    #[serde(rename = "restricted")]
    Restricted,
}

/// This object represents a chat photo
#[derive(Debug, Deserialize)]
pub struct ChatPhoto {
    /// Unique file identifier of small (160x160) chat photo
    /// This file_id can be used only for photo download
    pub small_file_id: String,
    /// Unique file identifier of big (640x640) chat photo
    /// This file_id can be used only for photo download
    pub big_file_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_channel() {
        let input = r#"{
            "id": 1,
            "type": "channel",
            "title": "channeltitle",
            "username": "channelusername",
            "photo": {
                "small_file_id": "smallfileid",
                "big_file_id": "bigfileid"
            },
            "description": "channeldescription",
            "invite_link": "channelinvitelink"
        }"#;
        // TODO: test pinned message
        let chat: Chat = serde_json::from_str(input).unwrap();
        if let Chat::Channel(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, String::from("channeltitle"));
            assert_eq!(chat.username, Some(String::from("channelusername")));
            let photo = chat.photo.unwrap();
            assert_eq!(photo.small_file_id, String::from("smallfileid"));
            assert_eq!(photo.big_file_id, String::from("bigfileid"));
            assert_eq!(chat.description, Some(String::from("channeldescription")));
            assert_eq!(chat.invite_link, Some(String::from("channelinvitelink")));
        } else {
            panic!("Unexpected chat: {:?}", chat);
        }
        let input = r#"{
            "id": 1,
            "type": "channel",
            "title": "channeltitle"
        }"#;
        let chat: Chat = serde_json::from_str(input).unwrap();
        if let Chat::Channel(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, String::from("channeltitle"));
            assert_eq!(chat.username.is_none(), true);
            assert_eq!(chat.photo.is_none(), true);
            assert_eq!(chat.description.is_none(), true);
            assert_eq!(chat.invite_link.is_none(), true);
            assert_eq!(chat.pinned_message.is_none(), true);
        } else {
            panic!("Unexpected chat: {:?}", chat);
        }
    }

    #[test]
    fn test_deserialize_group() {
        let input = r#"{
            "id": 1,
            "type": "group",
            "title": "grouptitle",
            "all_members_are_administrators": true,
            "photo": {
                "small_file_id": "smallfileid",
                "big_file_id": "bigfileid"
            },
            "invite_link": "groupinvitelink"
        }"#;
        // TODO: test pinned message
        let chat: Chat = serde_json::from_str(input).unwrap();
        if let Chat::Group(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, String::from("grouptitle"));
            assert_eq!(chat.all_members_are_administrators, true);
            let photo = chat.photo.unwrap();
            assert_eq!(photo.small_file_id, String::from("smallfileid"));
            assert_eq!(photo.big_file_id, String::from("bigfileid"));
            assert_eq!(chat.invite_link, Some(String::from("groupinvitelink")));
        } else {
            panic!("Unexpected chat: {:?}", chat);
        }
        let input = r#"{
            "id": 1,
            "type": "group",
            "title": "grouptitle",
            "all_members_are_administrators": false
        }"#;
        let chat: Chat = serde_json::from_str(input).unwrap();
        if let Chat::Group(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, String::from("grouptitle"));
            assert_eq!(chat.all_members_are_administrators, false);
            assert_eq!(chat.photo.is_none(), true);
            assert_eq!(chat.invite_link.is_none(), true);
            assert_eq!(chat.pinned_message.is_none(), true);
        } else {
            panic!("Unexpected chat: {:?}", chat);
        }
    }

    #[test]
    fn test_deserialize_private() {
        let input = r#"{
            "id": 1,
            "type": "private",
            "username": "testusername",
            "first_name": "testfirstname",
            "last_name": "testlastname",
            "photo": {
                "small_file_id": "smallfileid",
                "big_file_id": "bigfileid"
            }
        }"#;
        let chat: Chat = serde_json::from_str(input).unwrap();
        if let Chat::Private(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.username, Some(String::from("testusername")));
            assert_eq!(chat.first_name, String::from("testfirstname"));
            assert_eq!(chat.last_name, Some(String::from("testlastname")));
            let photo = chat.photo.unwrap();
            assert_eq!(photo.small_file_id, "smallfileid");
            assert_eq!(photo.big_file_id, "bigfileid");
        } else {
            panic!("Unexpected chat: {:?}", chat)
        }

        let input = r#"{
            "id": 1,
            "type": "private",
            "first_name": "testfirstname"
        }"#;
        let chat: Chat = serde_json::from_str(input).unwrap();
        if let Chat::Private(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.username.is_none(), true);
            assert_eq!(chat.first_name, String::from("testfirstname"));
            assert_eq!(chat.last_name.is_none(), true);
            assert_eq!(chat.photo.is_none(), true);
        } else {
            panic!("Unexpected chat: {:?}", chat)
        }
    }

    #[test]
    fn test_deserialize_supergroup() {
        let input = r#"{
            "id": 1,
            "type": "supergroup",
            "title": "supergrouptitle",
            "username": "supergroupusername",
            "photo": {
                "small_file_id": "smallfileid",
                "big_file_id": "bigfileid"
            },
            "description": "supergroupdescription",
            "invite_link": "supergroupinvitelink",
            "sticker_set_name": "supergroupstickersetname",
            "can_set_sticker_set": true
        }"#;
        // TODO: test pinned message
        let chat: Chat = serde_json::from_str(input).unwrap();
        if let Chat::Supergroup(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, String::from("supergrouptitle"));
            assert_eq!(chat.username, String::from("supergroupusername"));
            let photo = chat.photo.unwrap();
            assert_eq!(photo.small_file_id, "smallfileid");
            assert_eq!(photo.big_file_id, "bigfileid");
            assert_eq!(
                chat.description,
                Some(String::from("supergroupdescription"))
            );
            assert_eq!(chat.invite_link, Some(String::from("supergroupinvitelink")));
            assert_eq!(
                chat.sticker_set_name,
                Some(String::from("supergroupstickersetname"))
            );
            assert_eq!(chat.can_set_sticker_set, Some(true));
        } else {
            panic!("Unexpected chat: {:?}", chat)
        }
        let input = r#"{
            "id": 1,
            "type": "supergroup",
            "title": "supergrouptitle",
            "username": "supergroupusername"
        }"#;
        let chat: Chat = serde_json::from_str(input).unwrap();
        if let Chat::Supergroup(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, String::from("supergrouptitle"));
            assert_eq!(chat.username, String::from("supergroupusername"));
            assert_eq!(chat.photo.is_none(), true);
            assert_eq!(chat.description.is_none(), true);
            assert_eq!(chat.invite_link.is_none(), true);
            assert_eq!(chat.sticker_set_name.is_none(), true);
            assert_eq!(chat.can_set_sticker_set.is_none(), true);
            assert_eq!(chat.pinned_message.is_none(), true);
        } else {
            panic!("Unexpected chat: {:?}", chat)
        }
    }
}
