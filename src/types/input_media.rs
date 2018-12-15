use crate::types::primitive::{Integer, ParseMode};

/// This object represents the content of a media message to be sent.
#[derive(Debug)]
pub enum InputMedia {
    /// Animation file
    Animation(InputMediaAnimation),
    /// Audio file
    Audio(InputMediaAudio),
    /// A general file
    Document(InputMediaDocument),
    /// A photo
    Photo(InputMediaPhoto),
    /// Video file
    Video(InputMediaVideo),
}

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
#[derive(Debug)]
pub struct InputMediaAnimation {
    /// Type of the result, must be animation
    pub kind: String, // TODO: rename to type
    /// File to send.
    /// Pass a file_id to send a file that
    /// exists on the Telegram servers (recommended),
    /// pass an HTTP URL for Telegram to
    /// get a file from the Internet,
    /// or pass “attach://<file_attach_name>”
    /// to upload a new one using multipart/form-data
    /// under <file_attach_name> name.
    pub media: String,
    /// Thumbnail of the file sent.
    /// The thumbnail should be in JPEG format
    /// and less than 200 kB in size.
    /// A thumbnail‘s width and height
    /// should not exceed 90.
    /// Ignored if the file is not uploaded using multipart/form-data.
    /// Thumbnails can’t be reused and can be only uploaded
    /// as a new file, so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data
    /// under <file_attach_name>.
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

/// Represents an audio file to be treated as music to be sent.
#[derive(Debug)]
pub struct InputMediaAudio {
    /// Type of the result, must be audio
    pub kind: String, // TODO: rename to type
    /// File to send.
    /// Pass a file_id to send a file that
    /// exists on the Telegram servers (recommended),
    /// pass an HTTP URL for Telegram to
    /// get a file from the Internet,
    /// or pass “attach://<file_attach_name>”
    /// to upload a new one using multipart/form-data
    /// under <file_attach_name> name.
    pub media: String,
    /// Thumbnail of the file sent.
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail‘s width and height should not exceed 90.
    /// Ignored if the file is not uploaded using multipart/form-data.
    /// Thumbnails can’t be reused and can be only uploaded as a new file,
    /// so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data
    /// under <file_attach_name>.
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

/// Represents a general file to be sent.
#[derive(Debug)]
pub struct InputMediaDocument {
    /// Type of the result, must be document
    pub kind: String, // TODO: rename to type
    /// File to send.
    /// Pass a file_id to send a file that
    /// exists on the Telegram servers (recommended),
    /// pass an HTTP URL for Telegram to
    /// get a file from the Internet,
    /// or pass “attach://<file_attach_name>”
    /// to upload a new one using multipart/form-data
    /// under <file_attach_name> name.
    pub media: String,
    /// Thumbnail of the file sent.
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail‘s width and height should not exceed 90.
    /// Ignored if the file is not uploaded using multipart/form-data.
    /// Thumbnails can’t be reused and can be only uploaded as a new file,
    /// so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data
    /// under <file_attach_name>.
    pub thumb: Option<String>,
    /// Caption of the document to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
}

/// Represents a photo to be sent.
#[derive(Debug)]
pub struct InputMediaPhoto {
    /// Type of the result, must be photo
    pub kind: String, // TODO: rename to type
    /// File to send.
    /// Pass a file_id to send a file that exists on the Telegram servers (recommended),
    /// pass an HTTP URL for Telegram to get a file from the Internet,
    /// or pass “attach://<file_attach_name>”
    /// to upload a new one using multipart/form-data
    /// under <file_attach_name> name.
    pub media: String,
    /// Caption of the photo to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Parse mode
    pub parse_mode: Option<ParseMode>,
}

/// Represents a video to be sent.
#[derive(Debug)]
pub struct InputMediaVideo {
    /// Type of the result, must be video
    pub kind: String, // TODO: rename to type
    /// File to send.
    /// Pass a file_id to send a file that exists on the Telegram servers (recommended),
    /// pass an HTTP URL for Telegram to get a file from the Internet,
    /// or pass “attach://<file_attach_name>”
    /// to upload a new one using multipart/form-data under <file_attach_name> name.
    pub media: String,
    /// Thumbnail of the file sent.
    /// The thumbnail should be in JPEG format and less than 200 kB in size.
    /// A thumbnail‘s width and height should not exceed 90.
    /// Ignored if the file is not uploaded using multipart/form-data.
    /// Thumbnails can’t be reused and can be only uploaded as a new file,
    /// so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data
    /// under <file_attach_name>.
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
