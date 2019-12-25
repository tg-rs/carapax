use crate::{
    request::FormValue,
    types::{InputFile, InputFileKind, InputMediaPhoto, InputMediaVideo},
};
use serde::Serialize;
use serde_json::Error as JsonError;
use std::{collections::HashMap, error::Error as StdError, fmt};

const MIN_GROUP_ATTACHMENTS: usize = 2;
const MAX_GROUP_ATTACHMENTS: usize = 10;

/// Group of photos and/or videos to be sent
#[derive(Debug, Default)]
pub struct MediaGroup {
    files: HashMap<String, InputFile>,
    items: Vec<MediaGroupItem>,
}

impl MediaGroup {
    /// Adds a photo or video to group
    pub fn add_item<I, F>(mut self, file: F, info: I) -> Self
    where
        MediaGroupItem: From<(String, I)>,
        F: Into<InputFile>,
    {
        let file = self.add_file(file.into());
        self.items.push(MediaGroupItem::from((file, info)));
        self
    }

    /// Adds an item with thumbnail
    pub fn add_item_with_thumb<F, T, I>(mut self, file: F, thumb: T, info: I) -> Self
    where
        F: Into<InputFile>,
        T: Into<InputFile>,
        MediaGroupItem: From<(String, String, I)>,
    {
        let file = self.add_file(file.into());
        let thumb = self.add_file(thumb.into());
        self.items.push(MediaGroupItem::from((file, thumb, info)));
        self
    }

    fn add_file(&mut self, file: InputFile) -> String {
        match &file.kind {
            InputFileKind::Id(text) | InputFileKind::Url(text) => text.clone(),
            _ => {
                let idx = self.files.len();
                let key = format!("tgbot_im_file_{}", idx);
                self.files.insert(key.clone(), file);
                format!("attach://{}", key)
            }
        }
    }

    pub(crate) fn into_form(self) -> Result<HashMap<String, FormValue>, MediaGroupError> {
        let total_files = self.items.len();
        if total_files < MIN_GROUP_ATTACHMENTS {
            return Err(MediaGroupError::NotEnoughAttachments(MIN_GROUP_ATTACHMENTS));
        }
        if total_files > MAX_GROUP_ATTACHMENTS {
            return Err(MediaGroupError::TooManyAttachments(MAX_GROUP_ATTACHMENTS));
        }
        let mut fields: HashMap<String, FormValue> = self.files.into_iter().map(|(k, v)| (k, v.into())).collect();
        fields.insert(
            String::from("media"),
            serde_json::to_string(&self.items)
                .map_err(MediaGroupError::Serialize)?
                .into(),
        );
        Ok(fields)
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[doc(hidden)]
pub enum MediaGroupItem {
    #[serde(rename = "photo")]
    Photo {
        media: String,
        #[serde(flatten)]
        info: InputMediaPhoto,
    },
    #[serde(rename = "video")]
    Video {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<String>,
        #[serde(flatten)]
        info: InputMediaVideo,
    },
}

impl From<(String, InputMediaPhoto)> for MediaGroupItem {
    fn from((media, info): (String, InputMediaPhoto)) -> Self {
        MediaGroupItem::Photo { media, info }
    }
}

impl From<(String, InputMediaVideo)> for MediaGroupItem {
    fn from((media, info): (String, InputMediaVideo)) -> Self {
        MediaGroupItem::Video {
            media,
            info,
            thumb: None,
        }
    }
}

impl From<(String, String, InputMediaVideo)> for MediaGroupItem {
    fn from((media, thumb, info): (String, String, InputMediaVideo)) -> Self {
        MediaGroupItem::Video {
            media,
            info,
            thumb: Some(thumb),
        }
    }
}

/// A media group error
#[derive(Debug)]
pub enum MediaGroupError {
    /// Media group contains not enough files
    NotEnoughAttachments(usize),
    /// Media group contains too many files
    TooManyAttachments(usize),
    /// Can not serialize items
    Serialize(JsonError),
}

impl StdError for MediaGroupError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            MediaGroupError::Serialize(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for MediaGroupError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MediaGroupError::NotEnoughAttachments(number) => {
                write!(out, "media group must contain at least {} attachments", number)
            }
            MediaGroupError::TooManyAttachments(number) => {
                write!(out, "media group must contain no more than {} attachments", number)
            }
            MediaGroupError::Serialize(err) => write!(out, "can not serialize media group items: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::InputFileReader;
    use std::io::Cursor;

    #[test]
    fn media_group() {
        let group = MediaGroup::default()
            .add_item(InputFileReader::from(Cursor::new("test")), InputMediaPhoto::default())
            .add_item(InputFileReader::from(Cursor::new("test")), InputMediaVideo::default())
            .add_item_with_thumb(
                InputFile::file_id("file-id"),
                InputFile::url("thumb-url"),
                InputMediaVideo::default(),
            )
            .into_form()
            .unwrap();
        assert!(group.get("media").is_some());
        assert!(group.get("tgbot_im_file_0").is_some());
        assert!(group.get("tgbot_im_file_1").is_some());

        let err = MediaGroup::default().into_form().unwrap_err();
        assert_eq!(err.to_string(), "media group must contain at least 2 attachments");

        let mut group = MediaGroup::default();
        for _ in 0..11 {
            group = group.add_item(InputFile::file_id("file-id"), InputMediaPhoto::default());
        }
        let err = group.into_form().unwrap_err();
        assert_eq!(err.to_string(), "media group must contain no more than 10 attachments");
    }

    #[test]
    fn media_group_item() {
        assert_eq!(
            serde_json::to_value(MediaGroupItem::from((
                String::from("file-id"),
                String::from("thumb-id"),
                InputMediaVideo::default().caption("test"),
            )))
            .unwrap(),
            serde_json::json!({
                "type": "video",
                "media": "file-id",
                "thumb": "thumb-id",
                "caption": "test"
            })
        );
        assert_eq!(
            serde_json::to_value(MediaGroupItem::from((
                String::from("file-id"),
                InputMediaVideo::default().caption("test")
            )))
            .unwrap(),
            serde_json::json!({
                "type": "video",
                "media": "file-id",
                "caption": "test"
            })
        );
        assert_eq!(
            serde_json::to_value(MediaGroupItem::from((
                String::from("file-id"),
                InputMediaPhoto::default().caption("test")
            )))
            .unwrap(),
            serde_json::json!({
                "type": "photo",
                "media": "file-id",
                "caption": "test"
            })
        );
    }
}
