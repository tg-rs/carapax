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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_full() {
        let data: PhotoSize = serde_json::from_value(serde_json::json!({
            "file_id": "file-id",
            "width": 200,
            "height": 200,
            "file_size": 1234
        }))
        .unwrap();
        assert_eq!(data.file_id, "file-id");
        assert_eq!(data.width, 200);
        assert_eq!(data.height, 200);
        assert_eq!(data.file_size.unwrap(), 1234);
    }

    #[test]
    fn deserialize_partial() {
        let data: PhotoSize = serde_json::from_value(serde_json::json!({
            "file_id": "file-id",
            "width": 200,
            "height": 200
        }))
        .unwrap();
        assert_eq!(data.file_id, "file-id");
        assert_eq!(data.width, 200);
        assert_eq!(data.height, 200);
        assert!(data.file_size.is_none());
    }
}
