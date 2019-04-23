use crate::types::primitive::Integer;
use serde::Deserialize;

/// Contains information about a poll
#[derive(Clone, Debug, Deserialize)]
pub struct Poll {
    /// Unique poll identifier
    pub id: String,
    /// Poll question, 1-255 characters
    pub question: String,
    /// List of poll options
    pub options: Vec<PollOption>,
    /// True, if the poll is closed
    pub is_closed: bool,
}

/// Contains information about one answer option in a poll
#[derive(Clone, Debug, Deserialize)]
pub struct PollOption {
    /// Option text, 1-100 characters
    pub text: String,
    /// Number of users that voted for this option
    pub voter_count: Integer,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data: Poll = serde_json::from_value(serde_json::json!({
            "id": "poll-id",
            "question": "Rust?",
            "options": [
                {"text": "Yes", "voter_count": 1000},
                {"text": "No", "voter_count": 0}
            ],
            "is_closed": true
        }))
        .unwrap();
        assert_eq!(data.id, "poll-id");
        assert_eq!(data.question, "Rust?");
        assert_eq!(data.options.len(), 2);
        let yes = &data.options[0];
        assert_eq!(yes.text, "Yes");
        assert_eq!(yes.voter_count, 1000);
        let no = &data.options[1];
        assert_eq!(no.text, "No");
        assert_eq!(no.voter_count, 0);
        assert!(data.is_closed);
    }
}
