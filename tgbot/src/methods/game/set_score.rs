use crate::{
    methods::Method,
    request::Request,
    types::{EditMessageResult, Integer},
};
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

    fn into_request(self) -> Request {
        Request::json("setGameScore", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn set_game_score() {
        let request = SetGameScore::new(1, 2, 3, 100)
            .force(true)
            .disable_edit_message(true)
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/setGameScore");
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["message_id"], 2);
            assert_eq!(data["user_id"], 3);
            assert_eq!(data["score"], 100);
            assert_eq!(data["force"], true);
            assert_eq!(data["disable_edit_message"], true);
        } else {
            panic!("Unexpected request body");
        }

        let request = SetGameScore::with_inline_message_id("msg-id", 3, 100)
            .force(true)
            .disable_edit_message(true)
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/setGameScore");
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["inline_message_id"], "msg-id");
            assert_eq!(data["user_id"], 3);
            assert_eq!(data["score"], 100);
            assert_eq!(data["force"], true);
            assert_eq!(data["disable_edit_message"], true);
        } else {
            panic!("Unexpected request body");
        }
    }
}
