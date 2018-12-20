use crate::types::primitive::{Integer, ParseMode};

/// Content of a media message to be sent
#[derive(Clone, Debug, Serialize)]
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

macro_rules! impl_input_media_from {
    ($to:ident($from:ident)) => {
        impl From<$from> for InputMedia {
            fn from(obj: $from) -> InputMedia {
                InputMedia::$to(obj)
            }
        }
    };
}

impl_input_media_from!(Animation(InputMediaAnimation));
impl_input_media_from!(Audio(InputMediaAudio));
impl_input_media_from!(Document(InputMediaDocument));
impl_input_media_from!(Photo(InputMediaPhoto));
impl_input_media_from!(Video(InputMediaVideo));

/// Animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent
#[derive(Clone, Debug, Serialize)]
pub struct InputMediaAnimation {
    /// Pass a file_id to send a file that exists on the Telegram servers (recommended),
    /// pass an HTTP URL for Telegram to get a file from the Internet,
    /// or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data
    /// under <file_attach_name> name
    pub media: String,
    /// The thumbnail should be in JPEG format and less than 200 kB in size
    /// A thumbnail‘s width and height should not exceed 90
    /// Ignored if the file is not uploaded using multipart/form-data
    /// Thumbnails can’t be reused and can be only uploaded
    /// as a new file, so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data
    /// under <file_attach_name>
    pub thumb: Option<String>,
    /// Caption of the animation to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// Animation width
    pub width: Option<Integer>,
    /// Animation height
    pub height: Option<Integer>,
    /// Animation duration
    pub duration: Option<Integer>,
}

impl InputMediaAnimation {
    /// Returns a new InputMedia with with empty optional parameters
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
}

/// Audio file to be treated as music to be sent
#[derive(Clone, Debug, Serialize)]
pub struct InputMediaAudio {
    /// Pass a file_id to send a file that exists on the Telegram servers (recommended),
    /// pass an HTTP URL for Telegram to get a file from the Internet,
    /// or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data
    /// under <file_attach_name> name.
    pub media: String,
    /// The thumbnail should be in JPEG format and less than 200 kB in size
    /// A thumbnail‘s width and height should not exceed 90
    /// Ignored if the file is not uploaded using multipart/form-data
    /// Thumbnails can’t be reused and can be only uploaded as a new file,
    /// so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data
    /// under <file_attach_name>
    pub thumb: Option<String>,
    /// Caption of the audio to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// Duration of the audio in seconds
    pub duration: Option<Integer>,
    /// Performer of the audio
    pub performer: Option<String>,
    /// Title of the audio
    pub title: Option<String>,
}

impl InputMediaAudio {
    /// Returns a new InputMedia with with empty optional parameters
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
}

/// General file to be sent
#[derive(Clone, Debug, Serialize)]
pub struct InputMediaDocument {
    /// Pass a file_id to send a file that exists on the Telegram servers (recommended),
    /// pass an HTTP URL for Telegram to get a file from the Internet,
    /// or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data
    /// under <file_attach_name> name
    pub media: String,
    /// The thumbnail should be in JPEG format and less than 200 kB in size
    /// A thumbnail‘s width and height should not exceed 90
    /// Ignored if the file is not uploaded using multipart/form-data
    /// Thumbnails can’t be reused and can be only uploaded as a new file,
    /// so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data under <file_attach_name>
    pub thumb: Option<String>,
    /// Caption of the document to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
}

impl InputMediaDocument {
    /// Returns a new InputMedia with with empty optional parameters
    pub fn new<S: Into<String>>(media: S) -> Self {
        InputMediaDocument {
            media: media.into(),
            thumb: None,
            caption: None,
            parse_mode: None,
        }
    }
}

/// Photo to be sent
#[derive(Clone, Debug, Serialize)]
pub struct InputMediaPhoto {
    /// Pass a file_id to send a file that exists on the Telegram servers (recommended),
    /// pass an HTTP URL for Telegram to get a file from the Internet,
    /// or pass “attach://<file_attach_name>”
    /// to upload a new one using multipart/form-data under <file_attach_name> name
    pub media: String,
    /// Caption of the photo to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
}

impl InputMediaPhoto {
    /// Returns a new InputMedia with with empty optional parameters
    pub fn new<S: Into<String>>(media: S) -> Self {
        InputMediaPhoto {
            media: media.into(),
            caption: None,
            parse_mode: None,
        }
    }
}

/// Video to be sent
#[derive(Clone, Debug, Serialize)]
pub struct InputMediaVideo {
    /// Pass a file_id to send a file that exists on the Telegram servers (recommended),
    /// pass an HTTP URL for Telegram to get a file from the Internet,
    /// or pass “attach://<file_attach_name>”
    /// to upload a new one using multipart/form-data under <file_attach_name> name
    pub media: String,
    /// The thumbnail should be in JPEG format and less than 200 kB in size
    /// A thumbnail‘s width and height should not exceed 90
    /// Ignored if the file is not uploaded using multipart/form-data
    /// Thumbnails can’t be reused and can be only uploaded as a new file,
    /// so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data under <file_attach_name>
    pub thumb: Option<String>,
    /// Caption of the video to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
    /// Video width
    pub width: Option<Integer>,
    /// Video height
    pub height: Option<Integer>,
    /// Video duration
    pub duration: Option<Integer>,
    /// Pass True, if the uploaded video is suitable for streaming
    pub supports_streaming: Option<bool>,
}

impl InputMediaVideo {
    /// Returns a new InputMedia with with empty optional parameters
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
}
