use crate::types::primitive::Integer;
use serde::Deserialize;

/// Voice note
#[derive(Clone, Debug, Deserialize)]
pub struct Voice {
    /// Unique identifier for this file
    pub file_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: Integer,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<Integer>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_full() {
        let data: Voice = serde_json::from_value(serde_json::json!({
            "file_id": "voice file id",
            "duration": 123,
            "mime_type": "audio/ogg",
            "file_size": 1234
        }))
        .unwrap();

        assert_eq!(data.file_id, "voice file id");
        assert_eq!(data.duration, 123);
        assert_eq!(data.mime_type.unwrap(), "audio/ogg");
        assert_eq!(data.file_size.unwrap(), 1234);
    }

    #[test]
    fn deserialize_partial() {
        let data: Voice = serde_json::from_value(serde_json::json!({
            "file_id": "voice file id",
            "duration": 123
        }))
        .unwrap();

        assert_eq!(data.file_id, "voice file id");
        assert_eq!(data.duration, 123);
        assert!(data.mime_type.is_none());
        assert!(data.file_size.is_none());
    }
}
