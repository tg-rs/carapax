/// This object represents an inline keyboard that appears right next to the message it belongs to
#[derive(Clone, Debug, Serialize)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    /// Returns an empty KeyboardMarkup
    pub fn new() -> Self {
        InlineKeyboardMarkup {
            inline_keyboard: Vec::new(),
        }
    }

    /// Returns a KeyboardMarkup with given keyboard
    pub fn with_keyboard(inline_keyboard: Vec<Vec<InlineKeyboardButton>>) -> Self {
        InlineKeyboardMarkup { inline_keyboard }
    }

    /// Adds a row to keyboard
    pub fn add_row(&mut self, row: Vec<InlineKeyboardButton>) -> &mut Self {
        self.inline_keyboard.push(row);
        self
    }
}

impl From<Vec<Vec<InlineKeyboardButton>>> for InlineKeyboardMarkup {
    fn from(keyboard: Vec<Vec<InlineKeyboardButton>>) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::with_keyboard(keyboard)
    }
}

/// This object represents one button of an inline keyboard
/// You must use exactly one of the optional fields
#[derive(Clone, Debug, Serialize)]
pub struct InlineKeyboardButton {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    callback_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_inline_query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_inline_query_current_chat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    callback_game: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pay: Option<bool>,
}

impl InlineKeyboardButton {
    /// Returns a plain keyboard button
    pub fn new<S: Into<String>>(text: S) -> Self {
        InlineKeyboardButton {
            text: text.into(),
            url: None,
            callback_data: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
        }
    }

    /// HTTP or tg:// url to be opened when button is pressed
    pub fn with_url<S: Into<String>>(text: S, url: S) -> Self {
        InlineKeyboardButton {
            text: text.into(),
            url: Some(url.into()),
            callback_data: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
        }
    }

    /// Data to be sent in a callback query to the bot when button is pressed, 1-64 bytes
    pub fn with_callback_data<S: Into<String>>(text: S, callback_data: S) -> Self {
        InlineKeyboardButton {
            text: text.into(),
            url: None,
            callback_data: Some(callback_data.into()),
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
        }
    }

    /// Pressing the button will prompt the user to select one of their chats,
    /// open that chat and insert the bot‘s username and
    /// the specified inline query in the input field
    /// Can be empty, in which case just the bot’s username will be inserted
    /// Note: This offers an easy way for users to start using your bot
    /// in inline mode when they are currently in a private chat with it
    /// Especially useful when combined with switch_pm… actions – in this case the user
    /// will be automatically returned to the chat they switched from,
    /// skipping the chat selection screen
    pub fn with_switch_inline_query<S: Into<String>>(text: S, switch_inline_query: S) -> Self {
        InlineKeyboardButton {
            text: text.into(),
            url: None,
            callback_data: None,
            switch_inline_query: Some(switch_inline_query.into()),
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
        }
    }

    /// If set, pressing the button will insert the bot‘s username and
    /// the specified inline query in the current chat's input field
    /// Can be empty, in which case only the bot’s username will be inserted
    /// This offers a quick way for the user to open your bot in
    /// inline mode in the same chat – good for selecting something from multiple options
    pub fn with_switch_inline_query_current_chat<S: Into<String>>(
        text: S,
        switch_inline_query_current_chat: S,
    ) -> Self {
        InlineKeyboardButton {
            text: text.into(),
            url: None,
            callback_data: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: Some(switch_inline_query_current_chat.into()),
            callback_game: None,
            pay: None,
        }
    }

    /// Description of the game that will be launched when the user presses the button
    /// NOTE: This type of button must always be the first button in the first row
    pub fn with_callback_game<S: Into<String>>(text: S) -> Self {
        InlineKeyboardButton {
            text: text.into(),
            url: None,
            callback_data: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: Some(String::new()),
            pay: None,
        }
    }

    /// Send a Pay button
    /// NOTE: This type of button must always be the first button in the first row
    pub fn with_pay<S: Into<String>>(text: S) -> Self {
        InlineKeyboardButton {
            text: text.into(),
            url: None,
            callback_data: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: Some(true),
        }
    }
}
