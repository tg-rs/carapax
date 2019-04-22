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
                "audio": {
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
                        "file_size": 12324,
                    },
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Audio { data, caption },
            ..
        }) = update.kind
        {
            assert_eq!(caption.unwrap().data, String::from("test caption"));

            assert_eq!(data.file_id, String::from("AwADBAADbXXXXXXXXXXXGBdhD2l6_XX"));
            assert_eq!(data.duration, 243);
            assert_eq!(data.performer.unwrap(), String::from("Performer"));
            assert_eq!(data.title.unwrap(), String::from("Title"));
            assert_eq!(data.mime_type.unwrap(), String::from("audio/mpeg"));
            assert_eq!(data.file_size.unwrap(), 1234);

            let thumb = data.thumb.unwrap();
            assert_eq!(thumb.file_id, String::from("AdddddUuUUUUccccUUmm_PPP"));
            assert_eq!(thumb.width, 24);
            assert_eq!(thumb.height, 24);
            assert_eq!(thumb.file_size.unwrap(), 12324);
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
                "audio": {
                    "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
                    "duration": 243,
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Audio { data, caption },
            ..
        }) = update.kind
        {
            assert!(caption.is_none());
            assert_eq!(data.file_id, String::from("AwADBAADbXXXXXXXXXXXGBdhD2l6_XX"));
            assert_eq!(data.duration, 243);
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }
}
