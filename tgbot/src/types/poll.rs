use crate::types::primitive::Integer;
use serde::{Deserialize, Serialize};

/// Contains information about a poll
#[derive(Clone, Debug, Deserialize)]
pub struct Poll {
    /// Unique poll identifier
    pub id: String,
    /// Poll question, 1-255 characters
    pub question: String,
    /// List of poll options
    pub options: Vec<PollOption>,
    /// Total number of users that voted in the poll
    pub total_voter_count: Integer,
    /// True, if the poll is closed
    pub is_closed: bool,
    /// True, if the poll is anonymous
    pub is_anonymous: bool,
    #[serde(rename = "type")]
    /// Poll kind, currently can be “regular” or “quiz”
    pub kind: PollKind,
    /// True, if the poll allows multiple answers
    pub allows_multiple_answers: bool,
    /// 0-based identifier of the correct answer option
    ///
    /// Available only for polls in the quiz mode, which are closed,
    /// or was sent (not forwarded) by the bot or
    /// to the private chat with the bot
    pub correct_option_id: Option<Integer>,
}

/// Contains information about one answer option in a poll
#[derive(Clone, Debug, Deserialize)]
pub struct PollOption {
    /// Option text, 1-100 characters
    pub text: String,
    /// Number of users that voted for this option
    pub voter_count: Integer,
}

/// Kind of a native poll
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PollKind {
    /// Quiz Mode
    ///
    /// Such polls have one correct answer
    Quiz,
    /// A regular poll
    Regular,
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
            "is_closed": true,
            "total_voter_count": 100,
            "is_anonymous": true,
            "type": "regular",
            "allows_multiple_answers": false
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
        assert_eq!(data.total_voter_count, 100);
        assert!(data.is_anonymous);
        assert_eq!(data.kind, PollKind::Regular);
        assert!(!data.allows_multiple_answers);
        assert!(data.correct_option_id.is_none());
    }
}
