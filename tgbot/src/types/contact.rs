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
    use super::*;

    #[test]
    fn deserialize_full() {
        let data: Contact = serde_json::from_value(serde_json::json!({
            "phone_number": "+79001231212",
            "first_name": "First name",
            "last_name": "Last name",
            "user_id": 1234,
            "vcard": "Test vcard"
        }))
        .unwrap();

        assert_eq!(data.phone_number, "+79001231212");
        assert_eq!(data.first_name, "First name");
        assert_eq!(data.last_name.unwrap(), "Last name");
        assert_eq!(data.user_id.unwrap(), 1234);
        assert_eq!(data.vcard.unwrap(), "Test vcard");
    }

    #[test]
    fn deserialize_partial() {
        let data: Contact = serde_json::from_value(serde_json::json!({
            "phone_number": "+79001231212",
            "first_name": "First name"
        }))
        .unwrap();

        assert_eq!(data.phone_number, "+79001231212");
        assert_eq!(data.first_name, "First name");
        assert!(data.last_name.is_none());
        assert!(data.user_id.is_none());
        assert!(data.vcard.is_none());
    }
}
