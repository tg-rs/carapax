use crate::types::{location::Location, user::User};
use serde::Deserialize;

mod message_content;
mod query_result;

pub use self::{message_content::*, query_result::*};

/// Incoming inline query
///
/// When the user sends an empty query, your bot could return some default or trending results
#[derive(Clone, Debug, Deserialize)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Sender location, only for bots that request user location
    pub location: Option<Location>,
    /// Text of the query (up to 512 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
}

/// Result of an inline query that was chosen by the user and sent to their chat partner
#[derive(Clone, Debug, Deserialize)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// Sender location, only for bots that require user location
    pub location: Option<Location>,
    /// Identifier of the sent inline message.
    /// Available only if there is an inline keyboard attached to the message
    /// Will be also received in callback queries and can be used to edit the message
    pub inline_message_id: Option<String>,
    /// The query that was used to obtain the result
    pub query: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::float_cmp)]
    #[test]
    fn deserialize_inline_query() {
        let data: InlineQuery = serde_json::from_value(serde_json::json!({
            "id": "query id",
            "from": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            },
            "location": {
                "latitude": 2.1,
                "longitude": 3.0
            },
            "query": "query string",
            "offset": "query offset",
        }))
        .unwrap();
        assert_eq!(data.id, "query id");
        assert_eq!(data.from.id, 1);
        assert_eq!(data.location.unwrap().latitude, 2.1);
        assert_eq!(data.query, "query string");
        assert_eq!(data.offset, "query offset");
    }

    #[allow(clippy::float_cmp)]
    #[test]
    fn deserialize_chosen_inline_result() {
        let data: ChosenInlineResult = serde_json::from_value(serde_json::json!({
            "result_id": "result id",
            "from": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            },
            "location": {
                "latitude": 2.1,
                "longitude": 3.0
            },
            "inline_message_id": "imi",
            "query": "q",
        }))
        .unwrap();
        assert_eq!(data.result_id, "result id");
        assert_eq!(data.from.id, 1);
        assert_eq!(data.location.unwrap().latitude, 2.1);
        assert_eq!(data.inline_message_id.unwrap(), "imi");
        assert_eq!(data.query, "q");
    }
}
