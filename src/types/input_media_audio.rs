use crate::types::primitive::Integer;

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
    // pub thumb: Option<InputFile | String>, // TODO
    /// Caption of the audio to be sent, 0-1024 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show
    /// bold, italic, fixed-width text or
    /// inline URLs in the media caption.
    pub parse_mode: Option<String>, // TODO: enum
    /// Duration of the audio in seconds
    pub duration: Option<Integer>,
    /// Performer of the audio
    pub performer: Option<String>,
    /// Title of the audio
    pub title: Option<String>,
}
