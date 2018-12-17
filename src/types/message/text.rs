use crate::types::message::raw::{RawMessageEntity, RawMessageEntityKind};
use crate::types::primitive::Integer;
use crate::types::user::User;
use std::string::FromUtf16Error;

/// Text with entities
#[derive(Debug)]
pub struct Text {
    /// The actual UTF-8 text
    pub data: String,
    pub(super) entities: Option<Vec<RawMessageEntity>>,
}

impl Text {
    /// Returns parsed entities
    pub fn parse_entities(&self) -> Result<Vec<TextEntity>, ParseEntitiesError> {
        let raw_entities = match self.entities {
            Some(ref items) => items,
            None => return Err(ParseEntitiesError::NoData),
        };
        let text: Vec<u16> = self.data.encode_utf16().collect();
        let len = text.len() as i64;
        let mut result = Vec::new();
        for raw_entity in raw_entities {
            let (offset, length) = (raw_entity.offset, raw_entity.length);
            if offset > len || offset < 0 {
                return Err(ParseEntitiesError::BadOffset(offset));
            }
            let limit = offset + length;
            if limit > len || limit < 0 {
                return Err(ParseEntitiesError::BadLength(length));
            }
            let (offset, length) = (offset as usize, length as usize);
            let data = String::from_utf16(
                &text
                    .iter()
                    .skip(offset)
                    .take(length)
                    .map(|x| *x)
                    .collect::<Vec<u16>>(),
            )
            .map_err(|e| ParseEntitiesError::FromUtf16(e))?;
            let data = TextEntityData {
                offset,
                length,
                data,
            };
            result.push(match raw_entity.kind {
                RawMessageEntityKind::Bold => TextEntity::Bold(data),
                RawMessageEntityKind::BotCommand => TextEntity::BotCommand(data),
                RawMessageEntityKind::Cashtag => TextEntity::Cashtag(data),
                RawMessageEntityKind::Code => TextEntity::Code(data),
                RawMessageEntityKind::Email => TextEntity::Email(data),
                RawMessageEntityKind::Hashtag => TextEntity::Hashtag(data),
                RawMessageEntityKind::Italic => TextEntity::Italic(data),
                RawMessageEntityKind::Mention => TextEntity::Mention(data),
                RawMessageEntityKind::PhoneNumber => TextEntity::PhoneNumber(data),
                RawMessageEntityKind::Pre => TextEntity::Pre(data),
                RawMessageEntityKind::TextLink => match raw_entity.url {
                    Some(ref url) => TextEntity::TextLink {
                        data,
                        url: url.clone(),
                    },
                    None => return Err(ParseEntitiesError::NoUrl),
                },
                RawMessageEntityKind::TextMention => match raw_entity.user {
                    Some(ref user) => TextEntity::TextMention {
                        data,
                        user: user.clone(),
                    },
                    None => return Err(ParseEntitiesError::NoUser),
                },
                RawMessageEntityKind::Url => TextEntity::Url(data),
            })
        }
        Ok(result)
    }
}

/// Respresents an entity in a text
#[derive(Debug, PartialEq, PartialOrd)]
pub enum TextEntity {
    /// Bold text
    Bold(TextEntityData),
    /// Bot command
    BotCommand(TextEntityData),
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
    TextLink {
        /// Actual data of entity
        data: TextEntityData,
        /// URL that will be opened after user taps on the text
        url: String,
    },
    /// Mention user without username
    TextMention {
        /// Actual data of text entity
        data: TextEntityData,
        /// Mentioned user
        user: User,
    },
    /// URL
    Url(TextEntityData),
}

/// Actual data of text entity
#[derive(Debug, PartialEq, PartialOrd)]
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
pub enum ParseEntitiesError {
    /// Trying to parse empty entities
    #[fail(display = "There are no entities")]
    NoData,
    /// Offset is out of text bounds
    #[fail(display = "Offset {} is out of text bounds", _0)]
    BadOffset(Integer),
    /// Length is out of text bounds
    #[fail(display = "Length {} is out of text bounds", _0)]
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
