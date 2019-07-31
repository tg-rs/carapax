use serde::{Deserialize, Serialize};

/// Describes actions that a non-administrator user is allowed to take in a chat
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ChatPermissions {
    /// True, if the user is allowed to send text messages, contacts, locations and venues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,
    /// True, if the user is allowed to send audios, documents,
    /// photos, videos, video notes and voice notes, implies can_send_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_media_messages: Option<bool>,
    /// True, if the user is allowed to send polls, implies can_send_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,
    /// True, if the user is allowed to send animations, games, stickers
    /// and use inline bots, implies can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,
    /// True, if the user is allowed to add web page previews to their messages,
    /// implies can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
    /// True, if the user is allowed to change the chat title, photo and other settings
    ///
    /// Ignored in public supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// True, if the user is allowed to invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// True, if the user is allowed to pin messages
    ///
    /// Ignored in public supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
}

impl ChatPermissions {
    /// Restrict everything
    pub fn restricted() -> Self {
        Self {
            can_send_messages: Some(false),
            can_send_media_messages: Some(false),
            can_send_polls: Some(false),
            can_send_other_messages: Some(false),
            can_add_web_page_previews: Some(false),
            can_change_info: Some(false),
            can_invite_users: Some(false),
            can_pin_messages: Some(false),
        }
    }

    /// Allow everything
    pub fn allowed() -> Self {
        Self {
            can_send_messages: Some(true),
            can_send_media_messages: Some(true),
            can_send_polls: Some(true),
            can_send_other_messages: Some(true),
            can_add_web_page_previews: Some(true),
            can_change_info: Some(true),
            can_invite_users: Some(true),
            can_pin_messages: Some(true),
        }
    }

    /// Permission to send text messages, contacts, locations and venues
    pub fn with_send_messages(mut self, flag: bool) -> Self {
        self.can_send_messages = Some(flag);
        self
    }

    /// Permission to send audios, documents, photos, videos, video notes and voice notes
    pub fn with_send_media_messages(mut self, flag: bool) -> Self {
        self.can_send_media_messages = Some(flag);
        self
    }

    /// Permission to send polls
    pub fn with_send_polls(mut self, flag: bool) -> Self {
        self.can_send_polls = Some(flag);
        self
    }

    /// Permission to send animations, games, stickers and use inline bots
    pub fn with_send_other_messages(mut self, flag: bool) -> Self {
        self.can_send_other_messages = Some(flag);
        self
    }

    /// Permission add web page previews to messages
    pub fn with_add_web_page_previews(mut self, flag: bool) -> Self {
        self.can_add_web_page_previews = Some(flag);
        self
    }

    /// Permission to change the chat title, photo and other settings
    pub fn with_change_info(mut self, flag: bool) -> Self {
        self.can_change_info = Some(flag);
        self
    }

    /// Permission to invite new users to the chat
    pub fn with_invite_users(mut self, flag: bool) -> Self {
        self.can_invite_users = Some(flag);
        self
    }

    /// Permission to pin messages
    pub fn with_pin_messages(mut self, flag: bool) -> Self {
        self.can_pin_messages = Some(flag);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data: ChatPermissions = serde_json::from_value(serde_json::json!({
            "can_send_messages": true,
            "can_send_media_messages": false,
            "can_send_polls": true,
            "can_send_other_messages": false,
            "can_add_web_page_previews": true,
            "can_change_info": false,
            "can_invite_users": true,
            "can_pin_messages": false,
        }))
        .unwrap();
        assert!(data.can_send_messages.unwrap());
        assert!(!data.can_send_media_messages.unwrap());
        assert!(data.can_send_polls.unwrap());
        assert!(!data.can_send_other_messages.unwrap());
        assert!(data.can_add_web_page_previews.unwrap());
        assert!(!data.can_change_info.unwrap());
        assert!(data.can_invite_users.unwrap());
        assert!(!data.can_pin_messages.unwrap());
    }

    #[test]
    fn serialize() {
        let permissions = ChatPermissions::default()
            .with_send_messages(true)
            .with_send_media_messages(false)
            .with_send_polls(true)
            .with_send_other_messages(false)
            .with_add_web_page_previews(true)
            .with_change_info(false)
            .with_invite_users(true)
            .with_pin_messages(false);
        assert_eq!(
            serde_json::to_value(permissions).unwrap(),
            serde_json::json!({
                "can_send_messages": true,
                "can_send_media_messages": false,
                "can_send_polls": true,
                "can_send_other_messages": false,
                "can_add_web_page_previews": true,
                "can_change_info": false,
                "can_invite_users": true,
                "can_pin_messages": false,
            })
        );
        assert_eq!(
            serde_json::to_value(ChatPermissions::default()).unwrap(),
            serde_json::json!({})
        );

        assert_eq!(
            serde_json::to_value(ChatPermissions::allowed()).unwrap(),
            serde_json::json!({
                "can_send_messages": true,
                "can_send_media_messages": true,
                "can_send_polls": true,
                "can_send_other_messages": true,
                "can_add_web_page_previews": true,
                "can_change_info": true,
                "can_invite_users": true,
                "can_pin_messages": true,
            })
        );

        assert_eq!(
            serde_json::to_value(ChatPermissions::restricted()).unwrap(),
            serde_json::json!({
                "can_send_messages": false,
                "can_send_media_messages": false,
                "can_send_polls": false,
                "can_send_other_messages": false,
                "can_add_web_page_previews": false,
                "can_change_info": false,
                "can_invite_users": false,
                "can_pin_messages": false,
            })
        );
    }
}
