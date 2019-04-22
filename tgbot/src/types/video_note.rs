use crate::types::{photo_size::PhotoSize, primitive::Integer};
use serde::Deserialize;

/// Video message
#[derive(Clone, Debug, Deserialize)]
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

#[cfg(test)]
mod tests {
    use crate::types::{Message, MessageData, Update, UpdateKind};

    #[test]
    fn parse_full() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 10000,
            "message": {
                "date": 1441645532,
                "chat": {
                    "last_name": "Test Lastname",
                    "type": "private",
                    "id": 1111111,
                    "first_name": "Test Firstname",
                    "username": "Testusername"
                },
                "message_id": 1365,
                "from": {
                    "last_name": "Test Lastname",
                    "id": 1111111,
                    "first_name": "Test Firstname",
                    "username": "Testusername",
                    "is_bot": false
                },
                "video_note": {
                    "file_id": "video note file id",
                    "length": 124,
                    "duration": 1234,
                    "thumb": {
                        "file_id": "AdddddUuUUUUccccUUmm_PPP",
                        "width": 24,
                        "height": 24,
                        "file_size": 12324
                    },
                    "file_size": 12345,
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::VideoNote(data),
            ..
        }) = update.kind
        {
            assert_eq!(data.file_id, String::from("video note file id"));
            assert_eq!(data.length, 124);
            assert_eq!(data.duration, 1234);

            let thumb = data.thumb.unwrap();
            assert_eq!(thumb.file_id, String::from("AdddddUuUUUUccccUUmm_PPP"));
            assert_eq!(thumb.width, 24);
            assert_eq!(thumb.height, 24);
            assert_eq!(thumb.file_size.unwrap(), 12324);

            assert_eq!(data.file_size.unwrap(), 12345);
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }

    #[test]
    fn parse_partial() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 10000,
            "message": {
                "date": 1441645532,
                "chat": {
                    "last_name": "Test Lastname",
                    "type": "private",
                    "id": 1111111,
                    "first_name": "Test Firstname",
                    "username": "Testusername"
                },
                "message_id": 1365,
                "from": {
                    "last_name": "Test Lastname",
                    "id": 1111111,
                    "first_name": "Test Firstname",
                    "username": "Testusername",
                    "is_bot": false
                },
                "video_note": {
                    "file_id": "video note file id",
                    "length": 124,
                    "duration": 1234
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::VideoNote(data),
            ..
        }) = update.kind
        {
            assert_eq!(data.file_id, String::from("video note file id"));
            assert_eq!(data.length, 124);
            assert_eq!(data.duration, 1234);
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }
}
