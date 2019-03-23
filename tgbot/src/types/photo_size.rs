use crate::types::primitive::Integer;
use serde::Deserialize;

/// Size of a photo or a file / sticker thumbnail
#[derive(Clone, Debug, Deserialize)]
pub struct PhotoSize {
    /// Unique identifier for this file
    pub file_id: String,
    /// Photo width
    pub width: Integer,
    /// Photo height
    pub height: Integer,
    /// File size
    pub file_size: Option<Integer>,
}
