use crate::types::primitive::{Integer, ParseMode};
use serde::Serialize;

/// Photo or video to be sent in a media group
#[derive(Clone, Debug, derive_more::From, Serialize)]
pub enum MediaGroupItem {
    /// Photo to be sent
    #[serde(rename = "photo")]
    Photo(InputMediaPhoto),
    /// Video to be sent
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
    /// Creates a new InputMediaPhoto with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * media - Pass a file_id to send a file that exists on the Telegram servers (recommended),
    ///           pass an HTTP URL for Telegram to get a file from the Internet,
    ///           or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data
    ///           under <file_attach_name> name
    pub fn new<S: Into<String>>(media: S) -> Self {
        InputMediaPhoto {
            media: media.into(),
            caption: None,
            parse_mode: None,
        }
    }

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
    /// Creates a new InputMediaVideo with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * media - Pass a file_id to send a file that exists on the Telegram servers (recommended),
    ///           pass an HTTP URL for Telegram to get a file from the Internet,
    ///           or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data
    ///           under <file_attach_name> name
    pub fn new<S: Into<String>>(media: S) -> Self {
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
    pub fn thumb<S: Into<String>>(mut self, thumb: S) -> Self {
        self.thumb = Some(thumb.into());
        self
    }

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
