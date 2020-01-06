use crate::types::primitive::Integer;
use serde::Deserialize;

/// Size of a photo or a file / sticker thumbnail
#[derive(Clone, Debug, Deserialize)]
pub struct PhotoSize {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
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
            "file_unique_id": "unique-id",
            "width": 200,
            "height": 200,
            "file_size": 1234
        }))
        .unwrap();
        assert_eq!(data.file_id, "file-id");
        assert_eq!(data.file_unique_id, "unique-id");
        assert_eq!(data.width, 200);
        assert_eq!(data.height, 200);
        assert_eq!(data.file_size.unwrap(), 1234);
    }

    #[test]
    fn deserialize_partial() {
        let data: PhotoSize = serde_json::from_value(serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "unique-id",
            "width": 200,
            "height": 200
        }))
        .unwrap();
        assert_eq!(data.file_id, "file-id");
        assert_eq!(data.file_unique_id, "unique-id");
        assert_eq!(data.width, 200);
        assert_eq!(data.height, 200);
        assert!(data.file_size.is_none());
    }
}
