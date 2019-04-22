use crate::types::primitive::Float;
use serde::Deserialize;

/// Point on the map
#[derive(Clone, Debug, Deserialize)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: Float,
    /// Latitude as defined by sender
    pub latitude: Float,
}

#[cfg(test)]
mod tests {
    use crate::types::{Message, MessageData, Update, UpdateKind};

    #[test]
    fn parse() {
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
                "location": {
                    "longitude": 2.0,
                    "latitude": 1.0
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Location(data),
            ..
        }) = update.kind
        {
            assert_eq!(data.latitude, 1.0);
            assert_eq!(data.longitude, 2.0);
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }
}
