use crate::types::{chat::Chat, message::raw::RawMessage, primitive::Integer, user::User};
use serde::{de::Error, Deserialize, Deserializer};

mod data;
mod forward;
mod kind;
mod raw;
#[cfg(test)]
mod tests;
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
        ) {
            (Some(date), Some(user), None, None, None) => Some(Forward {
                date,
                from: ForwardFrom::User(user),
            }),
            (Some(date), None, Some(Chat::Channel(chat)), Some(message_id), signature) => Some(Forward {
                date,
                from: ForwardFrom::Channel {
                    chat,
                    message_id,
                    signature,
                },
            }),
            (None, None, None, None, None) => None,
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

        Err(ParseError::NoData)
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
pub enum EditMessageResult {
    /// Returned if edited message is sent by the bot
    Message(Message),
    /// Returned if edited message is NOT sent by the bot
    Bool(bool),
}

#[derive(Debug, Fail, From)]
enum ParseError {
    #[fail(display = "Unexpected forward_* fields combination")]
    BadForward,
    #[fail(display = "Failed to parse text: {}", _0)]
    BadText(#[cause] ParseTextError),
    #[fail(display = "\"{}\" field is missing", _0)]
    MissingField(&'static str),
    #[fail(display = "Can not get message data")]
    NoData,
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
