use crate::types::{message::Message, user::User};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Error as JsonError;
use std::{error::Error as StdError, fmt};

/// Incoming callback query from a callback button in an inline keyboard
///
/// If the button that originated the query was attached to a message sent by the bot,
/// the field message will be present
/// If the button was attached to a message sent via the bot (in inline mode),
/// the field inline_message_id will be present
/// Exactly one of the fields data or game_short_name will be present
#[derive(Clone, Debug, Deserialize)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Message with the callback button that originated the query
    /// Note that message content and message date
    /// will not be available if the message is too old
    pub message: Option<Message>,
    /// Identifier of the message sent via the bot
    /// in inline mode, that originated the query
    pub inline_message_id: Option<String>,
    /// Global identifier, uniquely corresponding
    /// to the chat to which the message with the
    /// callback button was sent
    /// Useful for high scores in games
    pub chat_instance: Option<String>,
    /// Data associated with the callback button.
    /// Be aware that a bad client can send arbitrary data in this field
    pub data: Option<String>,
    /// Short name of a Game to be returned,
    /// serves as the unique identifier for the game
    pub game_short_name: Option<String>,
}

impl CallbackQuery {
    /// Parses callback data using serde_json
    pub fn parse_data<T: DeserializeOwned>(&self) -> Result<Option<T>, CallbackQueryError> {
        if let Some(ref data) = self.data {
            serde_json::from_str(data)
                .map(Some)
                .map_err(CallbackQueryError::ParseJsonData)
        } else {
            Ok(None)
        }
    }
}

/// An error occurred in callback query
#[derive(Debug)]
pub enum CallbackQueryError {
    /// Failed to parse JSON data
    ParseJsonData(JsonError),
}

impl StdError for CallbackQueryError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            CallbackQueryError::ParseJsonData(err) => Some(err),
        }
    }
}

impl fmt::Display for CallbackQueryError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CallbackQueryError::ParseJsonData(err) => write!(out, "failed to parse callback query data: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Deserialize)]
    struct QueryData {
        k: String,
    }

    #[test]
    fn deserialize_full() {
        let mut data: CallbackQuery = serde_json::from_value(serde_json::json!({
            "id": "test",
            "from": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            },
            "message": {
                "message_id": 2,
                "date": 0,
                "from": {"id": 3, "first_name": "firstname", "is_bot": false},
                "chat": {"id": 4, "type": "supergroup", "title": "supergrouptitle"},
                "text": "test"
            },
            "inline_message_id": "inline message id",
            "chat_instance": "chat instance",
            "data": "{\"k\": \"v\"}",
            "game_short_name": "game short name"
        }))
        .unwrap();
        assert_eq!(data.id, "test");
        assert_eq!(data.from.id, 1);
        assert_eq!(data.from.first_name, "test");
        assert_eq!(data.from.is_bot, false);
        assert_eq!(data.message.take().unwrap().id, 2);
        assert_eq!(data.inline_message_id.take().unwrap(), "inline message id");
        assert_eq!(data.chat_instance.take().unwrap(), "chat instance");
        assert_eq!(data.data.take().unwrap(), r#"{"k": "v"}"#);
        assert_eq!(data.game_short_name.take().unwrap(), "game short name");
        data.data = Some(String::from(r#"{"k": "v"}"#));
        let parsed_query_data: QueryData = data.parse_data().unwrap().unwrap();
        assert_eq!(parsed_query_data.k, "v");
    }

    #[test]
    fn deserialize_partial() {
        let data: CallbackQuery = serde_json::from_value(serde_json::json!({
            "id": "test",
            "from": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            }
        }))
        .unwrap();
        assert_eq!(data.id, "test");
        assert_eq!(data.from.id, 1);
        assert_eq!(data.from.first_name, "test");
        assert_eq!(data.from.is_bot, false);
        assert!(data.message.is_none());
        assert!(data.inline_message_id.is_none());
        assert!(data.chat_instance.is_none());
        assert!(data.data.is_none());
        assert!(data.game_short_name.is_none());
    }
}
