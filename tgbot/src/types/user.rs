use crate::types::{
    photo_size::PhotoSize,
    primitive::{Integer, ParseMode},
};
use serde::Deserialize;
use std::{error::Error, fmt};

/// A Bot info returned in getMe
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd)]
pub struct Me {
    /// Unique identifier of this bot
    pub id: Integer,
    /// Bot's username
    pub username: String,
    /// Bot's first name
    pub first_name: String,
    /// Bot's last name
    pub last_name: Option<String>,
    /// True, if the bot can be invited to groups
    pub can_join_groups: bool,
    /// True, if privacy mode is disabled for the bot
    pub can_read_all_group_messages: bool,
    /// True, if the bot supports inline queries
    pub supports_inline_queries: bool,
}

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

impl User {
    /// Returns full name of the user (first name + last name)
    pub fn get_full_name(&self) -> String {
        let mut full_name = self.first_name.clone();
        if let Some(ref last_name) = self.last_name {
            full_name.push(' ');
            full_name += last_name;
        }
        full_name
    }

    /// Returns a link to the user (tg://user?id=xxx)
    ///
    /// These links will work only if they are used inside an inline link.
    /// For example, they will not work, when used in an inline keyboard button or in a message text.
    pub fn get_link(&self) -> String {
        format!("tg://user?id={}", self.id)
    }

    /// Returns a mention for the user
    ///
    /// These mentions are only guaranteed to work if the user has contacted the bot in the past,
    /// has sent a callback query to the bot via inline button or is a member
    /// in the group where he was mentioned.
    pub fn get_mention(&self, parse_mode: ParseMode) -> Result<String, MentionError> {
        let full_name = parse_mode.escape(self.get_full_name());
        let user_link = self.get_link();
        Ok(match parse_mode {
            ParseMode::Markdown => return Err(MentionError::UnsupportedParseMode(parse_mode)),
            ParseMode::MarkdownV2 => format!(r#"[{}]({})"#, full_name, user_link),
            ParseMode::Html => format!(r#"<a href="{}">{}</a>"#, user_link, full_name),
        })
    }
}

/// An error occurred when getting user mention
#[derive(Debug)]
pub enum MentionError {
    /// Parse mode is not supported
    UnsupportedParseMode(ParseMode),
}

impl Error for MentionError {}

impl fmt::Display for MentionError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MentionError::UnsupportedParseMode(parse_mode) => {
                write!(out, "can not mention with {} parse mode", parse_mode)
            }
        }
    }
}

/// User's profile pictures
#[derive(Clone, Debug, Deserialize)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: Integer,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<Vec<PhotoSize>>,
}

/// User ID
#[derive(Clone, Debug)]
pub enum UserId {
    /// @username of a user
    Username(String),
    /// ID of a user
    Id(Integer),
}

impl From<&str> for UserId {
    fn from(username: &str) -> UserId {
        UserId::Username(String::from(username))
    }
}

impl From<String> for UserId {
    fn from(username: String) -> UserId {
        UserId::Username(username)
    }
}

impl From<Integer> for UserId {
    fn from(id: Integer) -> UserId {
        UserId::Id(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_me() {
        let data: Me = serde_json::from_value(serde_json::json!({
            "id": 1,
            "is_bot": true,
            "first_name": "Loo",
            "last_name": "Maclin",
            "username": "loomaclinbot",
            "can_join_groups": true,
            "can_read_all_group_messages": true,
            "supports_inline_queries": false
        }))
        .unwrap();
        assert_eq!(data.id, 1);
        assert_eq!(data.first_name, "Loo");
        assert_eq!(data.last_name.unwrap(), "Maclin");
        assert_eq!(data.username, "loomaclinbot");
        assert!(data.can_join_groups);
        assert!(data.can_read_all_group_messages);
        assert!(!data.supports_inline_queries);
    }

    #[test]
    fn deserialize_user_full() {
        let data: User = serde_json::from_value(serde_json::json!({
            "id": 1,
            "first_name": "Vladimir",
            "last_name": "Zelenskiy",
            "is_bot": false,
            "username": "zelenskiy",
            "language_code": "UA"
        }))
        .unwrap();
        assert_eq!(data.id, 1);
        assert_eq!(data.first_name, "Vladimir");
        assert_eq!(data.last_name.unwrap(), "Zelenskiy");
        assert!(!data.is_bot);
        assert_eq!(data.username.unwrap(), "zelenskiy");
        assert_eq!(data.language_code.unwrap(), "UA");
    }

    #[test]
    fn deserialize_user_partial() {
        let data: User = serde_json::from_value(serde_json::json!({
            "id": 1,
            "first_name": "Vladimir",
            "is_bot": false
        }))
        .unwrap();
        assert_eq!(data.id, 1);
        assert_eq!(data.first_name, "Vladimir");
        assert!(data.last_name.is_none());
        assert!(!data.is_bot);
        assert!(data.username.is_none());
        assert!(data.language_code.is_none());
    }

    #[test]
    fn get_user_full_name() {
        let user: User = serde_json::from_value(serde_json::json!({
            "id": 1,
            "first_name": "first",
            "last_name": "last",
            "is_bot": false
        }))
        .unwrap();
        assert_eq!(user.get_full_name(), "first last");

        let user: User = serde_json::from_value(serde_json::json!({
            "id": 1,
            "first_name": "first",
            "is_bot": false
        }))
        .unwrap();
        assert_eq!(user.get_full_name(), "first");
    }

    #[test]
    fn get_user_mention() {
        let user: User = serde_json::from_value(serde_json::json!({
            "id": 1,
            "first_name": r#"_*[]()~`>#+-=|{}.!<&"#,
            "is_bot": false
        }))
        .unwrap();
        assert_eq!(
            user.get_mention(ParseMode::Html).unwrap(),
            r#"<a href="tg://user?id=1">_*[]()~`&gt;#+-=|{}.!&lt;&amp;</a>"#
        );
        assert_eq!(
            user.get_mention(ParseMode::MarkdownV2).unwrap(),
            r#"[\_\*\[\]\(\)\~\`\>\#\+\-\=\|\{\}\.\!<&](tg://user?id=1)"#
        );
        assert!(user.get_mention(ParseMode::Markdown).is_err());
    }

    #[test]
    fn deserialize_user_profile_photos() {
        let data: UserProfilePhotos = serde_json::from_value(serde_json::json!({
            "total_count": 2,
            "photos": [
                [
                    {
                        "file_id": "photo-1-big",
                        "file_unique_id": "photo-1-big-unique",
                        "width": 500,
                        "height": 500,
                        "file_size": 9999
                    },
                    {
                        "file_id": "photo-1-small",
                        "file_unique_id": "photo-1-small-unique",
                        "width": 100,
                        "height": 100,
                        "file_size": 1111
                    },
                ],
                [
                    {
                        "file_id": "photo-2-big",
                        "file_unique_id": "photo-2-big-unique",
                        "width": 500,
                        "height": 500,
                        "file_size": 9999
                    },
                    {
                        "file_id": "photo-2-small",
                        "file_unique_id": "photo-2-small-unique",
                        "width": 100,
                        "height": 100,
                        "file_size": 1111
                    },
                ],
            ]
        }))
        .unwrap();
        assert_eq!(data.total_count, 2);

        assert_eq!(data.photos.len(), 2);

        let group1 = &data.photos[0];
        assert_eq!(group1.len(), 2);
        let big = &group1[0];
        let small = &group1[1];
        assert_eq!(big.file_id, "photo-1-big");
        assert_eq!(big.file_unique_id, "photo-1-big-unique");
        assert_eq!(small.file_id, "photo-1-small");
        assert_eq!(small.file_unique_id, "photo-1-small-unique");

        let group2 = &data.photos[1];
        assert_eq!(group2.len(), 2);
        let big = &group2[0];
        let small = &group2[1];
        assert_eq!(big.file_id, "photo-2-big");
        assert_eq!(big.file_unique_id, "photo-2-big-unique");
        assert_eq!(small.file_id, "photo-2-small");
        assert_eq!(small.file_unique_id, "photo-2-small-unique");
    }

    #[test]
    fn user_id() {
        let username = UserId::from("username");
        if let UserId::Username(username) = username {
            assert_eq!(username, "username");
        } else {
            panic!("Unexpected username: {:?}", username);
        }

        let username = UserId::from(String::from("username"));
        if let UserId::Username(username) = username {
            assert_eq!(username, "username");
        } else {
            panic!("Unexpected username: {:?}", username);
        }

        let user_id = UserId::from(1);
        if let UserId::Id(user_id) = user_id {
            assert_eq!(user_id, 1);
        } else {
            panic!("Unexpected user_id: {:?}", user_id);
        }
    }
}
