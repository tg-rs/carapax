use crate::types::{
    chat::Chat, message::raw::RawMessage, primitive::Integer, reply_markup::InlineKeyboardMarkup, user::User,
};
use serde::{de::Error, Deserialize, Deserializer};
use std::{error::Error as StdError, fmt};

mod data;
mod forward;
mod kind;
mod raw;
mod text;

pub(crate) use self::raw::RawMessageEntity;
pub use self::{data::*, forward::*, kind::*, text::*};

/// This object represents a message
#[derive(Clone, Debug)]
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
    /// Contains command data
    pub commands: Option<Vec<BotCommand>>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl Message {
    /// Returns true if message has edited and false otherwise
    pub fn is_edited(&self) -> bool {
        self.edit_date.is_some()
    }

    /// Returns ID of the chat
    pub fn get_chat_id(&self) -> Integer {
        match self.kind {
            MessageKind::Private { ref chat, .. } => chat.id,
            MessageKind::Channel { ref chat, .. } => chat.id,
            MessageKind::Group { ref chat, .. } => chat.id,
            MessageKind::Supergroup { ref chat, .. } => chat.id,
        }
    }

    /// Returns username of the chat
    pub fn get_chat_username(&self) -> Option<&str> {
        if let Some(ref username) = match self.kind {
            MessageKind::Private { ref chat, .. } => &chat.username,
            MessageKind::Channel { ref chat, .. } => &chat.username,
            MessageKind::Supergroup { ref chat, .. } => &chat.username,
            _ => &None,
        } {
            Some(username.as_str())
        } else {
            None
        }
    }

    /// Returns author of the message
    pub fn get_user(&self) -> Option<&User> {
        match self.kind {
            MessageKind::Channel { .. } => None,
            MessageKind::Private { ref from, .. }
            | MessageKind::Group { ref from, .. }
            | MessageKind::Supergroup { ref from, .. } => Some(from),
        }
    }

    /// Returns text of the message (includes caption)
    pub fn get_text(&self) -> Option<&Text> {
        match self.data {
            MessageData::Text(ref text)
            | MessageData::Audio {
                caption: Some(ref text),
                ..
            }
            | MessageData::Document {
                caption: Some(ref text),
                ..
            }
            | MessageData::Photo {
                caption: Some(ref text),
                ..
            }
            | MessageData::Video {
                caption: Some(ref text),
                ..
            }
            | MessageData::Voice {
                caption: Some(ref text),
                ..
            } => Some(text),
            _ => None,
        }
    }

    fn from_raw(raw: RawMessage) -> Result<Message, ParseError> {
        macro_rules! required {
            ($name:ident) => {{
                match raw.$name {
                    Some(val) => val,
                    None => return Err(ParseError::MissingField(stringify!($name))),
                }
            }};
        };

        let forward_info = match (
            raw.forward_date,
            raw.forward_from,
            raw.forward_from_chat,
            raw.forward_from_message_id,
            raw.forward_signature,
            raw.forward_sender_name,
        ) {
            (Some(date), Some(user), None, None, None, None) => Some(Forward {
                date,
                from: ForwardFrom::User(user),
            }),
            (Some(date), None, None, None, None, Some(sender_name)) => Some(Forward {
                date,
                from: ForwardFrom::HiddenUser(sender_name),
            }),
            (Some(date), None, Some(Chat::Channel(chat)), Some(message_id), signature, None) => Some(Forward {
                date,
                from: ForwardFrom::Channel {
                    chat,
                    message_id,
                    signature,
                },
            }),
            (None, None, None, None, None, None) => None,
            _ => return Err(ParseError::BadForward),
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
            Some(data) => Some(Text::parse(data, raw.caption_entities)?),
            None => None,
        };

        let reply_to_message = match raw.reply_to_message {
            Some(x) => Some(Box::new(Message::from_raw(*x)?)),
            None => None,
        };

        macro_rules! message {
            ($variant:ident($attr:ident)) => {
                if let Some(data) = raw.$attr {
                    message!(MessageData::$variant(data), None);
                }
            };
            ($variant:ident($attr:ident,caption)) => {
                if let Some(data) = raw.$attr {
                    let commands = if let Some(ref text) = caption {
                        get_commands(text)
                    } else {
                        None
                    };

                    message!(MessageData::$variant { caption, data }, commands);
                }
            };
            ($variant:ident($attr:ident,flag)) => {
                if raw.$attr.unwrap_or(false) {
                    message!(MessageData::$variant, None);
                }
            };
            ($data:expr, $commands:expr) => {
                return Ok(Message {
                    id: raw.message_id,
                    date: raw.date,
                    kind: message_kind,
                    forward: forward_info,
                    reply_to: reply_to_message,
                    edit_date: raw.edit_date,
                    media_group_id: raw.media_group_id,
                    data: $data,
                    commands: $commands,
                    reply_markup: raw.reply_markup,
                });
            };
        };

        message!(Animation(animation));
        message!(Audio(audio, caption));
        message!(ChannelChatCreated(channel_chat_created, flag));
        message!(ConnectedWebsite(connected_website));
        message!(Contact(contact));
        message!(DeleteChatPhoto(delete_chat_photo, flag));
        message!(Document(document, caption));
        message!(Game(game));
        message!(GroupChatCreated(group_chat_created, flag));
        message!(Invoice(invoice));
        message!(LeftChatMember(left_chat_member));
        message!(Location(location));
        message!(MigrateFromChatId(migrate_from_chat_id));
        message!(MigrateToChatId(migrate_to_chat_id));
        message!(NewChatMembers(new_chat_members));
        message!(NewChatPhoto(new_chat_photo));
        message!(NewChatTitle(new_chat_title));
        message!(PassportData(passport_data));
        message!(Photo(photo, caption));
        message!(Poll(poll));
        message!(Sticker(sticker));
        message!(SuccessfulPayment(successful_payment));
        message!(SupergroupChatCreated(supergroup_chat_created, flag));
        message!(Venue(venue));
        message!(Video(video, caption));
        message!(VideoNote(video_note));
        message!(Voice(voice, caption));

        if let Some(data) = raw.pinned_message {
            let data = Message::from_raw(*data)?;
            message!(MessageData::PinnedMessage(Box::new(data)), None);
        }

        if let Some(data) = raw.text {
            let text = Text::parse(data, raw.entities)?;
            let commands = get_commands(&text);
            message!(MessageData::Text(text), commands);
        }

        message!(MessageData::Empty, None)
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Message, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_msg: RawMessage = Deserialize::deserialize(deserializer)?;
        Message::from_raw(raw_msg).map_err(D::Error::custom)
    }
}

/// Result of editMessage* requests
#[derive(Clone, Debug, Deserialize)]
#[allow(clippy::large_enum_variant)]
#[serde(untagged)]
pub enum EditMessageResult {
    /// Returned if edited message is sent by the bot
    Message(Message),
    /// Returned if edited message is NOT sent by the bot
    Bool(bool),
}

#[derive(Debug, derive_more::From)]
enum ParseError {
    BadForward,
    BadText(ParseTextError),
    MissingField(&'static str),
}

impl StdError for ParseError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            ParseError::BadText(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::BadForward => write!(out, "unexpected forward_* fields combination"),
            ParseError::BadText(err) => write!(out, "failed to parse text: {}", err),
            ParseError::MissingField(field) => write!(out, "\"{}\" field is missing", field),
        }
    }
}

fn get_commands(text: &Text) -> Option<Vec<BotCommand>> {
    if let Text {
        entities: Some(ref entities),
        ..
    } = text
    {
        let commands = entities
            .iter()
            .filter_map(|entity| {
                if let TextEntity::BotCommand(command) = entity {
                    Some(command.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<BotCommand>>();

        if !commands.is_empty() {
            return Some(commands);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reply_to() {
        let msg: Message = serde_json::from_value(serde_json::json!({
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
        }))
        .unwrap();
        if let Some(msg) = msg.reply_to {
            assert_eq!(msg.id, 1);
        } else {
            panic!("Unexpected reply_to data: {:?}", msg.reply_to);
        }
    }

    #[test]
    fn reply_to_with_empty_data() {
        let data: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test",
            "reply_to_message": {
                "message_id": 1, "date": 0,
                "from": {"id": 1, "first_name": "firstname", "is_bot": false},
                "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            }
        }))
        .unwrap();
        assert!(data.reply_to.is_some());
    }

    #[test]
    fn is_edited() {
        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test",
            "edit_date": 1
        }))
        .unwrap();
        assert!(msg.is_edited());
    }

    #[test]
    fn get_chat_and_user_data() {
        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "test"
        }))
        .unwrap();
        assert_eq!(msg.get_chat_id(), 1);
        assert!(msg.get_chat_username().is_none());
        assert!(msg.get_user().is_some());

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 2, "type": "supergroup", "title": "supergrouptitle", "username": "supergroupusername"},
            "text": "test"
        }))
        .unwrap();
        assert_eq!(msg.get_chat_id(), 2);
        assert_eq!(msg.get_chat_username().unwrap(), "supergroupusername");
        assert!(msg.get_user().is_some());

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 3, "type": "private", "first_name": "firstname"},
            "text": "test"
        }))
        .unwrap();
        assert_eq!(msg.get_chat_id(), 3);
        assert!(msg.get_chat_username().is_none());
        assert!(msg.get_user().is_some());

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 4, "type": "private", "first_name": "firstname", "username": "username"},
            "text": "test"
        }))
        .unwrap();
        assert_eq!(msg.get_chat_id(), 4);
        assert_eq!(msg.get_chat_username().unwrap(), "username");
        assert!(msg.get_user().is_some());

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 2, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 5, "type": "group", "title": "grouptitle", "all_members_are_administrators": false},
            "text": "test"
        }))
        .unwrap();
        assert_eq!(msg.get_chat_id(), 5);
        assert!(msg.get_chat_username().is_none());
        assert!(msg.get_user().is_some());

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1111,
                "date": 0,
                "author_signature": "test",
                "chat": {
                    "id": 6,
                    "type": "channel",
                    "title": "channeltitle",
                    "username": "channelusername"
                },
                "text": "test message from channel"
        }))
        .unwrap();
        assert_eq!(msg.get_chat_id(), 6);
        assert_eq!(msg.get_chat_username().unwrap(), "channelusername");
        assert!(msg.get_user().is_none());

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1111,
                "date": 0,
                "author_signature": "test",
                "chat": {
                    "id": 7,
                    "type": "channel",
                    "title": "channeltitle"
                },
                "text": "test message from channel"
        }))
        .unwrap();
        assert_eq!(msg.get_chat_id(), 7);
        assert!(msg.get_chat_username().is_none());
        assert!(msg.get_user().is_none());
    }

    #[test]
    fn get_text() {
        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "caption": "test audio caption",
            "audio": {
                "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
                "file_unique_id": "unique-id",
                "duration": 243
            }
        }))
        .unwrap();
        assert_eq!(msg.get_text().unwrap().data, "test audio caption");

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "caption": "test document caption",
            "document": {
                "file_id": "SSSxmmmsmsIIsooofiiiiaiiaIII_XLA",
                "file_unique_id": "unique-id",
            }
        }))
        .unwrap();
        assert_eq!(msg.get_text().unwrap().data, "test document caption");

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "caption": "test photo caption",
            "photo": [{
                "file_id": "photo-id",
                "file_unique_id": "unique-id",
                "width": 200,
                "height": 200
            }]
        }))
        .unwrap();
        assert_eq!(msg.get_text().unwrap().data, "test photo caption");

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "text"
        }))
        .unwrap();
        assert_eq!(msg.get_text().unwrap().data, "text");

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "caption": "test video caption",
            "video": {
                "file_id": "video-id",
                "file_unique_id": "unique-id",
                "width": 1,
                "height": 2,
                "duration": 3
            }
        }))
        .unwrap();
        assert_eq!(msg.get_text().unwrap().data, "test video caption");

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "caption": "test voice caption",
            "voice": {
                "file_id": "voice-id",
                "file_unique_id": "unique-id",
                "duration": 123
            }
        }))
        .unwrap();
        assert_eq!(msg.get_text().unwrap().data, "test voice caption");

        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "group_chat_created": true
        }))
        .unwrap();
        assert!(msg.get_text().is_none());
    }

    #[test]
    fn commands() {
        let msg: Message = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "/cmd1 /cmd2",
            "entities": [
                {"type": "bot_command", "offset": 0, "length": 5},
                {"type": "bot_command", "offset": 6, "length": 5}
            ]
        }))
        .unwrap();
        let commands = msg.commands.unwrap();
        assert_eq!(commands.len(), 2);
        let cmd1 = &commands[0];
        assert_eq!(cmd1.command, "/cmd1");
        let cmd2 = &commands[1];
        assert_eq!(cmd2.command, "/cmd2");
    }

    #[test]
    fn edit_message_result() {
        let data: EditMessageResult = serde_json::from_value(serde_json::json!({
            "message_id": 1, "date": 1,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "text"
        }))
        .unwrap();
        if let EditMessageResult::Message(msg) = data {
            assert_eq!(msg.id, 1);
        } else {
            panic!("Unexpected message result: {:?}", data);
        }

        let data: EditMessageResult = serde_json::from_value(serde_json::json!(false)).unwrap();
        if let EditMessageResult::Bool(flag) = data {
            assert!(!flag);
        } else {
            panic!("Unexpected message result: {:?}", data);
        }
    }
}
