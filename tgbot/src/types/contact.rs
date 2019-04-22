use crate::types::primitive::Integer;
use serde::Deserialize;

/// Phone contact
#[derive(Clone, Debug, Deserialize)]
pub struct Contact {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Contact's user identifier in Telegram
    pub user_id: Option<Integer>,
    /// Additional data about the contact in the form of a vCard
    pub vcard: Option<String>,
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
                "contact": {
                    "phone_number": "+79001231212",
                    "first_name": "First name",
                    "last_name": "Last name",
                    "user_id": 1234,
                    "vcard": "Test vcard",
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Contact(data),
            ..
        }) = update.kind
        {
            assert_eq!(data.phone_number, String::from("+79001231212"));
            assert_eq!(data.first_name, String::from("First name"));
            assert_eq!(data.last_name.unwrap(), String::from("Last name"));
            assert_eq!(data.user_id.unwrap(), 1234);
            assert_eq!(data.vcard.unwrap(), String::from("Test vcard"));
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
                "contact": {
                    "phone_number": "+79001231212",
                    "first_name": "First name",
                    "last_name": "Last name"
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Contact(data),
            ..
        }) = update.kind
        {
            assert_eq!(data.phone_number, String::from("+79001231212"));
            assert_eq!(data.first_name, String::from("First name"));
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }
}
