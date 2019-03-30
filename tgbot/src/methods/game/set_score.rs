use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{EditMessageResult, Integer},
};
use failure::Error;
use serde::Serialize;

/// Set the score of the specified user in a game
///
/// Returns an error, if the new score is not greater
/// than the user's current score in the chat and force is False
#[derive(Clone, Debug, Serialize)]
pub struct SetGameScore {
    user_id: Integer,
    score: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_edit_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
}

impl SetGameScore {
    /// Creates a new SetGameScore
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    /// * user_id - User identifier
    /// * score - New score, must be non-negative
    pub fn new(chat_id: Integer, message_id: Integer, user_id: Integer, score: Integer) -> Self {
        SetGameScore {
            user_id,
            score,
            force: None,
            disable_edit_message: None,
            chat_id: Some(chat_id),
            message_id: Some(message_id),
            inline_message_id: None,
        }
    }

    /// Creates a new SetGameScore
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    /// * user_id - User identifier
    /// * score - New score, must be non-negative
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S, user_id: Integer, score: Integer) -> Self {
        SetGameScore {
            user_id,
            score,
            force: None,
            disable_edit_message: None,
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
        }
    }

    /// Pass True, if the high score is allowed to decrease
    ///
    /// This can be useful when fixing mistakes or banning cheaters
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }

    /// Pass True, if the game message should not be automatically
    /// edited to include the current scoreboard
    pub fn disable_edit_message(mut self, disable_edit_message: bool) -> Self {
        self.disable_edit_message = Some(disable_edit_message);
        self
    }
}

impl Method for SetGameScore {
    type Response = EditMessageResult;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("setGameScore", &self)
    }
}
