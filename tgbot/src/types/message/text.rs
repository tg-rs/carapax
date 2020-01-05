use crate::types::{
    message::raw::{RawMessageEntity, RawMessageEntityKind},
    primitive::Integer,
    user::User,
};
use std::{error::Error as StdError, fmt, string::FromUtf16Error};

/// Text with entities
#[derive(Clone, Debug)]
pub struct Text {
    /// The actual UTF-8 text
    pub data: String,
    /// Text entities
    pub entities: Option<Vec<TextEntity>>,
}

impl Text {
    pub(crate) fn parse<S: Into<String>>(
        data: S,
        entities: Option<Vec<RawMessageEntity>>,
    ) -> Result<Text, ParseTextError> {
        let data = data.into();
        let entities = if let Some(entities) = entities {
            if entities.is_empty() {
                None
            } else {
                let text: Vec<u16> = data.encode_utf16().collect();
                let len = text.len() as i64;
                let mut result = Vec::new();
                for entity in entities {
                    let (offset, length) = (entity.offset, entity.length);
                    if offset > len || offset < 0 {
                        return Err(ParseTextError::BadOffset(offset));
                    }
                    let limit = offset + length;
                    if limit > len || limit < 0 {
                        return Err(ParseTextError::BadLength(length));
                    }
                    let (offset, length) = (offset as usize, length as usize);
                    let data =
                        String::from_utf16(&text.iter().skip(offset).take(length).cloned().collect::<Vec<u16>>())
                            .map_err(ParseTextError::FromUtf16)?;
                    let data = TextEntityData { offset, length, data };
                    result.push(TextEntity::from_raw(entity, data)?)
                }
                Some(result)
            }
        } else {
            None
        };
        Ok(Text { data, entities })
    }
}

/// Respresents an entity in a text
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum TextEntity {
    /// Bold text
    Bold(TextEntityData),
    /// Bot command
    BotCommand(BotCommand),
    /// Cashtag
    Cashtag(TextEntityData),
    /// Monowidth string
    Code(TextEntityData),
    /// E-Mail
    Email(TextEntityData),
    /// Hashtag
    Hashtag(TextEntityData),
    /// Italic text
    Italic(TextEntityData),
    /// User mention (e.g. @username)
    Mention(TextEntityData),
    /// Phone number
    PhoneNumber(TextEntityData),
    /// Monowidth block
    Pre(TextEntityData),
    /// Strikethrough text
    Strikethrough(TextEntityData),
    /// Clickable text URLs
    TextLink(TextLink),
    /// Mention user without username
    TextMention(TextMention),
    /// Underlined text
    Underline(TextEntityData),
    /// URL
    Url(TextEntityData),
}

impl TextEntity {
    fn from_raw(entity: RawMessageEntity, data: TextEntityData) -> Result<TextEntity, ParseTextError> {
        Ok(match entity.kind {
            RawMessageEntityKind::Bold => TextEntity::Bold(data),
            RawMessageEntityKind::BotCommand => {
                let parts = data.data.as_str().splitn(2, '@').collect::<Vec<&str>>();
                let len = parts.len();
                assert!(len >= 1);
                TextEntity::BotCommand(BotCommand {
                    command: parts[0].to_string(),
                    bot_name: if len == 2 { Some(parts[1].to_string()) } else { None },
                    data,
                })
            }
            RawMessageEntityKind::Cashtag => TextEntity::Cashtag(data),
            RawMessageEntityKind::Code => TextEntity::Code(data),
            RawMessageEntityKind::Email => TextEntity::Email(data),
            RawMessageEntityKind::Hashtag => TextEntity::Hashtag(data),
            RawMessageEntityKind::Italic => TextEntity::Italic(data),
            RawMessageEntityKind::Mention => TextEntity::Mention(data),
            RawMessageEntityKind::PhoneNumber => TextEntity::PhoneNumber(data),
            RawMessageEntityKind::Pre => TextEntity::Pre(data),
            RawMessageEntityKind::Strikethrough => TextEntity::Strikethrough(data),
            RawMessageEntityKind::TextLink => match entity.url {
                Some(url) => TextEntity::TextLink(TextLink { data, url }),
                None => return Err(ParseTextError::NoUrl),
            },
            RawMessageEntityKind::TextMention => match entity.user {
                Some(user) => TextEntity::TextMention(TextMention { data, user }),
                None => return Err(ParseTextError::NoUser),
            },
            RawMessageEntityKind::Underline => TextEntity::Underline(data),
            RawMessageEntityKind::Url => TextEntity::Url(data),
        })
    }
}

/// Bot command
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BotCommand {
    /// Actual command
    pub command: String,
    /// Bot's username
    pub bot_name: Option<String>,
    /// Entity data
    pub data: TextEntityData,
}

/// Clickable text URLs
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TextLink {
    /// Actual data of entity
    pub data: TextEntityData,
    /// URL that will be opened after user taps on the text
    pub url: String,
}

/// Mention user without username
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TextMention {
    /// Actual data of text entity
    pub data: TextEntityData,
    /// Mentioned user
    pub user: User,
}

/// Actual data of text entity
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TextEntityData {
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: usize,
    /// Length of the entity in UTF-16 code units
    pub length: usize,
    /// Data of the entity from text
    pub data: String,
}

/// An error when parsing entities
#[derive(Debug)]
pub(crate) enum ParseTextError {
    /// Offset is out of text bounds
    BadOffset(Integer),
    /// Length is out of text bounds
    BadLength(Integer),
    /// URL is required for text_link entity
    NoUrl,
    /// User is required for text_mention entity
    NoUser,
    /// Can not get UTF-16 text data
    FromUtf16(FromUtf16Error),
}

impl StdError for ParseTextError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            ParseTextError::FromUtf16(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for ParseTextError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseTextError::*;
        match self {
            BadOffset(offset) => write!(out, "offset \"{}\" is out of text bounds", offset),
            BadLength(length) => write!(out, "length \"{}\" is out of text bounds", length),
            NoUrl => write!(out, "URL is required for text_link entity"),
            NoUser => write!(out, "user is required for text_mention entity"),
            FromUtf16(err) => write!(out, "can not get UTF-16 text data: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Message, MessageData, User};
    use serde_json::json;

    #[test]
    fn deserialize_message_entities() {
        let input = json!({
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
            "text": "bold /botcommand $cashtag code u@h.z #hashtag italic @mention phone pre textlink textmention url underline strikethrough",
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
                {"type": "url", "offset": 93, "length": 3},
                {"type": "underline", "offset": 97, "length": 9},
                {"type": "strikethrough", "offset": 107, "length": 13}
            ]
        });
        let msg: Message = serde_json::from_value(input).unwrap();
        assert_eq!(msg.commands.unwrap().len(), 1);
        if let MessageData::Text(text) = msg.data {
            let entities = text.entities.unwrap();
            assert_eq!(
                vec![
                    TextEntity::Bold(TextEntityData {
                        data: String::from("bold"),
                        offset: 0,
                        length: 4
                    }),
                    TextEntity::BotCommand(BotCommand {
                        command: String::from("/botcommand"),
                        bot_name: None,
                        data: TextEntityData {
                            data: String::from("/botcommand"),
                            offset: 5,
                            length: 11
                        }
                    }),
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
                    TextEntity::TextLink(TextLink {
                        data: TextEntityData {
                            data: String::from("textlink"),
                            offset: 72,
                            length: 8
                        },
                        url: String::from("https://example.com")
                    }),
                    TextEntity::TextMention(TextMention {
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
                    }),
                    TextEntity::Url(TextEntityData {
                        data: String::from("url"),
                        offset: 93,
                        length: 3
                    }),
                    TextEntity::Underline(TextEntityData {
                        data: String::from("underline"),
                        offset: 97,
                        length: 9
                    }),
                    TextEntity::Strikethrough(TextEntityData {
                        data: String::from("strikethrough"),
                        offset: 107,
                        length: 13
                    })
                ],
                entities
            );
        } else {
            panic!("Unexpected message data: {:?}", msg.data);
        }
    }

    #[test]
    fn deserialize_message_bad_entities() {
        for (input, error) in vec![
            (
                json!({
                    "message_id": 1, "date": 0,
                    "from": {"id": 1, "first_name": "firstname", "is_bot": false},
                    "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
                    "text": "bad offset",
                    "entities": [
                        {
                            "type": "bold",
                            "offset": -1,
                            "length": 1
                        }
                    ]
                }),
                "failed to parse text: offset \"-1\" is out of text bounds",
            ),
            (
                json!({
                    "message_id": 1, "date": 0,
                    "from": {"id": 1, "first_name": "firstname", "is_bot": false},
                    "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
                    "text": "bad offset",
                    "entities": [
                        {
                            "type": "bold",
                            "offset": 11,
                            "length": 1
                        }
                    ]
                }),
                "failed to parse text: offset \"11\" is out of text bounds",
            ),
            (
                json!({
                    "message_id": 1, "date": 0,
                    "from": {"id": 1, "first_name": "firstname", "is_bot": false},
                    "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
                    "text": "bad offset",
                    "entities": [
                        {
                            "type": "bold",
                            "offset": 0,
                            "length": -1
                        }
                    ]
                }),
                "failed to parse text: length \"-1\" is out of text bounds",
            ),
            (
                json!({
                    "message_id": 1, "date": 0,
                    "from": {"id": 1, "first_name": "firstname", "is_bot": false},
                    "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
                    "text": "bad offset",
                    "entities": [
                        {
                            "type": "bold",
                            "offset": 0,
                            "length": 11
                        }
                    ]
                }),
                "failed to parse text: length \"11\" is out of text bounds",
            ),
            (
                json!({
                    "message_id": 1, "date": 0,
                    "from": {"id": 1, "first_name": "firstname", "is_bot": false},
                    "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
                    "text": "bad offset",
                    "entities": [
                        {
                            "type": "text_link",
                            "offset": 0,
                            "length": 2
                        }
                    ]
                }),
                "failed to parse text: URL is required for text_link entity",
            ),
            (
                json!({
                    "message_id": 1, "date": 0,
                    "from": {"id": 1, "first_name": "firstname", "is_bot": false},
                    "chat": {"id": 1, "type": "supergroup", "title": "supergrouptitle"},
                    "text": "bad offset",
                    "entities": [
                        {
                            "type": "text_mention",
                            "offset": 0,
                            "length": 2
                        }
                    ]
                }),
                "failed to parse text: user is required for text_mention entity",
            ),
        ] {
            let err = serde_json::from_value::<Message>(input).unwrap_err();
            assert_eq!(err.to_string(), error.to_string());
        }
    }
}
