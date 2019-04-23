use crate::types::{photo_size::PhotoSize, primitive::Integer};
use serde::Deserialize;

/// Audio file to be treated as music by the Telegram clients
#[derive(Clone, Debug, Deserialize)]
pub struct Audio {
    /// Unique identifier for this file
    pub file_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: Integer,
    /// Performer of the audio as defined by sender or by audio tags
    pub performer: Option<String>,
    /// Title of the audio as defined by sender or by audio tags
    pub title: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<Integer>,
    /// Thumbnail of the album cover to which the music file belongs
    pub thumb: Option<PhotoSize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_full() {
        let data: Audio = serde_json::from_value(serde_json::json!({
            "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
            "duration": 243,
            "performer": "Performer",
            "title": "Title",
            "mime_type": "audio/mpeg",
            "file_size": 1234,
            "thumb": {
                "file_id": "AdddddUuUUUUccccUUmm_PPP",
                "width": 24,
                "height": 24,
                "file_size": 12324
            }
        }))
        .unwrap();

        assert_eq!(data.file_id, "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX");
        assert_eq!(data.duration, 243);
        assert_eq!(data.performer.unwrap(), "Performer");
        assert_eq!(data.title.unwrap(), "Title");
        assert_eq!(data.mime_type.unwrap(), "audio/mpeg");
        assert_eq!(data.file_size.unwrap(), 1234);

        let thumb = data.thumb.unwrap();
        assert_eq!(thumb.file_id, "AdddddUuUUUUccccUUmm_PPP");
        assert_eq!(thumb.width, 24);
        assert_eq!(thumb.height, 24);
        assert_eq!(thumb.file_size.unwrap(), 12324);
    }

    #[test]
    fn deserialize_partial() {
        let data: Audio = serde_json::from_value(serde_json::json!({
            "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
            "duration": 243
        }))
        .unwrap();
        assert_eq!(data.file_id, "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX");
        assert_eq!(data.duration, 243);
        assert!(data.performer.is_none());
        assert!(data.title.is_none());
        assert!(data.mime_type.is_none());
        assert!(data.file_size.is_none());
        assert!(data.thumb.is_none());
    }
}
