use crate::methods::method::*;
use crate::types::{InlineKeyboardMarkup, Integer, Message};
use failure::Error;
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
    pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(&mut self, reply_to_message_id: Integer) -> &mut Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Additional interface options
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(&mut self, reply_markup: I) -> &mut Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendGame {
    type Response = Message;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("sendGame", &self)
    }
}
