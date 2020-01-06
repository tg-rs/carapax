use crate::types::{photo_size::PhotoSize, primitive::Integer};
use serde::Deserialize;

/// Video file
#[derive(Clone, Debug, Deserialize)]
pub struct Video {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width as defined by sender
    pub width: Integer,
    /// Video height as defined by sender
    pub height: Integer,
    /// Duration of the video in seconds as defined by sender
    pub duration: Integer,
    /// Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Mime type of a file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<Integer>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_full() {
        let data: Video = serde_json::from_value(serde_json::json!({
            "file_id": "test video file id",
            "file_unique_id": "unique-id",
            "width": 1,
            "height": 2,
            "duration": 3,
            "thumb": {
                "file_id": "AdddddUuUUUUccccUUmm_PPP",
                "file_unique_id": "unique-thumb-id",
                "width": 24,
                "height": 24,
                "file_size": 12324
            },
            "mime_type": "video/mpeg",
            "file_size": 4
        }))
        .unwrap();

        assert_eq!(data.file_id, "test video file id");
        assert_eq!(data.file_unique_id, "unique-id");
        assert_eq!(data.width, 1);
        assert_eq!(data.height, 2);
        assert_eq!(data.duration, 3);

        let thumb = data.thumb.unwrap();
        assert_eq!(thumb.file_id, "AdddddUuUUUUccccUUmm_PPP");
        assert_eq!(thumb.file_unique_id, "unique-thumb-id");
        assert_eq!(thumb.width, 24);
        assert_eq!(thumb.height, 24);
        assert_eq!(thumb.file_size.unwrap(), 12324);

        assert_eq!(data.mime_type.unwrap(), "video/mpeg");
        assert_eq!(data.file_size.unwrap(), 4);
    }

    #[test]
    fn deserialize_partial() {
        let data: Video = serde_json::from_value(serde_json::json!({
            "file_id": "test video file id",
            "file_unique_id": "unique-id",
            "width": 1,
            "height": 2,
            "duration": 3
        }))
        .unwrap();

        assert_eq!(data.file_id, "test video file id");
        assert_eq!(data.file_unique_id, "unique-id");
        assert_eq!(data.width, 1);
        assert_eq!(data.height, 2);
        assert_eq!(data.duration, 3);
        assert!(data.thumb.is_none());
        assert!(data.mime_type.is_none());
        assert!(data.file_size.is_none());
    }
}
