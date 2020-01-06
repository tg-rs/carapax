use crate::types::{chat::ChannelChat, primitive::Integer, user::User};

/// Contains information about original message
#[derive(Clone, Debug)]
pub struct Forward {
    /// Sender of the original message
    pub from: ForwardFrom,
    /// Date the original message was sent in Unix time
    pub date: Integer,
}

/// Sender of the original message
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum ForwardFrom {
    /// Information about user
    User(User),
    /// Name of user who has hidden link to account
    HiddenUser(String),
    /// Information about channel
    Channel {
        /// Information about the original chat
        chat: ChannelChat,
        /// Identifier of the original message in the channel
        message_id: Integer,
        /// Signature of the post author if present
        signature: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Message;
    use serde_json::json;

    #[test]
    fn deserialize_forward_from_user() {
        let input = json!({
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test",
            "forward_from": {"id": 2, "first_name": "firstname", "is_bot": false},
            "forward_date": 0
        });
        let msg: Message = serde_json::from_value(input).unwrap();
        if let Some(Forward {
            date,
            from: ForwardFrom::User(user),
        }) = msg.forward
        {
            assert_eq!(date, 0);
            assert_eq!(user.id, 2);
            assert_eq!(user.first_name, String::from("firstname"));
            assert_eq!(user.is_bot, false);
        } else {
            panic!("Unexpected forward data: {:?}", msg.forward);
        }
    }

    #[test]
    fn deserialize_forward_from_hidden_user() {
        let input = json!({
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test",
            "forward_sender_name": "Hidden User",
            "forward_date": 0
        });
        let msg: Message = serde_json::from_value(input).unwrap();
        if let Some(Forward {
            date,
            from: ForwardFrom::HiddenUser(name),
        }) = msg.forward
        {
            assert_eq!(date, 0);
            assert_eq!(name, String::from("Hidden User"));
        } else {
            panic!("Unexpected forward data: {:?}", msg.forward);
        }
    }

    #[test]
    fn deserialize_forward_from_channel() {
        let input = json!({
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test",
            "forward_from_chat": {"id": 1, "type": "channel", "title": "test"},
            "forward_from_message_id": 1,
            "forward_signature": "test",
            "forward_date": 0
        });
        let msg: Message = serde_json::from_value(input).unwrap();
        if let Some(Forward {
            date,
            from:
                ForwardFrom::Channel {
                    chat,
                    message_id,
                    signature,
                },
        }) = msg.forward
        {
            assert_eq!(date, 0);
            assert_eq!(message_id, 1);
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, String::from("test"));
            assert_eq!(signature, Some(String::from("test")));
        } else {
            panic!("Unexpected forward data: {:?}", msg.forward);
        }
    }

    #[test]
    fn deserialize_forward_from_unexpected() {
        let input = json!({
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test",
            "forward_from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "forward_from_chat": {"id": 1, "type": "channel", "title": "test"},
            "forward_from_message_id": 1,
            "forward_signature": "test",
            "forward_date": 0
        });
        let err = serde_json::from_value::<Message>(input).unwrap_err();
        assert_eq!(err.to_string(), String::from("unexpected forward_* fields combination"));
    }
}
