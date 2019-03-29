use crate::{
    request::FormValue,
    types::{InputFile, InputFileKind, Integer, ParseMode},
};
use failure::{Error, Fail};
use serde::Serialize;
use std::collections::HashMap;

const MIN_GROUP_ATTACHMENTS: usize = 2;
const MAX_GROUP_ATTACHMENTS: usize = 10;

/// A media group error
#[derive(Debug, Fail)]
pub enum MediaGroupError {
    /// Media group contains not enough files
    #[fail(display = "Media group must contain at least {} attachments", _0)]
    NotEnoughAttachments(usize),
    /// Media group contains too many files
    #[fail(display = "Media group must contain no more than {} attachments", _0)]
    TooManyAttachments(usize),
}

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

    pub(crate) fn into_form(self) -> Result<HashMap<String, FormValue>, Error> {
        let total_files = self.items.len();
        if total_files < MIN_GROUP_ATTACHMENTS {
            return Err(MediaGroupError::NotEnoughAttachments(MIN_GROUP_ATTACHMENTS).into());
        }
        if total_files > MAX_GROUP_ATTACHMENTS {
            return Err(MediaGroupError::TooManyAttachments(MAX_GROUP_ATTACHMENTS).into());
        }
        let mut fields: HashMap<String, FormValue> = self.files.into_iter().map(|(k, v)| (k, v.into())).collect();
        fields.insert(String::from("media"), serde_json::to_string(&self.items)?.into());
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

/// Animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent
#[derive(Clone, Default, Debug, Serialize)]
pub struct InputMediaAnimation {
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
}

impl InputMediaAnimation {
    /// Set a thumbnail
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size
    /// A thumbnail‘s width and height should not exceed 90
    /// Ignored if the file is not uploaded using multipart/form-data
    /// Thumbnails can’t be reused and can be only uploaded
    /// as a new file, so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data
    /// under <file_attach_name>
    pub fn thumb<S: Into<String>>(mut self, thumb: S) -> Self {
        self.thumb = Some(thumb.into());
        self
    }

    /// Caption of the animation to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Set width
    pub fn width(mut self, width: Integer) -> Self {
        self.width = Some(width);
        self
    }

    /// Set height
    pub fn height(mut self, height: Integer) -> Self {
        self.height = Some(height);
        self
    }

    /// Set duration
    pub fn duration(mut self, duration: Integer) -> Self {
        self.duration = Some(duration);
        self
    }
}

/// Audio file to be treated as music to be sent
#[derive(Clone, Default, Debug, Serialize)]
pub struct InputMediaAudio {
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

impl InputMediaAudio {
    /// Caption of the audio to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Set duration
    pub fn duration(mut self, duration: Integer) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Performer of the audio
    pub fn performer<S: Into<String>>(mut self, performer: S) -> Self {
        self.performer = Some(performer.into());
        self
    }

    /// Title of the audio
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }
}

/// General file to be sent
#[derive(Clone, Default, Debug, Serialize)]
pub struct InputMediaDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
}

impl InputMediaDocument {
    /// Caption of the document to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
}

/// Photo to be sent
#[derive(Clone, Default, Debug, Serialize)]
pub struct InputMediaPhoto {
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
}

impl InputMediaPhoto {
    /// Caption of the photo to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
}

/// Video to be sent
#[derive(Clone, Default, Debug, Serialize)]
pub struct InputMediaVideo {
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_streaming: Option<bool>,
}

impl InputMediaVideo {
    /// Caption of the video to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Set width
    pub fn width(mut self, width: Integer) -> Self {
        self.width = Some(width);
        self
    }

    /// Set height
    pub fn height(mut self, height: Integer) -> Self {
        self.height = Some(height);
        self
    }

    /// Set duration
    pub fn duration(mut self, duration: Integer) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Pass True, if the uploaded video is suitable for streaming
    pub fn supports_streaming(mut self, supports_streaming: bool) -> Self {
        self.supports_streaming = Some(supports_streaming);
        self
    }
}
