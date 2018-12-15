use crate::types::photo_size::PhotoSize;
use crate::types::primitive::Integer;

/// This object represents a video message
#[derive(Debug)]
pub struct VideoNote {
    /// Unique identifier for this file
    pub file_id: String,
    /// Video width and height
    pub length: Integer,
    ///  Duration of the video in seconds
    pub duration: Integer,
    /// Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// File size
    pub file_size: Option<Integer>,
}
