use crate::types::chat::Chat;
use crate::types::message::raw::RawMessage;
use crate::types::primitive::Integer;
use serde::de::{Deserialize, Deserializer, Error};

mod data;
mod forward;
mod kind;
mod raw;
#[cfg(test)]
mod tests;
mod text;

pub use self::data::*;
pub use self::forward::*;
pub use self::kind::*;
pub(crate) use self::raw::RawMessageEntity;
pub use self::text::*;

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
