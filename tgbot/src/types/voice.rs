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
                "voice": {
                    "file_id": "voice file id",
                    "duration": 123,
                    "mime_type": "audio/ogg",
                    "file_size": 1234
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Voice { data, caption },
            ..
        }) = update.kind
        {
            assert_eq!(data.file_id, String::from("voice file id"));
            assert_eq!(data.duration, 123);
            assert_eq!(data.mime_type.unwrap(), String::from("audio/ogg"));
            assert_eq!(data.file_size.unwrap(), 1234);
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
                "voice": {
                    "file_id": "voice file id",
                    "duration": 123
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Voice { data, caption },
            ..
        }) = update.kind
        {
            assert_eq!(data.file_id, String::from("voice file id"));
            assert_eq!(data.duration, 123);
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }
}
