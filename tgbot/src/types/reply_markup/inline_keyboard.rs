use crate::types::login_url::LoginUrl;
use failure::Error;
use serde::Serialize;

/// Inline keyboard that appears right next to the message it belongs to
#[derive(Clone, Debug, Default, Serialize)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    /// Returns a KeyboardMarkup with given keyboard
    pub fn from_vec(inline_keyboard: Vec<Vec<InlineKeyboardButton>>) -> Self {
        InlineKeyboardMarkup { inline_keyboard }
    }

    /// Adds a row to keyboard
    pub fn row(mut self, row: Vec<InlineKeyboardButton>) -> Self {
        self.inline_keyboard.push(row);
        self
    }
}

impl From<Vec<Vec<InlineKeyboardButton>>> for InlineKeyboardMarkup {
    fn from(keyboard: Vec<Vec<InlineKeyboardButton>>) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::from_vec(keyboard)
    }
}

/// Button of an inline keyboard
///
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
    #[serde(skip_serializing_if = "Option::is_none")]
    login_url: Option<LoginUrl>,
}

impl InlineKeyboardButton {
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
            login_url: None,
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
            login_url: None,
        }
    }

    /// Same as with_callback_data, but takes a serializable type
    ///
    /// Data will be serialized using serde_json
    pub fn with_callback_data_struct<S: Into<String>, D: Serialize>(text: S, callback_data: &D) -> Result<Self, Error> {
        Ok(InlineKeyboardButton {
            text: text.into(),
            url: None,
            callback_data: Some(serde_json::to_string(callback_data)?),
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
            login_url: None,
        })
    }

    /// Pressing the button will prompt the user to select one of their chats,
    /// open that chat and insert the bot‘s username and
    /// the specified inline query in the input field
    ///
    /// Can be empty, in which case just the bot’s username will be inserted
    ///
    /// Note: This offers an easy way for users to start using your bot
    /// in inline mode when they are currently in a private chat with it
    ///
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
            login_url: None,
        }
    }

    /// If set, pressing the button will insert the bot‘s username and
    /// the specified inline query in the current chat's input field
    ///
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
            login_url: None,
        }
    }

    /// Description of the game that will be launched when the user presses the button
    ///
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
            login_url: None,
        }
    }

    /// Send a Pay button
    ///
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
            login_url: None,
        }
    }

    /// An HTTP URL used to automatically authorize the user
    ///
    /// Can be used as a replacement for the [Telegram Login Widget][1]
    ///
    /// [1]: https://core.telegram.org/widgets/login
    pub fn with_login_url<T, U>(text: T, login_url: U) -> Self
    where
        T: Into<String>,
        U: Into<LoginUrl>,
    {
        InlineKeyboardButton {
            text: text.into(),
            url: None,
            callback_data: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
            login_url: Some(login_url.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ReplyMarkup;

    #[derive(Serialize)]
    struct CallbackData {
        value: String,
    }

    #[test]
    fn serialize() {
        let callback_data = CallbackData {
            value: String::from("cdstruct"),
        };

        let markup: ReplyMarkup = vec![vec![
            InlineKeyboardButton::with_url("url", "tg://user?id=1"),
            InlineKeyboardButton::with_callback_data("cd", "cd"),
            InlineKeyboardButton::with_callback_data_struct("cd", &callback_data).unwrap(),
            InlineKeyboardButton::with_switch_inline_query("siq", "siq"),
            InlineKeyboardButton::with_switch_inline_query_current_chat("siqcc", "siqcc"),
            InlineKeyboardButton::with_callback_game("cg"),
            InlineKeyboardButton::with_pay("pay"),
            InlineKeyboardButton::with_login_url("login url", "http://example.com"),
        ]]
        .into();
        let data = serde_json::to_value(&markup).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "inline_keyboard": [
                    [
                        {"text":"url","url":"tg://user?id=1"},
                        {"text":"cd","callback_data":"cd"},
                        {"text":"cd","callback_data":"{\"value\":\"cdstruct\"}"},
                        {"text":"siq","switch_inline_query":"siq"},
                        {"text":"siqcc","switch_inline_query_current_chat":"siqcc"},
                        {"text":"cg","callback_game":""},
                        {"text":"pay","pay":true},
                        {"text":"login url","login_url":{"url":"http://example.com"}}
                    ]
                ]
            })
        );
    }
}
