use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{GameHighScore, Integer},
};
use failure::Error;
use serde::Serialize;

/// Get data for high score tables
///
/// Will return the score of the specified user and several of his neighbors in a game
/// This method will currently return scores for the target user,
/// plus two of his closest neighbors on each side
/// Will also return the top three users if the user and his neighbors are not among them
/// Please note that this behavior is subject to change
#[derive(Clone, Debug, Serialize)]
pub struct GetGameHighScores {
    user_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
}

impl GetGameHighScores {
    /// Creates a new GetGameHighScores
    ///
    /// # Arguments
    ///
    /// * user_id - Target user id
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    pub fn new(user_id: Integer, chat_id: Integer, message_id: Integer) -> Self {
        GetGameHighScores {
            user_id,
            chat_id: Some(chat_id),
            message_id: Some(message_id),
            inline_message_id: None,
        }
    }

    /// Creates a new GetGameHighScores
    ///
    /// # Arguments
    ///
    /// * user_id - Target user id
    /// * inline_message_id - Identifier of the inline message
    pub fn with_inline_message_id<S: Into<String>>(user_id: Integer, inline_message_id: S) -> Self {
        GetGameHighScores {
            user_id,
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
        }
    }
}

impl Method for GetGameHighScores {
    type Response = Vec<GameHighScore>;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("getGameHighScores", &self)
    }
}
