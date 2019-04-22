use crate::types::{photo_size::PhotoSize, primitive::Integer};
use serde::Deserialize;

/// An animation file (GIF or H.264/MPEG-4 AVC video without sound)
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
                "animation": {
                    "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
                    "width": 200,
                    "height": 200,
                    "duration": 243,
                    "thumb": {
                        "file_id": "AdddddUuUUUUccccUUmm_PPP",
                        "width": 24,
                        "height": 24,
                        "file_size": 12324
                    },
                    "file_name": "testfilename",
                    "mime_type": "image/gif",
                    "file_size": 3897500
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Animation(data),
            ..
        }) = update.kind
        {
            assert_eq!(data.file_id, String::from("AwADBAADbXXXXXXXXXXXGBdhD2l6_XX"));
            assert_eq!(data.width, 200);
            assert_eq!(data.height, 200);
            assert_eq!(data.duration, 243);

            let thumb = data.thumb.unwrap();
            assert_eq!(thumb.file_id, String::from("AdddddUuUUUUccccUUmm_PPP"));
            assert_eq!(thumb.width, 24);
            assert_eq!(thumb.height, 24);
            assert_eq!(thumb.file_size.unwrap(), 12324);

            assert_eq!(data.file_name.unwrap(), String::from("testfilename"));
            assert_eq!(data.mime_type.unwrap(), String::from("image/gif"));
            assert_eq!(data.file_size.unwrap(), 3897500);
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
                "caption": "test caption",
                "animation": {
                    "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
                    "width": 200,
                    "height": 200,
                    "duration": 243
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Animation(data),
            ..
        }) = update.kind
        {
            assert_eq!(data.file_id, String::from("AwADBAADbXXXXXXXXXXXGBdhD2l6_XX"));
            assert_eq!(data.width, 200);
            assert_eq!(data.height, 200);
            assert_eq!(data.duration, 243);
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }
}
