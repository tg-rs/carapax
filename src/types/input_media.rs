use crate::types::primitive::{Integer, ParseMode};

/// This object represents the content of a media message to be sent.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
pub enum InputMedia {
    /// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
    #[serde(rename = "animation")]
    Animation {
        /// File to send.
        /// Pass a file_id to send a file that
        /// exists on the Telegram servers (recommended),
        /// pass an HTTP URL for Telegram to
        /// get a file from the Internet,
        /// or pass “attach://<file_attach_name>”
        /// to upload a new one using multipart/form-data
        /// under <file_attach_name> name.
        media: String,
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
        thumb: Option<String>,
        /// Caption of the animation to be sent, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Animation width
        width: Option<Integer>,
        /// Animation height
        height: Option<Integer>,
        /// Animation duration
        duration: Option<Integer>,
    },
    /// Represents an audio file to be treated as music to be sent.
    #[serde(rename = "audio")]
    Audio {
        /// File to send.
        /// Pass a file_id to send a file that
        /// exists on the Telegram servers (recommended),
        /// pass an HTTP URL for Telegram to
        /// get a file from the Internet,
        /// or pass “attach://<file_attach_name>”
        /// to upload a new one using multipart/form-data
        /// under <file_attach_name> name.
        media: String,
        /// Thumbnail of the file sent.
        /// The thumbnail should be in JPEG format and less than 200 kB in size.
        /// A thumbnail‘s width and height should not exceed 90.
        /// Ignored if the file is not uploaded using multipart/form-data.
        /// Thumbnails can’t be reused and can be only uploaded as a new file,
        /// so you can pass “attach://<file_attach_name>”
        /// if the thumbnail was uploaded using multipart/form-data
        /// under <file_attach_name>.
        thumb: Option<String>,
        /// Caption of the audio to be sent, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Duration of the audio in seconds
        duration: Option<Integer>,
        /// Performer of the audio
        performer: Option<String>,
        /// Title of the audio
        title: Option<String>,
    },
    /// Represents a general file to be sent.
    #[serde(rename = "document")]
    Document {
        /// File to send.
        /// Pass a file_id to send a file that
        /// exists on the Telegram servers (recommended),
        /// pass an HTTP URL for Telegram to
        /// get a file from the Internet,
        /// or pass “attach://<file_attach_name>”
        /// to upload a new one using multipart/form-data
        /// under <file_attach_name> name.
        media: String,
        /// Thumbnail of the file sent.
        /// The thumbnail should be in JPEG format and less than 200 kB in size.
        /// A thumbnail‘s width and height should not exceed 90.
        /// Ignored if the file is not uploaded using multipart/form-data.
        /// Thumbnails can’t be reused and can be only uploaded as a new file,
        /// so you can pass “attach://<file_attach_name>”
        /// if the thumbnail was uploaded using multipart/form-data
        /// under <file_attach_name>.
        thumb: Option<String>,
        /// Caption of the document to be sent, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
    },
    /// Represents a photo to be sent.
    #[serde(rename = "photo")]
    Photo {
        /// File to send.
        /// Pass a file_id to send a file that exists on the Telegram servers (recommended),
        /// pass an HTTP URL for Telegram to get a file from the Internet,
        /// or pass “attach://<file_attach_name>”
        /// to upload a new one using multipart/form-data
        /// under <file_attach_name> name.
        media: String,
        /// Caption of the photo to be sent, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
    },
    /// Represents a video to be sent.
    #[serde(rename = "video")]
    Video {
        /// File to send.
        /// Pass a file_id to send a file that exists on the Telegram servers (recommended),
        /// pass an HTTP URL for Telegram to get a file from the Internet,
        /// or pass “attach://<file_attach_name>”
        /// to upload a new one using multipart/form-data under <file_attach_name> name.
        media: String,
        /// Thumbnail of the file sent.
        /// The thumbnail should be in JPEG format and less than 200 kB in size.
        /// A thumbnail‘s width and height should not exceed 90.
        /// Ignored if the file is not uploaded using multipart/form-data.
        /// Thumbnails can’t be reused and can be only uploaded as a new file,
        /// so you can pass “attach://<file_attach_name>”
        /// if the thumbnail was uploaded using multipart/form-data
        /// under <file_attach_name>.
        thumb: Option<String>,
        /// Caption of the video to be sent, 0-1024 characters
        caption: Option<String>,
        /// Parse mode
        parse_mode: Option<ParseMode>,
        /// Video width
        width: Option<Integer>,
        /// Video height
        height: Option<Integer>,
        /// Video duration
        duration: Option<Integer>,
        /// Pass True, if the uploaded video is suitable for streaming
        supports_streaming: Option<bool>,
    },
}
