use crate::types::{photo_size::PhotoSize, primitive::Integer};
use serde::Deserialize;

/// Video file
#[derive(Clone, Debug, Deserialize)]
pub struct Video {
    /// Unique identifier for this file
    pub file_id: String,
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
                "caption": "test caption",
                "video": {
                    "file_id": "test video file id",
                    "width": 1,
                    "height": 2,
                    "duration": 3,
                    "thumb": {
                        "file_id": "AdddddUuUUUUccccUUmm_PPP",
                        "width": 24,
                        "height": 24,
                        "file_size": 12324
                    },
                    "mime_type": "video/mpeg",
                    "file_size": 4,
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Video { caption, data },
            ..
        }) = update.kind
        {
            assert_eq!(caption.unwrap().data, String::from("test caption"));
            assert_eq!(data.file_id, String::from("test video file id"));
            assert_eq!(data.width, 1);
            assert_eq!(data.height, 2);
            assert_eq!(data.duration, 3);

            let thumb = data.thumb.unwrap();
            assert_eq!(thumb.file_id, String::from("AdddddUuUUUUccccUUmm_PPP"));
            assert_eq!(thumb.width, 24);
            assert_eq!(thumb.height, 24);
            assert_eq!(thumb.file_size.unwrap(), 12324);

            assert_eq!(data.mime_type.unwrap(), String::from("video/mpeg"));
            assert_eq!(data.file_size.unwrap(), 4);
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
                "video": {
                    "file_id": "test video file id",
                    "width": 1,
                    "height": 2,
                    "duration": 3
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Video { caption, data },
            ..
        }) = update.kind
        {
            assert!(caption.is_none());
            assert_eq!(data.file_id, String::from("test video file id"));
            assert_eq!(data.width, 1);
            assert_eq!(data.height, 2);
            assert_eq!(data.duration, 3);
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }
}
