use crate::types::{photo_size::PhotoSize, primitive::Integer};
use serde::Deserialize;

/// General file (as opposed to photos, voice messages and audio files)
#[derive(Clone, Debug, Deserialize)]
pub struct Document {
    /// Unique file identifier
    pub file_id: String,
    /// Document thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Original filename as defined by sender
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
                "caption": "test caption",
                "document": {
                    "file_id": "SSSxmmmsmsIIsooofiiiiaiiaIII_XLA",
                    "thumb": {
                        "file_id": "AdddddUuUUUUccccUUmm_PPP",
                        "width": 24,
                        "height": 24,
                        "file_size": 12324
                    },
                    "file_name": "Test file name",
                    "mime_type": "image/jpeg",
                    "file_size": 1234
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Document { data, caption },
            ..
        }) = update.kind
        {
            assert_eq!(caption.unwrap().data, String::from("test caption"));

            assert_eq!(data.file_id, String::from("SSSxmmmsmsIIsooofiiiiaiiaIII_XLA"));

            let thumb = data.thumb.unwrap();
            assert_eq!(thumb.file_id, String::from("AdddddUuUUUUccccUUmm_PPP"));
            assert_eq!(thumb.width, 24);
            assert_eq!(thumb.height, 24);
            assert_eq!(thumb.file_size.unwrap(), 12324);

            assert_eq!(data.file_name.unwrap(), String::from("Test file name"));
            assert_eq!(data.mime_type.unwrap(), String::from("image/jpeg"));
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
                "document": {
                    "file_id": "SSSxmmmsmsIIsooofiiiiaiiaIII_XLA"
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Document { data, caption },
            ..
        }) = update.kind
        {
            assert!(caption.is_none());
            assert_eq!(data.file_id, String::from("SSSxmmmsmsIIsooofiiiiaiiaIII_XLA"));
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }
}
