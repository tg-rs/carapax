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

/// This object contains information about one member of a chat
#[derive(Debug)]
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
            RawChatMemberStatus::Administrator => {
                ChatMember::Administrator(ChatMemberAdministrator {
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
                })
            }
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
            }),
        })
    }
}

/// Chat admin
#[derive(Debug)]
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
#[derive(Debug)]
pub struct ChatMemberKicked {
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user, unix time
    pub until_date: Integer,
}

/// Restricted user
#[derive(Debug)]
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
}

/// This object contains information about one member of a chat.
#[derive(Debug, Deserialize)]
struct RawChatMember {
    user: User,
    status: RawChatMemberStatus,
    until_date: Option<Integer>,
    can_be_edited: Option<bool>,
    can_change_info: Option<bool>,
    can_post_messages: Option<bool>,
    can_edit_messages: Option<bool>,
    can_delete_messages: Option<bool>,
    can_invite_users: Option<bool>,
    can_restrict_members: Option<bool>,
    can_pin_messages: Option<bool>,
    can_promote_members: Option<bool>,
    can_send_messages: Option<bool>,
    can_send_media_messages: Option<bool>,
    can_send_other_messages: Option<bool>,
    can_add_web_page_previews: Option<bool>,
}

#[derive(Debug, Deserialize)]
enum RawChatMemberStatus {
    #[serde(rename = "administrator")]
    Administrator,
    #[serde(rename = "creator")]
    Creator,
    #[serde(rename = "kicked")]
    Kicked,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "member")]
    Member,
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
            assert_eq!(chat.username, Some(String::from("supergroupusername")));
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
            assert_eq!(chat.username, Some(String::from("supergroupusername")));
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

    #[test]
    fn test_deserialize_chat_member_admin() {
        let admin = r#"{
            "status": "administrator",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "firstname",
                "last_name": "lastname",
                "username": "username",
                "language_code": "RU"
            },
            "can_be_edited": true,
            "can_change_info": false,
            "can_post_messages": true,
            "can_edit_messages": false,
            "can_delete_messages": true,
            "can_invite_users": false,
            "can_restrict_members": true,
            "can_pin_messages": false,
            "can_promote_members": true
        }"#;
        let admin: ChatMember = serde_json::from_str(admin).unwrap();
        if let ChatMember::Administrator(ref admin) = admin {
            assert_eq!(admin.user.id, 1);
            assert_eq!(admin.user.is_bot, false);
            assert_eq!(admin.user.first_name, String::from("firstname"));
            assert_eq!(admin.user.last_name, Some(String::from("lastname")));
            assert_eq!(admin.user.username, Some(String::from("username")));
            assert_eq!(admin.user.language_code, Some(String::from("RU")));
            assert_eq!(admin.can_be_edited, true);
            assert_eq!(admin.can_change_info, false);
            assert_eq!(admin.can_post_messages, true);
            assert_eq!(admin.can_edit_messages, false);
            assert_eq!(admin.can_delete_messages, true);
            assert_eq!(admin.can_invite_users, false);
            assert_eq!(admin.can_restrict_members, true);
            assert_eq!(admin.can_pin_messages, false);
            assert_eq!(admin.can_promote_members, true);
        } else {
            panic!("Unexpected chat member: {:?}", admin);
        }
    }

    #[test]
    fn test_deserialize_chat_member_creator() {
        let creator = r#"{
            "status": "creator",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "firstname"
            }
        }"#;
        let creator: ChatMember = serde_json::from_str(creator).unwrap();
        if let ChatMember::Creator(ref creator) = creator {
            assert_eq!(creator.id, 1);
            assert_eq!(creator.is_bot, false);
            assert_eq!(creator.first_name, String::from("firstname"));
            assert_eq!(creator.last_name, None);
            assert_eq!(creator.username, None);
            assert_eq!(creator.language_code, None);
        } else {
            panic!("Unexpected chat member: {:?}", creator);
        }
    }

    #[test]
    fn test_deserialize_chat_member_kicked() {
        let kicked = r#"{
            "status": "kicked",
            "user": {
                "id": 1,
                "is_bot": true,
                "first_name": "firstname",
                "last_name": "lastname",
                "username": "username"
            },
            "until_date": 0
        }"#;
        let kicked: ChatMember = serde_json::from_str(kicked).unwrap();
        if let ChatMember::Kicked(ref kicked) = kicked {
            assert_eq!(kicked.user.id, 1);
            assert_eq!(kicked.user.is_bot, true);
            assert_eq!(kicked.user.first_name, String::from("firstname"));
            assert_eq!(kicked.user.last_name, Some(String::from("lastname")));
            assert_eq!(kicked.user.username, Some(String::from("username")));
            assert_eq!(kicked.user.language_code, None);
            assert_eq!(kicked.until_date, 0);
        } else {
            panic!("Unexpected chat member: {:?}", kicked);
        }
    }

    #[test]
    fn test_deserialize_chat_member_left() {
        let left = r#"{
            "status": "left",
            "user": {
                "id": 1,
                "is_bot": true,
                "first_name": "firstname"
            }
        }"#;
        let left: ChatMember = serde_json::from_str(left).unwrap();
        if let ChatMember::Left(ref left) = left {
            assert_eq!(left.id, 1);
            assert_eq!(left.is_bot, true);
            assert_eq!(left.first_name, String::from("firstname"));
            assert_eq!(left.last_name, None);
            assert_eq!(left.username, None);
            assert_eq!(left.language_code, None);
        } else {
            panic!("Unexpected chat member: {:?}", left);
        }
    }

    #[test]
    fn test_deserialize_chat_member_plain() {
        let plain = r#"{
            "status": "member",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "firstname"
            }
        }"#;
        let plain: ChatMember = serde_json::from_str(plain).unwrap();
        if let ChatMember::Member(ref plain) = plain {
            assert_eq!(plain.id, 1);
            assert_eq!(plain.is_bot, false);
            assert_eq!(plain.first_name, String::from("firstname"));
            assert_eq!(plain.last_name, None);
            assert_eq!(plain.username, None);
            assert_eq!(plain.language_code, None);
        } else {
            panic!("Unexpected chat member: {:?}", plain);
        }
    }

    #[test]
    fn test_deserialize_chat_member_restricted() {
        let restricted = r#"{
            "status": "restricted",
            "user": {
                "id": 1,
                "is_bot": true,
                "first_name": "firstname"
            },
            "until_date": 0,
            "can_send_messages": true,
            "can_send_media_messages": false,
            "can_send_other_messages": true,
            "can_add_web_page_previews": false
        }"#;
        let restricted: ChatMember = serde_json::from_str(restricted).unwrap();
        if let ChatMember::Restricted(ref restricted) = restricted {
            assert_eq!(restricted.user.id, 1);
            assert_eq!(restricted.user.is_bot, true);
            assert_eq!(restricted.user.first_name, String::from("firstname"));
            assert_eq!(restricted.user.last_name, None);
            assert_eq!(restricted.user.username, None);
            assert_eq!(restricted.user.language_code, None);
            assert_eq!(restricted.until_date, 0);
            assert_eq!(restricted.can_send_messages, true);
            assert_eq!(restricted.can_send_media_messages, false);
            assert_eq!(restricted.can_send_other_messages, true);
            assert_eq!(restricted.can_add_web_page_previews, false);
        } else {
            panic!("Unexpected chat member: {:?}", restricted);
        }
    }
}
