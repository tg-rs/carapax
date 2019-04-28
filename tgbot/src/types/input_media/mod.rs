use crate::{
    request::FormValue,
    types::{InputFile, InputFileKind},
};
use failure::Error;
use serde::Serialize;
use std::collections::HashMap;

mod animation;
mod audio;
mod document;
mod photo;
mod video;

pub use self::{animation::*, audio::*, document::*, photo::*, video::*};

/// Content of a media message to be sent
#[derive(Debug, Default)]
pub struct InputMedia {
    fields: HashMap<String, FormValue>,
}

impl InputMedia {
    /// Creates a new input media
    pub fn new<F, K>(file: F, info: K) -> Result<InputMedia, Error>
    where
        F: Into<InputFile>,
        InputMediaKind: From<(String, K)>,
    {
        let mut result = Self::default();
        let file = result.add_file("tgbot_im_file", file.into());
        result.add_info(InputMediaKind::from((file, info)))?;
        Ok(result)
    }

    /// Creates a new input media with thumbnail
    pub fn with_thumb<F, T, K>(file: F, thumb: T, info: K) -> Result<InputMedia, Error>
    where
        F: Into<InputFile>,
        T: Into<InputFile>,
        InputMediaKind: From<(String, String, K)>,
    {
        let mut result = Self::default();
        let file = result.add_file("tgbot_im_file", file.into());
        let thumb = result.add_file("tgbot_im_thumb", thumb.into());
        result.add_info(InputMediaKind::from((file, thumb, info)))?;
        Ok(result)
    }

    fn add_file<S: Into<String>>(&mut self, key: S, file: InputFile) -> String {
        let key = key.into();
        match file.kind {
            InputFileKind::Id(text) | InputFileKind::Url(text) => text,
            _ => {
                self.fields.insert(key.clone(), file.into());
                format!("attach://{}", key)
            }
        }
    }

    fn add_info(&mut self, info: InputMediaKind) -> Result<(), Error> {
        let info = serde_json::to_string(&info)?;
        self.fields.insert(String::from("media"), info.into());
        Ok(())
    }

    pub(crate) fn into_form(self) -> HashMap<String, FormValue> {
        self.fields
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[doc(hidden)]
pub enum InputMediaKind {
    #[serde(rename = "animation")]
    Animation {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<String>,
        #[serde(flatten)]
        info: InputMediaAnimation,
    },
    #[serde(rename = "audio")]
    Audio {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<String>,
        #[serde(flatten)]
        info: InputMediaAudio,
    },
    #[serde(rename = "document")]
    Document {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<String>,
        #[serde(flatten)]
        info: InputMediaDocument,
    },
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

macro_rules! convert_media_kind {
    (
        $($to:ident(thumb $from:ty)),*
    ) => {
        $(
            impl From<(String, $from)> for InputMediaKind {
                fn from((media, info): (String, $from)) -> Self {
                    InputMediaKind::$to {
                        media,
                        info,
                        thumb: None,
                    }
                }
            }

            impl From<(String, String, $from)> for InputMediaKind {
                fn from((media, thumb, info): (String, String, $from)) -> Self {
                    InputMediaKind::$to {
                        media,
                        info,
                        thumb: Some(thumb),
                    }
                }
            }
        )*
    };
    (
        $($to:ident($from:ty)),*
    ) => {
        $(
            impl From<(String, $from)> for InputMediaKind {
                fn from((media, info): (String, $from)) -> Self {
                    InputMediaKind::$to {
                        media,
                        info,
                    }
                }
            }
        )*
    };
}

convert_media_kind!(
    Animation(thumb InputMediaAnimation),
    Audio(thumb InputMediaAudio),
    Document(thumb InputMediaDocument),
    Video(thumb InputMediaVideo)
);

convert_media_kind!(Photo(InputMediaPhoto));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::InputFileReader;
    use std::io::Cursor;

    #[test]
    fn input_media() {
        let data = InputMedia::new(
            InputFile::file_id("animation-file-id"),
            InputMediaAnimation::default().caption("test"),
        )
        .unwrap()
        .into_form();
        assert!(data.get("media").is_some());

        let data = InputMedia::with_thumb(
            InputFileReader::from(Cursor::new("animation-file-data")),
            InputFileReader::from(Cursor::new("animation-thumb-data")),
            InputMediaAnimation::default(),
        )
        .unwrap()
        .into_form();
        assert!(data.get("tgbot_im_file").is_some());
        assert!(data.get("tgbot_im_thumb").is_some());
        assert!(data.get("media").is_some());
    }

    #[test]
    fn input_media_kind() {
        assert_eq!(
            serde_json::to_value(InputMediaKind::from((
                String::from("file-id"),
                InputMediaAnimation::default().caption("test")
            )))
            .unwrap(),
            serde_json::json!({
                "type": "animation",
                "media": "file-id",
                "caption": "test"
            })
        );
        assert_eq!(
            serde_json::to_value(InputMediaKind::from((
                String::from("file-id"),
                String::from("thumb-id"),
                InputMediaAnimation::default().caption("test"),
            )))
            .unwrap(),
            serde_json::json!({
                "type": "animation",
                "media": "file-id",
                "thumb": "thumb-id",
                "caption": "test"
            })
        );

        assert_eq!(
            serde_json::to_value(InputMediaKind::from((
                String::from("file-id"),
                InputMediaAudio::default().caption("test")
            )))
            .unwrap(),
            serde_json::json!({
                "type": "audio",
                "media": "file-id",
                "caption": "test"
            })
        );
        assert_eq!(
            serde_json::to_value(InputMediaKind::from((
                String::from("file-id"),
                String::from("thumb-id"),
                InputMediaAudio::default().caption("test"),
            )))
            .unwrap(),
            serde_json::json!({
                "type": "audio",
                "media": "file-id",
                "thumb": "thumb-id",
                "caption": "test"
            })
        );

        assert_eq!(
            serde_json::to_value(InputMediaKind::from((
                String::from("file-id"),
                InputMediaDocument::default().caption("test")
            )))
            .unwrap(),
            serde_json::json!({
                "type": "document",
                "media": "file-id",
                "caption": "test"
            })
        );
        assert_eq!(
            serde_json::to_value(InputMediaKind::from((
                String::from("file-id"),
                String::from("thumb-id"),
                InputMediaDocument::default().caption("test"),
            )))
            .unwrap(),
            serde_json::json!({
                "type": "document",
                "media": "file-id",
                "thumb": "thumb-id",
                "caption": "test"
            })
        );

        assert_eq!(
            serde_json::to_value(InputMediaKind::from((
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
            serde_json::to_value(InputMediaKind::from((
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
            serde_json::to_value(InputMediaKind::from((
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
