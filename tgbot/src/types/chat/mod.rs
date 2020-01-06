use crate::types::{
    chat::raw::{RawChat, RawChatKind},
    message::Message,
    primitive::Integer,
};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

mod member;
mod permissions;
mod photo;
mod raw;

pub use self::{
    member::{ChatMember, ChatMemberAdministrator, ChatMemberKicked, ChatMemberRestricted},
    permissions::ChatPermissions,
    photo::ChatPhoto,
};

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
                permissions: raw_chat.permissions,
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
                permissions: raw_chat.permissions,
                slow_mode_delay: raw_chat.slow_mode_delay,
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
    ///
    /// Returned only in getChat
    pub photo: Option<ChatPhoto>,
    /// Description of a channel
    ///
    /// Returned only in getChat
    pub description: Option<String>,
    /// Invite link
    ///
    /// Returned only in getChat
    pub invite_link: Option<String>,
    /// Pinned message
    ///
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
    ///
    /// The field is still returned in the object for backward compatibility,
    /// but new bots should use the permissions field instead
    pub all_members_are_administrators: bool,
    /// Chat photo
    ///
    /// Returned only in getChat
    pub photo: Option<ChatPhoto>,
    /// Invite link
    ///
    /// Returned only in getChat
    pub invite_link: Option<String>,
    /// Pinned message
    /// Returned only in getChat
    pub pinned_message: Option<Box<Message>>,
    /// Default chat member permissions, for groups and supergroups
    ///
    /// Returned only in getChat
    pub permissions: Option<ChatPermissions>,
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
    ///
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
    ///
    /// Returned only in getChat
    pub photo: Option<ChatPhoto>,
    /// Description of a supergroup
    ///
    /// Returned only in getChat
    pub description: Option<String>,
    /// Invite link
    ///
    /// Returned only in getChat
    pub invite_link: Option<String>,
    /// Pinned message
    ///
    /// Returned only in getChat
    pub pinned_message: Option<Box<Message>>,
    /// For supergroups, name of group sticker set
    ///
    /// Returned only in getChat
    pub sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set
    ///
    /// Returned only in getChat
    pub can_set_sticker_set: Option<bool>,
    /// Default chat member permissions, for groups and supergroups
    ///
    /// Returned only in getChat
    pub permissions: Option<ChatPermissions>,
    /// The minimum allowed delay between consecutive messages sent by each unpriviledged user
    ///
    /// Returned only in getChat
    pub slow_mode_delay: Option<Integer>,
}

/// Chat ID or username
#[derive(Clone, Debug)]
pub enum ChatId {
    /// @username of a chat
    Username(String),
    /// ID of a chat
    Id(Integer),
}

impl ToString for ChatId {
    fn to_string(&self) -> String {
        match self {
            ChatId::Username(username) => username.clone(),
            ChatId::Id(chat_id) => chat_id.to_string(),
        }
    }
}

impl Serialize for ChatId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ChatId::Username(username) => serializer.serialize_str(username),
            ChatId::Id(id) => serializer.serialize_i64(*id),
        }
    }
}

impl From<&str> for ChatId {
    fn from(username: &str) -> ChatId {
        ChatId::Username(String::from(username))
    }
}

impl From<String> for ChatId {
    fn from(username: String) -> ChatId {
        ChatId::Username(username)
    }
}

impl From<Integer> for ChatId {
    fn from(id: Integer) -> ChatId {
        ChatId::Id(id)
    }
}

/// Type of action to tell the user that some is happening on the bot's side
#[derive(Clone, Copy, Debug, Serialize)]
pub enum ChatAction {
    /// For location data
    #[serde(rename = "find_location")]
    FindLocation,
    /// For audio files
    #[serde(rename = "record_audio")]
    RecordAudio,
    /// For videos
    #[serde(rename = "record_video")]
    RecordVideo,
    /// For video notes
    #[serde(rename = "record_video_note")]
    RecordVideoNote,
    /// For text messages
    #[serde(rename = "typing")]
    Typing,
    /// For audio files
    #[serde(rename = "upload_audio")]
    UploadAudio,
    /// For general files
    #[serde(rename = "upload_document")]
    UploadDocument,
    /// For photos
    #[serde(rename = "upload_photo")]
    UploadPhoto,
    /// For videos
    #[serde(rename = "upload_video")]
    UploadVideo,
    /// For video notes
    #[serde(rename = "upload_video_note")]
    UploadVideoNote,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_channel() {
        let chat: Chat = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": "channel",
            "title": "channeltitle",
            "username": "channelusername",
            "photo": {
                "small_file_id": "smallfileid",
                "small_file_unique_id": "smallfileuniqueid",
                "big_file_id": "bigfileid",
                "big_file_unique_id": "bigfileuniqueid",
            },
            "description": "channeldescription",
            "invite_link": "channelinvitelink",
            "pinned_message": {
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": 1,
                    "type": "channel",
                    "title": "channeltitle"
                },
                "text": "test"
            }
        }))
        .unwrap();
        if let Chat::Channel(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, "channeltitle");
            assert_eq!(chat.username.unwrap(), "channelusername");
            let photo = chat.photo.unwrap();
            assert_eq!(photo.small_file_id, "smallfileid");
            assert_eq!(photo.small_file_unique_id, "smallfileuniqueid");
            assert_eq!(photo.big_file_id, "bigfileid");
            assert_eq!(photo.big_file_unique_id, "bigfileuniqueid");
            assert_eq!(chat.description.unwrap(), "channeldescription");
            assert_eq!(chat.invite_link.unwrap(), "channelinvitelink");
            assert!(chat.pinned_message.is_some());
        } else {
            panic!("Unexpected chat: {:?}", chat);
        }

        let chat: Chat = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": "channel",
            "title": "channeltitle"
        }))
        .unwrap();
        if let Chat::Channel(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, "channeltitle");
            assert!(chat.username.is_none());
            assert!(chat.photo.is_none());
            assert!(chat.description.is_none());
            assert!(chat.invite_link.is_none());
            assert!(chat.pinned_message.is_none());
        } else {
            panic!("Unexpected chat: {:?}", chat);
        }
    }

    #[test]
    fn deserialize_group() {
        let chat: Chat = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": "group",
            "title": "grouptitle",
            "all_members_are_administrators": true,
            "photo": {
                "small_file_id": "smallfileid",
                "small_file_unique_id": "smallfileuniqueid",
                "big_file_id": "bigfileid",
                "big_file_unique_id": "bigfileuniqueid",
            },
            "invite_link": "groupinvitelink",
            "pinned_message": {
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": 1,
                    "type": "group",
                    "title": "grouptitle",
                    "all_members_are_administrators": true
                },
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "user"
                },
                "text": "test"
            },
            "permissions": {"can_send_messages": true}
        }))
        .unwrap();
        if let Chat::Group(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, "grouptitle");
            assert!(chat.all_members_are_administrators);
            let photo = chat.photo.unwrap();
            assert_eq!(photo.small_file_id, "smallfileid");
            assert_eq!(photo.small_file_unique_id, "smallfileuniqueid");
            assert_eq!(photo.big_file_id, "bigfileid");
            assert_eq!(photo.big_file_unique_id, "bigfileuniqueid");
            assert_eq!(chat.invite_link.unwrap(), "groupinvitelink");
            let permissions = chat.permissions.unwrap();
            assert!(permissions.can_send_messages.unwrap());
            assert!(chat.pinned_message.is_some());
        } else {
            panic!("Unexpected chat: {:?}", chat);
        }

        let chat: Chat = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": "group",
            "title": "grouptitle",
            "all_members_are_administrators": false
        }))
        .unwrap();
        if let Chat::Group(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, "grouptitle");
            assert!(!chat.all_members_are_administrators);
            assert!(chat.photo.is_none());
            assert!(chat.invite_link.is_none());
            assert!(chat.pinned_message.is_none());
            assert!(chat.permissions.is_none());
        } else {
            panic!("Unexpected chat: {:?}", chat);
        }
    }

    #[test]
    fn deserialize_private() {
        let chat: Chat = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": "private",
            "username": "testusername",
            "first_name": "testfirstname",
            "last_name": "testlastname",
            "photo": {
                "small_file_id": "smallfileid",
                "small_file_unique_id": "smallfileuniqueid",
                "big_file_id": "bigfileid",
                "big_file_unique_id": "bigfileuniqueid",
            }
        }))
        .unwrap();
        if let Chat::Private(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.username.unwrap(), "testusername");
            assert_eq!(chat.first_name, "testfirstname");
            assert_eq!(chat.last_name.unwrap(), "testlastname");
            let photo = chat.photo.unwrap();
            assert_eq!(photo.small_file_id, "smallfileid");
            assert_eq!(photo.small_file_unique_id, "smallfileuniqueid");
            assert_eq!(photo.big_file_id, "bigfileid");
            assert_eq!(photo.big_file_unique_id, "bigfileuniqueid");
        } else {
            panic!("Unexpected chat: {:?}", chat)
        }

        let chat: Chat = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": "private",
            "first_name": "testfirstname"
        }))
        .unwrap();
        if let Chat::Private(chat) = chat {
            assert_eq!(chat.id, 1);
            assert!(chat.username.is_none());
            assert_eq!(chat.first_name, "testfirstname");
            assert!(chat.last_name.is_none());
            assert!(chat.photo.is_none());
        } else {
            panic!("Unexpected chat: {:?}", chat)
        }
    }

    #[test]
    fn deserialize_supergroup_full() {
        let chat: Chat = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": "supergroup",
            "title": "supergrouptitle",
            "username": "supergroupusername",
            "photo": {
                "small_file_id": "smallfileid",
                "small_file_unique_id": "smallfileuniqueid",
                "big_file_id": "bigfileid",
                "big_file_unique_id": "bigfileuniqueid",
            },
            "description": "supergroupdescription",
            "invite_link": "supergroupinvitelink",
            "sticker_set_name": "supergroupstickersetname",
            "can_set_sticker_set": true,
            "slow_mode_delay": 10,
            "permissions": {
                "can_send_messages": true
            },
            "pinned_message": {
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": 1,
                    "type": "supergroup",
                    "title": "supergrouptitle",
                    "username": "supergroupusername"
                },
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "user"
                },
                "text": "test"
            }
        }))
        .unwrap();
        if let Chat::Supergroup(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, "supergrouptitle");
            assert_eq!(chat.username.unwrap(), "supergroupusername");
            let photo = chat.photo.unwrap();
            assert_eq!(photo.small_file_id, "smallfileid");
            assert_eq!(photo.small_file_unique_id, "smallfileuniqueid");
            assert_eq!(photo.big_file_id, "bigfileid");
            assert_eq!(photo.big_file_unique_id, "bigfileuniqueid");
            assert_eq!(chat.description.unwrap(), "supergroupdescription");
            assert_eq!(chat.invite_link.unwrap(), "supergroupinvitelink");
            assert_eq!(chat.sticker_set_name.unwrap(), "supergroupstickersetname");
            assert_eq!(chat.slow_mode_delay.unwrap(), 10);
            assert!(chat.can_set_sticker_set.unwrap());
            assert!(chat.pinned_message.is_some());
            let permissions = chat.permissions.unwrap();
            assert!(permissions.can_send_messages.unwrap());
        } else {
            panic!("Unexpected chat: {:?}", chat)
        }
    }

    #[test]
    fn deserialize_supergroup_partial() {
        let chat: Chat = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": "supergroup",
            "title": "supergrouptitle",
            "username": "supergroupusername"
        }))
        .unwrap();
        if let Chat::Supergroup(chat) = chat {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, "supergrouptitle");
            assert_eq!(chat.username.unwrap(), "supergroupusername");
            assert!(chat.photo.is_none());
            assert!(chat.description.is_none());
            assert!(chat.invite_link.is_none());
            assert!(chat.sticker_set_name.is_none());
            assert!(chat.can_set_sticker_set.is_none());
            assert!(chat.pinned_message.is_none());
            assert!(chat.permissions.is_none());
        } else {
            panic!("Unexpected chat: {:?}", chat)
        }
    }

    #[test]
    fn chat_id() {
        let chat_id = ChatId::from(1);
        if let ChatId::Id(chat_id) = chat_id {
            assert_eq!(chat_id, 1);
        } else {
            panic!("Unexpected chat id: {:?}", chat_id);
        }
        assert_eq!(serde_json::to_string(&chat_id).unwrap(), r#"1"#);
        assert_eq!(chat_id.to_string(), "1");

        let chat_id = ChatId::from("username");
        if let ChatId::Username(ref username) = chat_id {
            assert_eq!(username, "username");
        } else {
            panic!("Unexpected chat id: {:?}", chat_id);
        }
        assert_eq!(serde_json::to_string(&chat_id).unwrap(), r#""username""#);
        assert_eq!(chat_id.to_string(), "username");

        let chat_id = ChatId::from(String::from("username"));
        if let ChatId::Username(ref username) = chat_id {
            assert_eq!(username, "username");
        } else {
            panic!("Unexpected chat id: {:?}", chat_id);
        }
        assert_eq!(serde_json::to_string(&chat_id).unwrap(), r#""username""#);
        assert_eq!(chat_id.to_string(), "username");
    }

    #[test]
    fn chat_action() {
        assert_eq!(
            serde_json::to_string(&ChatAction::FindLocation).unwrap(),
            r#""find_location""#
        );
        assert_eq!(
            serde_json::to_string(&ChatAction::RecordAudio).unwrap(),
            r#""record_audio""#
        );
        assert_eq!(
            serde_json::to_string(&ChatAction::RecordVideo).unwrap(),
            r#""record_video""#
        );
        assert_eq!(
            serde_json::to_string(&ChatAction::RecordVideoNote).unwrap(),
            r#""record_video_note""#
        );
        assert_eq!(serde_json::to_string(&ChatAction::Typing).unwrap(), r#""typing""#);
        assert_eq!(
            serde_json::to_string(&ChatAction::UploadAudio).unwrap(),
            r#""upload_audio""#
        );
        assert_eq!(
            serde_json::to_string(&ChatAction::UploadDocument).unwrap(),
            r#""upload_document""#
        );
        assert_eq!(
            serde_json::to_string(&ChatAction::UploadPhoto).unwrap(),
            r#""upload_photo""#
        );
        assert_eq!(
            serde_json::to_string(&ChatAction::UploadVideo).unwrap(),
            r#""upload_video""#
        );
        assert_eq!(
            serde_json::to_string(&ChatAction::UploadVideoNote).unwrap(),
            r#""upload_video_note""#
        );
    }
}
