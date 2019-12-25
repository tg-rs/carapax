use crate::{
    methods::Method,
    request::Request,
    types::{GameHighScore, Integer},
};
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

    fn into_request(self) -> Request {
        Request::json("getGameHighScores", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn get_game_high_scores() {
        let request = GetGameHighScores::new(1, 2, 3).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/getGameHighScores"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["user_id"], 1);
            assert_eq!(data["chat_id"], 2);
            assert_eq!(data["message_id"], 3);
        } else {
            panic!("Unexpected request body");
        }

        let request = GetGameHighScores::with_inline_message_id(1, "msg-id").into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/getGameHighScores"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["user_id"], 1);
            assert_eq!(data["inline_message_id"], "msg-id");
        } else {
            panic!("Unexpected request body");
        }
    }
}
