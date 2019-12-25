use crate::{
    methods::Method,
    request::Request,
    types::{InlineKeyboardMarkup, Integer, Message},
};
use serde::Serialize;

/// Use this method to send a game
#[derive(Clone, Debug, Serialize)]
pub struct SendGame {
    chat_id: Integer,
    game_short_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl SendGame {
    /// Creates a new SendGame with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * game_short_name - Short name of the game, serves as the unique identifier for the game
    pub fn new<S: Into<String>>(chat_id: Integer, game_short_name: S) -> Self {
        SendGame {
            chat_id,
            game_short_name: game_short_name.into(),
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Additional interface options
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendGame {
    type Response = Message;

    fn into_request(self) -> Request {
        Request::json("sendGame", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::InlineKeyboardButton,
    };
    use serde_json::Value;

    #[test]
    fn send_game() {
        let request = SendGame::new(1, "name")
            .disable_notification(true)
            .reply_to_message_id(1)
            .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendGame");
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["game_short_name"], "name");
            assert_eq!(data["disable_notification"], true);
            assert_eq!(data["reply_to_message_id"], 1);
            assert_eq!(data["reply_markup"]["inline_keyboard"][0][0]["text"], "text");
        } else {
            panic!("Unexpected request body");
        }
    }
}
