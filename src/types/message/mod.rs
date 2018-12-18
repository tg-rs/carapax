use crate::types::chat::Chat;
use crate::types::message::raw::RawMessage;
use crate::types::primitive::Integer;
use serde::de::{Deserialize, Deserializer, Error};

mod data;
mod forward;
mod kind;
mod raw;
mod text;

pub use self::data::MessageData;
pub use self::forward::{Forward, ForwardFrom};
pub use self::kind::MessageKind;
pub(crate) use self::raw::RawMessageEntity;
pub use self::text::{ParseEntitiesError, ParsedText, Text, TextEntity, TextEntityData};

/// This object represents a message
#[derive(Debug)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub id: Integer,
    /// Date the message was sent in Unix time
    pub date: Integer,
    /// Contains chat-specific data
    pub kind: MessageKind,
    /// Forwarded data
    pub forward: Option<Forward>,
    /// For replies, the original message
    /// Note that the Message object in this field will not contain further
    /// reply_to fields even if it itself is a reply
    pub reply_to: Option<Box<Message>>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<Integer>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<String>,
    /// Contains message data
    pub data: MessageData,
}

impl Message {
    fn from_raw(raw: RawMessage) -> Result<Message, String> {
        macro_rules! required {
            ($name:ident) => {{
                match raw.$name {
                    Some(val) => val,
                    None => return Err(format!("\"{}\" field is missing", stringify!($name))),
                }
            }};
        };

        let forward_info = match (
            raw.forward_date,
            raw.forward_from,
            raw.forward_from_chat,
            raw.forward_from_message_id,
            raw.forward_signature,
        ) {
            (Some(date), Some(user), None, None, None) => Some(Forward {
                date,
                from: ForwardFrom::User(user),
            }),
            (Some(date), None, Some(Chat::Channel(chat)), Some(message_id), signature) => {
                Some(Forward {
                    date,
                    from: ForwardFrom::Channel {
                        chat,
                        message_id,
                        signature,
                    },
                })
            }
            (None, None, None, None, None) => None,
            _ => return Err(String::from("Unexpected forward_* fields combination")),
        };

        let message_kind = match raw.chat {
            Chat::Channel(chat) => MessageKind::Channel {
                chat,
                author_signature: raw.author_signature,
            },
            Chat::Group(chat) => MessageKind::Group {
                chat,
                from: required!(from),
            },
            Chat::Private(chat) => MessageKind::Private {
                chat,
                from: required!(from),
            },
            Chat::Supergroup(chat) => MessageKind::Supergroup {
                chat,
                from: required!(from),
            },
        };

        let caption = match raw.caption {
            Some(data) => Some(Text {
                data,
                entities: raw.caption_entities,
            }),
            None => None,
        };

        let message_data = if let Some(data) = raw.animation {
            // Animation must be matched before document
            MessageData::Animation(data)
        } else if let Some(data) = raw.audio {
            MessageData::Audio { caption, data }
        } else if let Some(_) = raw.channel_chat_created {
            MessageData::ChannelChatCreated
        } else if let Some(data) = raw.connected_website {
            MessageData::ConnectedWebsite(data)
        } else if let Some(data) = raw.contact {
            MessageData::Contact(data)
        } else if let Some(_) = raw.delete_chat_photo {
            MessageData::DeleteChatPhoto
        } else if let Some(data) = raw.document {
            MessageData::Document { caption, data }
        } else if let Some(data) = raw.game {
            MessageData::Game(data)
        } else if let Some(_) = raw.group_chat_created {
            MessageData::GroupChatCreated
        } else if let Some(data) = raw.invoice {
            MessageData::Invoice(data)
        } else if let Some(data) = raw.left_chat_member {
            MessageData::LeftChatMember(data)
        } else if let Some(data) = raw.location {
            MessageData::Location(data)
        } else if let Some(data) = raw.migrate_from_chat_id {
            MessageData::MigrateFromChatId(data)
        } else if let Some(data) = raw.migrate_to_chat_id {
            MessageData::MigrateToChatId(data)
        } else if let Some(data) = raw.new_chat_members {
            MessageData::NewChatMembers(data)
        } else if let Some(data) = raw.new_chat_photo {
            MessageData::NewChatPhoto(data)
        } else if let Some(data) = raw.new_chat_title {
            MessageData::NewChatTitle(data)
        } else if let Some(data) = raw.passport_data {
            MessageData::PassportData(data)
        } else if let Some(data) = raw.pinned_message {
            let data = Message::from_raw(*data)?;
            MessageData::PinnedMessage(Box::new(data))
        } else if let Some(data) = raw.photo {
            MessageData::Photo { caption, data }
        } else if let Some(data) = raw.sticker {
            MessageData::Sticker(data)
        } else if let Some(data) = raw.successful_payment {
            MessageData::SuccessfulPayment(data)
        } else if let Some(_) = raw.supergroup_chat_created {
            MessageData::SupergroupChatCreated
        } else if let Some(data) = raw.text {
            MessageData::Text(Text {
                data,
                entities: raw.entities,
            })
        } else if let Some(data) = raw.venue {
            MessageData::Venue(data)
        } else if let Some(data) = raw.video {
            MessageData::Video { caption, data }
        } else if let Some(data) = raw.video_note {
            MessageData::VideoNote(data)
        } else if let Some(data) = raw.voice {
            MessageData::Voice { caption, data }
        } else {
            return Err(String::from("Can not get message data"));
        };

        let reply_to_message = match raw.reply_to_message {
            Some(x) => Some(Box::new(Message::from_raw(*x)?)),
            None => None,
        };

        Ok(Message {
            id: raw.message_id,
            date: raw.date,
            kind: message_kind,
            forward: forward_info,
            reply_to: reply_to_message,
            edit_date: raw.edit_date,
            media_group_id: raw.media_group_id,
            data: message_data,
        })
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Message, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_msg: RawMessage = Deserialize::deserialize(deserializer)?;
        Message::from_raw(raw_msg).map_err(|e| D::Error::custom(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::chat::ChannelChat;
    use crate::types::user::User;

    #[test]
    fn test_deserialize_message_channel() {
        let input = r#"{
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": 1,
                "type": "channel",
                "title": "channeltitle"
            },
            "text": "test"
        }"#;
        let msg: Message = serde_json::from_str(input).unwrap();
        assert_eq!(msg.id, 1);
        assert_eq!(msg.date, 0);
        if let MessageKind::Channel {
            chat: ChannelChat { id, title, .. },
            author_signature,
        } = msg.kind
        {
            assert_eq!(id, 1);
            assert_eq!(title, "channeltitle");
            assert_eq!(author_signature, None);
        } else {
            panic!("Unexpected message kind: {:?}", msg.kind);
        }
        if let MessageData::Text(Text { data, entities }) = msg.data {
            assert_eq!(data, "test");
            assert_eq!(entities.is_none(), true);
        } else {
            panic!("Unexpected message data: {:?}", msg.data);
        }
    }

    #[test]
    fn test_deserialize_message_group() {
        let input = r#"{
            "message_id": 1,
            "date": 0,
            "from": {
                "id": 1,
                "first_name": "firstname",
                "is_bot": false
            },
            "chat": {
                "id": 1,
                "type": "group",
                "title": "grouptitle",
                "all_members_are_administrators": true
            },
            "text": "test"
        }"#;
        let msg: Message = serde_json::from_str(input).unwrap();
        assert_eq!(msg.id, 1);
        assert_eq!(msg.date, 0);
        if let MessageKind::Group { chat, from } = msg.kind {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, "grouptitle");
            assert_eq!(chat.all_members_are_administrators, true);
            assert_eq!(from.id, 1);
            assert_eq!(from.first_name, "firstname");
            assert_eq!(from.is_bot, false);
        } else {
            panic!("Unexpected message kind: {:?}", msg.kind);
        }
        if let MessageData::Text(Text { data, entities }) = msg.data {
            assert_eq!(data, "test");
            assert_eq!(entities.is_none(), true);
        } else {
            panic!("Unexpected message data: {:?}", msg.data);
        }

        let input = r#"{
            "message_id": 1, "date": 0, "text": "test",
            "chat": {"id": 1, "type": "group", "title": "grouptitle", "all_members_are_administrators": true}
        }"#;
        let err = serde_json::from_str::<Message>(input).unwrap_err();
        assert_eq!(err.to_string(), String::from("\"from\" field is missing"));
    }

    #[test]
    fn test_deserialize_message_private() {
        let input = r#"{
            "message_id": 1,
            "date": 0,
            "from": {
                "id": 1,
                "first_name": "firstname",
                "is_bot": false
            },
            "chat": {
                "id": 1,
                "type": "private",
                "first_name": "firstname"
            },
            "text": "test"
        }"#;
        let msg: Message = serde_json::from_str(input).unwrap();
        assert_eq!(msg.id, 1);
        assert_eq!(msg.date, 0);
        if let MessageKind::Private { chat, from } = msg.kind {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.first_name, "firstname");
            assert_eq!(from.id, 1);
            assert_eq!(from.first_name, "firstname");
            assert_eq!(from.is_bot, false);
        } else {
            panic!("Unexpected message kind: {:?}", msg.kind);
        }
        if let MessageData::Text(Text { data, entities }) = msg.data {
            assert_eq!(data, "test");
            assert_eq!(entities.is_none(), true);
        } else {
            panic!("Unexpected message data: {:?}", msg.data);
        }

        let input = r#"{
            "message_id": 1, "date": 0, "text": "test",
            "chat": {"id": 1, "type": "private", "first_name": "firstname"}
        }"#;
        let err = serde_json::from_str::<Message>(input).unwrap_err();
        assert_eq!(err.to_string(), String::from("\"from\" field is missing"));
    }

    #[test]
    fn test_deserialize_message_supergroup() {
        let input = r#"{
            "message_id": 1,
            "date": 0,
            "from": {
                "id": 1,
                "first_name": "firstname",
                "is_bot": false
            },
            "chat": {
                "id": 1,
                "type": "supergroup",
                "title": "supergrouptitle"
            },
            "text": "test"
        }"#;
        let msg: Message = serde_json::from_str(input).unwrap();
        assert_eq!(msg.id, 1);
        assert_eq!(msg.date, 0);
        if let MessageKind::Supergroup { chat, from } = msg.kind {
            assert_eq!(chat.id, 1);
            assert_eq!(chat.title, "supergrouptitle");
            assert_eq!(from.id, 1);
            assert_eq!(from.first_name, "firstname");
            assert_eq!(from.is_bot, false);
        } else {
            panic!("Unexpected message kind: {:?}", msg.kind);
        }
        if let MessageData::Text(Text { data, entities }) = msg.data {
            assert_eq!(data, "test");
            assert_eq!(entities.is_none(), true);
        } else {
            panic!("Unexpected message data: {:?}", msg.data);
        }

        let input = r#"{
            "message_id": 1, "date": 0,
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test"
        }"#;
        let err = serde_json::from_str::<Message>(input).unwrap_err();
        assert_eq!(err.to_string(), String::from("\"from\" field is missing"));
    }

    #[test]
    fn test_deserialize_message_forward() {
        let input = r#"{
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test",
            "forward_from": {"id": 2, "first_name": "firstname", "is_bot": false},
            "forward_date": 0
        }"#;
        let msg: Message = serde_json::from_str(input).unwrap();
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

        let input = r#"{
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test",
            "forward_from_chat": {"id": 1, "type": "channel", "title": "test"},
            "forward_from_message_id": 1,
            "forward_signature": "test",
            "forward_date": 0
        }"#;
        let msg: Message = serde_json::from_str(input).unwrap();
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

        let input = r#"{
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test",
            "forward_from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "forward_from_chat": {"id": 1, "type": "channel", "title": "test"},
            "forward_from_message_id": 1,
            "forward_signature": "test",
            "forward_date": 0
        }"#;
        let err = serde_json::from_str::<Message>(input).unwrap_err();
        assert_eq!(
            err.to_string(),
            String::from("Unexpected forward_* fields combination")
        );
    }

    #[test]
    fn test_deserialize_message_reply() {
        let input = r#"{
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test",
            "reply_to_message": {
                "message_id": 1, "date": 0,
                "from": {"id": 1, "first_name": "firstname", "is_bot": false},
                "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
                "text": "test"
            }
        }"#;
        let msg: Message = serde_json::from_str(input).unwrap();
        if let Some(msg) = msg.reply_to {
            assert_eq!(msg.id, 1);
        } else {
            panic!("Unexpected reply_to data: {:?}", msg.reply_to);
        }
    }

    #[test]
    fn test_deserialize_message_data() {
        let input = r#"{
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "animation": {
                "file_id": "fileid",
                "width": 200,
                "height": 200,
                "duration": 10
            },
            "document": {
                "file_id": "fileid"
            }
        }"#;
        let msg: Message = serde_json::from_str(input).unwrap();
        if let MessageData::Animation(animation) = msg.data {
            assert_eq!(animation.file_id, String::from("fileid"));
            assert_eq!(animation.width, 200);
            assert_eq!(animation.height, 200);
            assert_eq!(animation.duration, 10);
        } else {
            panic!("Unexpected message data: {:?}", msg.data)
        }
    }

    #[test]
    fn test_deserialize_message_entities() {
        let input = r#"{
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "bold /botcommand $cashtag code u@h.z #hashtag italic @mention phone pre textlink textmention url",
            "entities": [
                {"type": "bold", "offset": 0, "length": 4},
                {"type": "bot_command", "offset": 5, "length": 11},
                {"type": "cashtag", "offset": 17, "length": 8},
                {"type": "code", "offset": 26, "length": 4},
                {"type": "email", "offset": 31, "length": 5},
                {"type": "hashtag", "offset": 37, "length": 8},
                {"type": "italic", "offset": 46, "length": 6},
                {"type": "mention", "offset": 53, "length": 8},
                {"type": "phone_number", "offset": 62, "length": 5},
                {"type": "pre", "offset": 68, "length": 3},
                {"type": "text_link", "offset": 72, "length": 8, "url": "https://example.com"},
                {
                    "type": "text_mention",
                    "offset": 81,
                    "length": 11,
                    "user": {
                        "id": 1,
                        "first_name": "test",
                        "is_bot": false
                    }
                },
                {"type": "url", "offset": 93, "length": 3}
            ]
        }"#;
        let msg: Message = serde_json::from_str(input).unwrap();
        if let MessageData::Text(text) = msg.data {
            let parsed = text.to_parsed().unwrap();
            let entities = parsed.entities;
            assert_eq!(
                vec![
                    TextEntity::Bold(TextEntityData {
                        data: String::from("bold"),
                        offset: 0,
                        length: 4
                    }),
                    TextEntity::BotCommand {
                        command: String::from("/botcommand"),
                        bot_name: None,
                        data: TextEntityData {
                            data: String::from("/botcommand"),
                            offset: 5,
                            length: 11
                        }
                    },
                    TextEntity::Cashtag(TextEntityData {
                        data: String::from("$cashtag"),
                        offset: 17,
                        length: 8
                    }),
                    TextEntity::Code(TextEntityData {
                        data: String::from("code"),
                        offset: 26,
                        length: 4
                    }),
                    TextEntity::Email(TextEntityData {
                        data: String::from("u@h.z"),
                        offset: 31,
                        length: 5
                    }),
                    TextEntity::Hashtag(TextEntityData {
                        data: String::from("#hashtag"),
                        offset: 37,
                        length: 8
                    }),
                    TextEntity::Italic(TextEntityData {
                        data: String::from("italic"),
                        offset: 46,
                        length: 6
                    }),
                    TextEntity::Mention(TextEntityData {
                        data: String::from("@mention"),
                        offset: 53,
                        length: 8
                    }),
                    TextEntity::PhoneNumber(TextEntityData {
                        data: String::from("phone"),
                        offset: 62,
                        length: 5
                    }),
                    TextEntity::Pre(TextEntityData {
                        data: String::from("pre"),
                        offset: 68,
                        length: 3
                    }),
                    TextEntity::TextLink {
                        data: TextEntityData {
                            data: String::from("textlink"),
                            offset: 72,
                            length: 8
                        },
                        url: String::from("https://example.com")
                    },
                    TextEntity::TextMention {
                        data: TextEntityData {
                            data: String::from("textmention"),
                            offset: 81,
                            length: 11
                        },
                        user: User {
                            id: 1,
                            is_bot: false,
                            first_name: String::from("test"),
                            last_name: None,
                            username: None,
                            language_code: None
                        }
                    },
                    TextEntity::Url(TextEntityData {
                        data: String::from("url"),
                        offset: 93,
                        length: 3
                    })
                ],
                entities
            )
        } else {
            panic!("Unexpected message data: {:?}", msg.data)
        }
    }
}
