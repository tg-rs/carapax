use crate::types::message::raw::{RawMessageEntity, RawMessageEntityKind};
use crate::types::primitive::Integer;
use crate::types::user::User;
use std::string::FromUtf16Error;

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
                    let data = String::from_utf16(
                        &text
                            .iter()
                            .skip(offset)
                            .take(length)
                            .cloned()
                            .collect::<Vec<u16>>(),
                    )
                    .map_err(ParseTextError::FromUtf16)?;
                    let data = TextEntityData {
                        offset,
                        length,
                        data,
                    };
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
    /// Clickable text URLs
    TextLink(TextLink),
    /// Mention user without username
    TextMention(TextMention),
    /// URL
    Url(TextEntityData),
}

impl TextEntity {
    fn from_raw(
        entity: RawMessageEntity,
        data: TextEntityData,
    ) -> Result<TextEntity, ParseTextError> {
        Ok(match entity.kind {
            RawMessageEntityKind::Bold => TextEntity::Bold(data),
            RawMessageEntityKind::BotCommand => {
                let parts = data.data.as_str().splitn(2, '@').collect::<Vec<&str>>();
                let len = parts.len();
                assert!(len >= 1);
                TextEntity::BotCommand(BotCommand {
                    command: parts[0].to_string(),
                    bot_name: if len == 2 {
                        Some(parts[1].to_string())
                    } else {
                        None
                    },
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
            RawMessageEntityKind::TextLink => match entity.url {
                Some(url) => TextEntity::TextLink(TextLink { data, url }),
                None => return Err(ParseTextError::NoUrl),
            },
            RawMessageEntityKind::TextMention => match entity.user {
                Some(user) => TextEntity::TextMention(TextMention { data, user }),
                None => return Err(ParseTextError::NoUser),
            },
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
#[derive(Debug, Fail)]
pub(crate) enum ParseTextError {
    /// Offset is out of text bounds
    #[fail(display = "Offset \"{}\" is out of text bounds", _0)]
    BadOffset(Integer),
    /// Length is out of text bounds
    #[fail(display = "Length \"{}\" is out of text bounds", _0)]
    BadLength(Integer),
    /// URL is required for text_link entity
    #[fail(display = "URL is required for text_link entity")]
    NoUrl,
    /// User is required for text_mention entity
    #[fail(display = "User is required for text_mention entity")]
    NoUser,
    /// Can not get UTF-16 text data
    #[fail(display = "Can not get UTF-16 text data: {}", _0)]
    FromUtf16(#[cause] FromUtf16Error),
}
