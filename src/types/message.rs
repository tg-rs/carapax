use crate::types::chat::{ChannelChat, Chat, GroupChat, PrivateChat, SupergroupChat};
use crate::types::contact::Contact;
use crate::types::games::Game;
use crate::types::location::Location;
use crate::types::media::{
    Animation, Audio, Document, PhotoSize, Sticker, Venue, Video, VideoNote, Voice,
};
use crate::types::passport::PassportData;
use crate::types::payments::{Invoice, SuccessfulPayment};
use crate::types::primitive::Integer;
use crate::types::user::User;
use serde::de::{Deserialize, Deserializer, Error};

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

/// Contains chat-specific data
#[derive(Debug)]
pub enum MessageKind {
    /// Channel chat
    Channel {
        /// Channel chat
        chat: ChannelChat,
        /// Author signature, if exists
        author_signature: Option<String>,
    },
    /// Group chat
    Group {
        /// Group chat
        chat: GroupChat,
        /// Sender
        from: User,
    },
    /// Private chat
    Private {
        /// Private chat
        chat: PrivateChat,
        /// Sender
        from: User,
    },
    /// Supergroup chat
    Supergroup {
        /// Supergroup chat
        chat: SupergroupChat,
        /// Sender
        from: User,
    },
}

/// Contains information about original message
#[derive(Debug)]
pub struct Forward {
    /// Sender of the original message
    pub from: ForwardFrom,
    /// Date the original message was sent in Unix time
    pub date: Integer,
}

/// Sender of the original message
#[derive(Debug)]
pub enum ForwardFrom {
    /// Information about user
    User(User),
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

/// Contains message data
#[derive(Debug)]
pub enum MessageData {
    /// Message is an animation, information about the animation
    Animation(Animation),
    /// Audio message
    Audio {
        /// Audio caption
        caption: Option<Caption>,
        /// Audio data
        data: Audio,
    },
    /// Service message: the channel has been created.
    /// This field can‘t be received in a message coming through updates,
    /// because bot can’t be a member of a channel when it is created.
    /// It can only be found in reply_to_message if someone replies to a very first message in a channel.
    ChannelChatCreated,
    /// The domain name of the website on which the user has logged in
    ConnectedWebsite(String),
    /// Message is a shared contact, information about the contact
    Contact(Contact),
    /// Service message: the chat photo was deleted
    DeleteChatPhoto,
    /// Document message
    Document {
        /// Document caption
        caption: Option<Caption>,
        /// Document data
        data: Document,
    },
    /// Message is a game, information about the game
    Game(Game),
    /// Service message: the group has been created
    GroupChatCreated,
    /// Message is an invoice for a payment, information about the invoice
    Invoice(Invoice),
    /// A member was removed from the group
    /// (this member may be the bot itself)
    LeftChatMember(User),
    /// Message is a shared location, information about the location
    Location(Location),
    /// The supergroup has been migrated from a group with the specified identifier
    MigrateFromChatId(Integer),
    /// The group has been migrated to a supergroup with the specified identifier
    MigrateToChatId(Integer),
    /// New members that were added to the group or supergroup
    /// (the bot itself may be one of these members)
    NewChatMembers(Vec<User>),
    /// A chat photo was change to this value
    NewChatPhoto(Vec<PhotoSize>),
    /// A chat title was changed to this value
    NewChatTitle(String),
    /// Telegram Passport data
    PassportData(PassportData),
    /// Specified message was pinned
    /// Note that the Message object in this field will not contain
    /// further reply_to_message fields even if it is itself a reply
    PinnedMessage(Box<Message>),
    /// Message is a photo, available sizes of the photo
    Photo {
        /// Photo caption
        caption: Option<Caption>,
        /// Photos
        data: Vec<PhotoSize>,
    },
    /// Message is a sticker, information about the sticker
    Sticker(Sticker),
    /// Message is a service message about a successful payment, information about the payment
    SuccessfulPayment(SuccessfulPayment),
    /// Service message: the supergroup has been created.
    /// This field can‘t be received in a message coming through updates,
    /// because bot can’t be a member of a supergroup when it is created.
    /// It can only be found in reply_to_message if someone replies to a very first message
    /// in a directly created supergroup.
    SupergroupChatCreated,
    /// Text message
    Text {
        /// The actual UTF-8 text of the message, 0-4096 characters
        data: String,
        /// Special entities like usernames, URLs, bot commands, etc. that appear in the text
        entities: Option<Vec<MessageEntity>>,
    },
    /// Message is a venue, information about the venue
    Venue(Venue),
    /// Message is a video, information about the video
    Video {
        /// Video caption
        caption: Option<Caption>,
        /// Video data
        data: Video,
    },
    /// Message is a video note, information about the video message
    VideoNote(VideoNote),
    /// Message is a voice message, information about the file
    Voice {
        /// Voice caption
        caption: Option<Caption>,
        /// Voice data
        data: Voice,
    },
}

/// Caption for a media
#[derive(Debug)]
pub struct Caption {
    /// Caption for the audio, document, photo, video or voice, 0-1024 characters
    text: String,
    /// Special entities like usernames, URLs, bot commands, etc. that appear in the caption
    entities: Option<Vec<MessageEntity>>,
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
            Some(text) => Some(Caption {
                text,
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
            MessageData::Text {
                data,
                entities: raw.entities,
            }
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

#[derive(Debug, Deserialize)]
struct RawMessage {
    message_id: Integer,
    from: Option<User>,
    date: Integer,
    chat: Chat,
    forward_from: Option<User>,
    forward_from_chat: Option<Chat>,
    forward_from_message_id: Option<Integer>,
    forward_signature: Option<String>,
    forward_date: Option<Integer>,
    reply_to_message: Option<Box<RawMessage>>,
    edit_date: Option<Integer>,
    media_group_id: Option<String>,
    author_signature: Option<String>,
    text: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    caption_entities: Option<Vec<MessageEntity>>,
    audio: Option<Audio>,
    animation: Option<Animation>,
    document: Option<Document>,
    game: Option<Game>,
    photo: Option<Vec<PhotoSize>>,
    sticker: Option<Sticker>,
    video: Option<Video>,
    voice: Option<Voice>,
    video_note: Option<VideoNote>,
    caption: Option<String>,
    contact: Option<Contact>,
    location: Option<Location>,
    venue: Option<Venue>,
    new_chat_members: Option<Vec<User>>,
    left_chat_member: Option<User>,
    new_chat_title: Option<String>,
    new_chat_photo: Option<Vec<PhotoSize>>,
    delete_chat_photo: Option<bool>,
    group_chat_created: Option<bool>,
    supergroup_chat_created: Option<bool>,
    channel_chat_created: Option<bool>,
    migrate_to_chat_id: Option<Integer>,
    migrate_from_chat_id: Option<Integer>,
    pinned_message: Option<Box<RawMessage>>,
    invoice: Option<Invoice>,
    successful_payment: Option<SuccessfulPayment>,
    connected_website: Option<String>,
    passport_data: Option<PassportData>,
}

/// This object represents one special entity in a text message
/// For example, hashtags, usernames, URLs, etc
#[derive(Debug, Deserialize)]
pub struct MessageEntity {
    /// Type of the entity
    #[serde(rename = "type")]
    pub kind: MessageEntityKind,
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: Integer,
    /// Length of the entity in UTF-16 code units
    pub length: Integer,
    /// For “text_link” only, url that will be opened after user taps on the text
    pub url: Option<String>,
    /// For “text_mention” only, the mentioned user
    pub user: Option<User>,
}

/// Type of the message entity
#[derive(Debug, Deserialize)]
pub enum MessageEntityKind {
    /// Bold text
    #[serde(rename = "bold")]
    Bold,
    /// Bot command
    #[serde(rename = "bot_command")]
    BotCommand,
    /// Cashtag
    #[serde(rename = "cashtag")]
    Cashtag,
    /// Monowidth string
    #[serde(rename = "code")]
    Code,
    /// E-Mail
    #[serde(rename = "email")]
    Email,
    /// Hashtag
    #[serde(rename = "hashtag")]
    Hashtag,
    /// Italic text
    #[serde(rename = "italic")]
    Italic,
    /// User mention (e.g. @username)
    #[serde(rename = "mention")]
    Mention,
    /// Phone number
    #[serde(rename = "phone_number")]
    PhoneNumber,
    /// Monowidth block
    #[serde(rename = "pre")]
    Pre,
    /// Clickable text URLs
    #[serde(rename = "text_link")]
    TextLink,
    /// Mention user without username
    #[serde(rename = "text_mention")]
    TextMention,
    /// URL
    #[serde(rename = "url")]
    Url,
}

#[cfg(test)]
mod tests {
    use super::*;

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
        if let MessageData::Text { data, entities } = msg.data {
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
        if let MessageData::Text { data, entities } = msg.data {
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
        if let MessageData::Text { data, entities } = msg.data {
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
        if let MessageData::Text { data, entities } = msg.data {
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
}
