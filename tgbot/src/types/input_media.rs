use crate::{
    request::FormValue,
    types::{InputFile, InputFileKind, Integer, ParseMode},
};
use failure::{Error, Fail};
use serde::Serialize;
use std::collections::HashMap;

const MIN_GROUP_FILES: usize = 2;
const MAX_GROUP_FILES: usize = 10;

/// A media group error
#[derive(Debug, Fail)]
pub enum MediaGroupError {
    /// Media group contains not enough files
    #[fail(display = "Media group must contain at least {} files", _0)]
    NotEnoughFiles(usize),
    /// Media group contains too many files
    #[fail(display = "Media group must contain no more than {} files", _0)]
    TooManyFiles(usize),
}

/// Group of photos and/or videos to be sent
#[derive(Debug, Default)]
pub struct MediaGroup {
    pub(crate) files: HashMap<String, InputFile>,
    pub(crate) photos: Vec<InputMediaPhoto>,
    pub(crate) videos: Vec<InputMediaVideo>,
}

impl MediaGroup {
    /// Adds a photo to group
    ///
    /// Use returned value to set a caption and other attributes of photo
    pub fn add_photo(&mut self, photo: InputFile) -> &mut InputMediaPhoto {
        let info = match &photo.kind {
            InputFileKind::Id(photo) | InputFileKind::Url(photo) => InputMediaPhoto::new(photo.clone()),
            _ => {
                let idx = self.files.len();
                let key = format!("tgbot_im_photo_{}", idx);
                self.files.insert(key.clone(), photo);
                InputMediaPhoto::new(format!("attach://{}", key))
            }
        };
        self.photos.push(info);
        self.photos.last_mut().unwrap()
    }

    /// Adds a video to group
    ///
    /// Use returned value to set a caption and other attributes of video
    pub fn add_video(&mut self, video: InputFile) -> &mut InputMediaVideo {
        let info = match &video.kind {
            InputFileKind::Id(video) | InputFileKind::Url(video) => InputMediaVideo::new(video.clone()),
            _ => {
                let idx = self.files.len();
                let key = format!("tgbot_im_video_{}", idx);
                self.files.insert(key.clone(), video);
                InputMediaVideo::new(format!("attach://{}", key))
            }
        };
        self.videos.push(info);
        self.videos.last_mut().unwrap()
    }

    pub(crate) fn into_form(self) -> Result<HashMap<String, FormValue>, Error> {
        let total_files = self.photos.len() + self.videos.len();
        if total_files < MIN_GROUP_FILES {
            return Err(MediaGroupError::NotEnoughFiles(MIN_GROUP_FILES).into());
        }
        if total_files > MAX_GROUP_FILES {
            return Err(MediaGroupError::TooManyFiles(MAX_GROUP_FILES).into());
        }
        let mut fields: HashMap<String, FormValue> = self.files.into_iter().map(|(k, v)| (k, v.into())).collect();
        let mut media: Vec<MediaGroupItem> = self.photos.into_iter().map(|x| x.into()).collect();
        media.extend(self.videos.into_iter().map(|x| x.into()));
        fields.insert(String::from("media"), serde_json::to_string(&media)?.into());
        Ok(fields)
    }
}

#[derive(Debug, derive_more::From, Serialize)]
#[serde(tag = "type")]
enum MediaGroupItem {
    #[serde(rename = "photo")]
    Photo(InputMediaPhoto),
    #[serde(rename = "video")]
    Video(InputMediaVideo),
}

/// Content of a media message to be sent
#[derive(Clone, Debug, derive_more::From, Serialize)]
#[serde(tag = "type")]
pub enum InputMedia {
    /// Animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent
    #[serde(rename = "animation")]
    Animation(InputMediaAnimation),
    /// Audio file to be treated as music to be sent
    #[serde(rename = "audio")]
    Audio(InputMediaAudio),
    /// General file to be sent
    #[serde(rename = "document")]
    Document(InputMediaDocument),
    /// Photo to be sent
    #[serde(rename = "photo")]
    Photo(InputMediaPhoto),
    /// Video to be sent
    #[serde(rename = "video")]
    Video(InputMediaVideo),
}

/// Animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent
#[derive(Clone, Debug, Serialize)]
pub struct InputMediaAnimation {
    media: String,
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
    /// Creates a new InputMediaAnimation with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * media - Pass a file_id to send a file that exists on the Telegram servers (recommended),
    ///           pass an HTTP URL for Telegram to get a file from the Internet,
    ///           or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data
    ///           under <file_attach_name> name
    pub fn new<S: Into<String>>(media: S) -> Self {
        InputMediaAnimation {
            media: media.into(),
            thumb: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
        }
    }

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
#[derive(Clone, Debug, Serialize)]
pub struct InputMediaAudio {
    media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<String>,
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
    /// Creates a new InputMediaAudio with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * media - Pass a file_id to send a file that exists on the Telegram servers (recommended),
    ///           pass an HTTP URL for Telegram to get a file from the Internet,
    ///           or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data
    ///           under <file_attach_name> name
    pub fn new<S: Into<String>>(media: S) -> Self {
        InputMediaAudio {
            media: media.into(),
            thumb: None,
            caption: None,
            parse_mode: None,
            duration: None,
            performer: None,
            title: None,
        }
    }

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
#[derive(Clone, Debug, Serialize)]
pub struct InputMediaDocument {
    media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
}

impl InputMediaDocument {
    /// Creates a new InputMediaDocument with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * media - Pass a file_id to send a file that exists on the Telegram servers (recommended),
    ///           pass an HTTP URL for Telegram to get a file from the Internet,
    ///           or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data
    ///           under <file_attach_name> name
    pub fn new<S: Into<String>>(media: S) -> Self {
        InputMediaDocument {
            media: media.into(),
            thumb: None,
            caption: None,
            parse_mode: None,
        }
    }

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
#[derive(Clone, Debug, Serialize)]
pub struct InputMediaPhoto {
    media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
}

impl InputMediaPhoto {
    pub(crate) fn new<S: Into<String>>(media: S) -> Self {
        InputMediaPhoto {
            media: media.into(),
            caption: None,
            parse_mode: None,
        }
    }

    /// Caption of the photo to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(&mut self, caption: S) -> &mut Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set parse mode
    pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }
}

/// Video to be sent
#[derive(Clone, Debug, Serialize)]
pub struct InputMediaVideo {
    media: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_streaming: Option<bool>,
}

impl InputMediaVideo {
    pub(crate) fn new<S: Into<String>>(media: S) -> Self {
        InputMediaVideo {
            media: media.into(),
            thumb: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
        }
    }

    /// Set a thumbnail
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size
    /// A thumbnail‘s width and height should not exceed 90
    /// Ignored if the file is not uploaded using multipart/form-data
    /// Thumbnails can’t be reused and can be only uploaded
    /// as a new file, so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data
    /// under <file_attach_name>
    pub fn thumb<S: Into<String>>(&mut self, thumb: S) -> &mut Self {
        self.thumb = Some(thumb.into());
        self
    }

    /// Caption of the video to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(&mut self, caption: S) -> &mut Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set parse mode
    pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Set width
    pub fn width(&mut self, width: Integer) -> &mut Self {
        self.width = Some(width);
        self
    }

    /// Set height
    pub fn height(&mut self, height: Integer) -> &mut Self {
        self.height = Some(height);
        self
    }

    /// Set duration
    pub fn duration(&mut self, duration: Integer) -> &mut Self {
        self.duration = Some(duration);
        self
    }

    /// Pass True, if the uploaded video is suitable for streaming
    pub fn supports_streaming(&mut self, supports_streaming: bool) -> &mut Self {
        self.supports_streaming = Some(supports_streaming);
        self
    }
}
