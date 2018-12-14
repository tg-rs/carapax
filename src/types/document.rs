use crate::types::photo_size::PhotoSize;

/// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Debug)]
pub struct Document {
    /// Unique file identifier
    pub file_id: String,
    /// Document thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Original filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<i64>,
}
