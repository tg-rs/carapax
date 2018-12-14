use crate::types::photo_size::PhotoSize;

/// This object represents a video message
#[derive(Debug)]
pub struct VideoNote {
    /// Unique identifier for this file
    pub file_id: String,
    /// Video width and height
    pub length: i64,
    ///  Duration of the video in seconds
    pub duration: i64,
    /// Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// File size
    pub file_size: Option<i64>,
}
