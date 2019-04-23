use crate::types::{photo_size::PhotoSize, primitive::Integer};
use serde::Deserialize;

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
    fn deserialize_user_profile_photos() {
        let data: UserProfilePhotos = serde_json::from_value(serde_json::json!({
            "total_count": 2,
            "photos": [
                [
                    {
                        "file_id": "photo-1-big",
                        "width": 500,
                        "height": 500,
                        "file_size": 9999
                    },
                    {
                        "file_id": "photo-1-small",
                        "width": 100,
                        "height": 100,
                        "file_size": 1111
                    },
                ],
                [
                    {
                        "file_id": "photo-2-big",
                        "width": 500,
                        "height": 500,
                        "file_size": 9999
                    },
                    {
                        "file_id": "photo-2-small",
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
        assert_eq!(small.file_id, "photo-1-small");

        let group2 = &data.photos[1];
        assert_eq!(group2.len(), 2);
        let big = &group2[0];
        let small = &group2[1];
        assert_eq!(big.file_id, "photo-2-big");
        assert_eq!(small.file_id, "photo-2-small");
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
