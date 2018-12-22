use crate::types::reply_markup::InlineKeyboardMarkup;

/// Game
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultGame {
    id: String,
    game_short_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl InlineQueryResultGame {
    /// Creates a new InlineQueryResultGame with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * game_short_name - Short name of the game
    pub fn new<S: Into<String>>(id: S, game_short_name: S) -> Self {
        InlineQueryResultGame {
            id: id.into(),
            game_short_name: game_short_name.into(),
            reply_markup: None,
        }
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup(&mut self, reply_markup: InlineKeyboardMarkup) -> &mut Self {
        self.reply_markup = Some(reply_markup);
        self
    }
}
