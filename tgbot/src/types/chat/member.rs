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

    /// Whether a user is a member of the chat
    pub fn is_member(&self) -> bool {
        use self::ChatMember::*;
        match self {
            Administrator(_) | Creator(_) | Member(_) => true,
            Kicked(_) | Left(_) => false,
            Restricted(ref restricted) => restricted.is_member,
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
                can_post_messages: raw.can_post_messages,
                can_edit_messages: raw.can_edit_messages,
                can_delete_messages: required!(can_delete_messages),
                can_invite_users: required!(can_invite_users),
                can_restrict_members: required!(can_restrict_members),
                can_pin_messages: raw.can_pin_messages,
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
                can_change_info: required!(can_change_info),
                can_invite_users: required!(can_invite_users),
                can_pin_messages: raw.can_pin_messages,
                can_send_messages: required!(can_send_messages),
                can_send_polls: required!(can_send_polls),
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
    pub can_post_messages: Option<bool>,
    /// True, if the administrator can edit messages
    /// of other users and can pin messages, channels only
    pub can_edit_messages: Option<bool>,
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// True, if the administrator can invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: bool,
    /// True, if the administrator can pin messages, groups and supergroups only
    pub can_pin_messages: Option<bool>,
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
    /// True, if the user allowed to change
    /// the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the user allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the user allowed to pin messages, groups and supergroups only
    pub can_pin_messages: Option<bool>,
    /// True, if the user can send
    /// text messages, contacts, locations and venues
    pub can_send_messages: bool,
    /// True, if the user is allowed to send polls
    pub can_send_polls: bool,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_chat_member_admin() {
        let mut admin: ChatMember = serde_json::from_value(serde_json::json!({
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
        }))
        .unwrap();
        assert!(admin.is_member());
        assert_eq!(admin.user().id, 1);
        if let ChatMember::Administrator(ref mut admin) = admin {
            assert_eq!(admin.user.id, 1);
            assert_eq!(admin.user.is_bot, false);
            assert_eq!(admin.user.first_name, "firstname");
            assert_eq!(admin.user.last_name.take().unwrap(), "lastname");
            assert_eq!(admin.user.username.take().unwrap(), "username");
            assert_eq!(admin.user.language_code.take().unwrap(), "RU");
            assert_eq!(admin.can_be_edited, true);
            assert_eq!(admin.can_change_info, false);
            assert_eq!(admin.can_post_messages, Some(true));
            assert_eq!(admin.can_edit_messages, Some(false));
            assert_eq!(admin.can_delete_messages, true);
            assert_eq!(admin.can_invite_users, false);
            assert_eq!(admin.can_restrict_members, true);
            assert_eq!(admin.can_pin_messages, Some(false));
            assert_eq!(admin.can_promote_members, true);
        } else {
            panic!("Unexpected chat member: {:?}", admin);
        }
    }

    #[test]
    fn deserialize_chat_member_creator() {
        let creator: ChatMember = serde_json::from_value(serde_json::json!({
            "status": "creator",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "firstname"
            }
        }))
        .unwrap();
        assert!(creator.is_member());
        assert_eq!(creator.user().id, 1);
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
    fn deserialize_chat_member_kicked() {
        let mut kicked: ChatMember = serde_json::from_value(serde_json::json!({
            "status": "kicked",
            "user": {
                "id": 1,
                "is_bot": true,
                "first_name": "firstname",
                "last_name": "lastname",
                "username": "username"
            },
            "until_date": 0
        }))
        .unwrap();
        assert!(!kicked.is_member());
        assert_eq!(kicked.user().id, 1);
        if let ChatMember::Kicked(ref mut kicked) = kicked {
            assert_eq!(kicked.user.id, 1);
            assert_eq!(kicked.user.is_bot, true);
            assert_eq!(kicked.user.first_name, "firstname");
            assert_eq!(kicked.user.last_name.take().unwrap(), "lastname");
            assert_eq!(kicked.user.username.take().unwrap(), "username");
            assert!(kicked.user.language_code.is_none());
            assert_eq!(kicked.until_date, 0);
        } else {
            panic!("Unexpected chat member: {:?}", kicked);
        }
    }

    #[test]
    fn deserialize_chat_member_left() {
        let left: ChatMember = serde_json::from_value(serde_json::json!({
            "status": "left",
            "user": {
                "id": 1,
                "is_bot": true,
                "first_name": "firstname"
            }
        }))
        .unwrap();
        assert!(!left.is_member());
        assert_eq!(left.user().id, 1);
        if let ChatMember::Left(ref left) = left {
            assert_eq!(left.id, 1);
            assert_eq!(left.is_bot, true);
            assert_eq!(left.first_name, "firstname");
            assert!(left.last_name.is_none());
            assert!(left.username.is_none());
            assert!(left.language_code.is_none());
        } else {
            panic!("Unexpected chat member: {:?}", left);
        }
    }

    #[test]
    fn deserialize_chat_member_plain() {
        let plain: ChatMember = serde_json::from_value(serde_json::json!({
            "status": "member",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "firstname"
            }
        }))
        .unwrap();
        assert!(plain.is_member());
        assert_eq!(plain.user().id, 1);
        if let ChatMember::Member(ref plain) = plain {
            assert_eq!(plain.id, 1);
            assert_eq!(plain.is_bot, false);
            assert_eq!(plain.first_name, "firstname");
            assert!(plain.last_name.is_none());
            assert!(plain.username.is_none());
            assert!(plain.language_code.is_none());
        } else {
            panic!("Unexpected chat member: {:?}", plain);
        }
    }

    #[test]
    fn deserialize_chat_member_restricted() {
        let restricted: ChatMember = serde_json::from_value(serde_json::json!({
            "status": "restricted",
            "user": {
                "id": 1,
                "is_bot": true,
                "first_name": "firstname"
            },
            "until_date": 0,
            "can_change_info": true,
            "can_invite_users": false,
            "can_send_polls": true,
            "can_pin_messages": false,
            "can_send_messages": true,
            "can_send_media_messages": false,
            "can_send_other_messages": true,
            "can_add_web_page_previews": false,
            "is_member": true
        }))
        .unwrap();
        assert_eq!(restricted.user().id, 1);
        assert!(restricted.is_member());
        if let ChatMember::Restricted(ref restricted) = restricted {
            assert_eq!(restricted.user.id, 1);
            assert_eq!(restricted.user.is_bot, true);
            assert_eq!(restricted.user.first_name, "firstname");
            assert!(restricted.user.last_name.is_none());
            assert!(restricted.user.username.is_none());
            assert!(restricted.user.language_code.is_none());
            assert_eq!(restricted.until_date, 0);
            assert!(restricted.can_change_info);
            assert!(!restricted.can_invite_users);
            assert!(restricted.can_send_polls);
            assert!(!restricted.can_pin_messages.unwrap());
            assert!(restricted.can_send_messages);
            assert!(!restricted.can_send_media_messages);
            assert!(restricted.can_send_other_messages);
            assert!(!restricted.can_add_web_page_previews);
            assert!(restricted.is_member);
        } else {
            panic!("Unexpected chat member: {:?}", restricted);
        }
    }
}
