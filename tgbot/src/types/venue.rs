use crate::types::location::Location;
use serde::Deserialize;

/// Venue
#[derive(Clone, Debug, Deserialize)]
pub struct Venue {
    /// Venue location
    pub location: Location,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue
    /// For example: “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”
    pub foursquare_type: Option<String>,
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
                "venue": {
                    "location": {
                        "latitude": 1.1,
                        "longitude": 2.0
                    },
                    "title": "venue title",
                    "address": "venue address",
                    "foursquare_id": "f-id",
                    "foursquare_type": "f-type"
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Venue(data),
            ..
        }) = update.kind
        {
            assert_eq!(data.location.latitude, 1.1);
            assert_eq!(data.location.longitude, 2.0);
            assert_eq!(data.title, String::from("venue title"));
            assert_eq!(data.address, String::from("venue address"));
            assert_eq!(data.foursquare_id.unwrap(), String::from("f-id"));
            assert_eq!(data.foursquare_type.unwrap(), String::from("f-type"));
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
                "venue": {
                    "location": {
                        "latitude": 1.1,
                        "longitude": 2.0
                    },
                    "title": "venue title",
                    "address": "venue address"
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Venue(data),
            ..
        }) = update.kind
        {
            assert_eq!(data.location.latitude, 1.1);
            assert_eq!(data.location.longitude, 2.0);
            assert_eq!(data.title, String::from("venue title"));
            assert_eq!(data.address, String::from("venue address"));
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }
}
