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
    /// Send Markdown or HTML,
    /// if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>, // TODO: enum
}
