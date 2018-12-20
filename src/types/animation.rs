use crate::types::photo_size::PhotoSize;
use crate::types::primitive::Integer;

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound)
#[derive(Clone, Debug, Deserialize)]
pub struct Animation {
    /// Unique file identifier
    pub file_id: String,
    /// Animation width as defined by sender
    pub width: Integer,
    /// Animation height as defined by sender
    pub height: Integer,
    /// Duration of the video in seconds as defined by sender
    pub duration: Integer,
    /// Animation thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Original animation filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<Integer>,
}
