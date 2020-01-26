use crate::types::poll::PollKind;
use serde::Serialize;
use std::ops::Not;

/// Custom keyboard with reply options
#[derive(Clone, Debug, Default, Serialize)]
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an Array of KeyboardButton objects
    keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(skip_serializing_if = "Not::not")]
    resize_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    one_time_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    selective: bool,
}

impl ReplyKeyboardMarkup {
    /// Returns a KeyboardMarkup with given keyboard
    pub fn from_vec(keyboard: Vec<Vec<KeyboardButton>>) -> Self {
        ReplyKeyboardMarkup {
            keyboard,
            resize_keyboard: false,
            one_time_keyboard: false,
            selective: false,
        }
    }

    /// Requests clients to resize the keyboard vertically for optimal fit
    ///
    /// (e.g., make the keyboard smaller if there are just two rows of buttons)
    /// Defaults to false, in which case the custom keyboard
    /// is always of the same height as the app's standard keyboard
    pub fn resize_keyboard(mut self, resize_keyboard: bool) -> Self {
        self.resize_keyboard = resize_keyboard;
        self
    }

    /// Requests clients to hide the keyboard as soon as it's been used
    ///
    /// The keyboard will still be available, but clients will automatically
    /// display the usual letter-keyboard in the chat – the user
    /// can press a special button in the input field to see the custom keyboard again
    /// Defaults to false
    pub fn one_time_keyboard(mut self, one_time_keyboard: bool) -> Self {
        self.one_time_keyboard = one_time_keyboard;
        self
    }

    /// Use this parameter if you want to show the keyboard to specific users only
    ///
    /// Targets:
    ///
    /// 1. users that are @mentioned in the text of the Message object;
    /// 2. if the bot's message is a reply (has reply_to_message_id), sender of the original message
    ///
    /// Example: A user requests to change the bot‘s language,
    /// bot replies to the request with a keyboard to select the new language
    /// Other users in the group don’t see the keyboard
    pub fn selective(mut self, selective: bool) -> Self {
        self.selective = selective;
        self
    }

    /// Adds a row to keyboard
    pub fn row(mut self, row: Vec<KeyboardButton>) -> Self {
        self.keyboard.push(row);
        self
    }
}

impl From<Vec<Vec<KeyboardButton>>> for ReplyKeyboardMarkup {
    fn from(keyboard: Vec<Vec<KeyboardButton>>) -> ReplyKeyboardMarkup {
        ReplyKeyboardMarkup::from_vec(keyboard)
    }
}

/// Button of the reply keyboard
#[derive(Clone, Debug, Serialize)]
pub struct KeyboardButton {
    text: String,
    #[serde(skip_serializing_if = "Not::not")]
    request_contact: bool,
    #[serde(skip_serializing_if = "Not::not")]
    request_location: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_poll: Option<KeyboardButtonPollType>,
}

impl KeyboardButton {
    /// Creates a new KeyboardButton
    ///
    /// # Arguments
    ///
    /// * text - Text of the button
    ///          If none of the optional fields are used,
    ///          it will be sent as a message when the button is pressed
    pub fn new<S: Into<String>>(text: S) -> Self {
        KeyboardButton {
            text: text.into(),
            request_contact: false,
            request_location: false,
            request_poll: None,
        }
    }

    /// The user's phone number will be sent as a contact when the button is pressed
    ///
    /// Available in private chats only
    pub fn request_contact(mut self) -> Self {
        self.request_contact = true;
        self.request_location = false;
        self.request_poll = None;
        self
    }

    /// The user's current location will be sent when the button is pressed
    ///
    /// Available in private chats only
    pub fn request_location(mut self) -> Self {
        self.request_location = true;
        self.request_contact = false;
        self.request_poll = None;
        self
    }

    /// The user will be asked to create a poll and send it to the bot when the button is pressed
    ///
    /// Available in private chats only
    ///
    /// If quiz is passed, the user will be allowed to create only polls in the quiz mode.
    /// If regular is passed, only regular polls will be allowed.
    /// Otherwise, the user will be allowed to create a poll of any type.
    pub fn request_poll<T>(mut self, button_type: T) -> Self
    where
        T: Into<KeyboardButtonPollType>,
    {
        self.request_poll = Some(button_type.into());
        self.request_contact = false;
        self.request_location = false;
        self
    }
}

/// This object represents type of a poll which is allowed to be created
/// and sent when the corresponding button is pressed
#[derive(Clone, Copy, Debug, Serialize, PartialEq, PartialOrd)]
pub struct KeyboardButtonPollType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    kind: Option<PollKind>,
}

impl From<PollKind> for KeyboardButtonPollType {
    fn from(kind: PollKind) -> Self {
        KeyboardButtonPollType { kind: Some(kind) }
    }
}

impl From<Option<PollKind>> for KeyboardButtonPollType {
    fn from(kind: Option<PollKind>) -> Self {
        KeyboardButtonPollType { kind }
    }
}

/// Requests clients to remove the custom keyboard
///
/// (user will not be able to summon this keyboard;
/// if you want to hide the keyboard from sight but keep it accessible,
/// use one_time_keyboard in ReplyKeyboardMarkup)
#[derive(Clone, Debug, Serialize)]
pub struct ReplyKeyboardRemove {
    remove_keyboard: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    selective: Option<bool>,
}

impl Default for ReplyKeyboardRemove {
    /// Returns an new keyboard
    fn default() -> ReplyKeyboardRemove {
        ReplyKeyboardRemove {
            remove_keyboard: true,
            selective: None,
        }
    }
}

impl ReplyKeyboardRemove {
    /// Use this parameter if you want to remove the keyboard for specific users only
    ///
    /// Targets:
    ///
    /// 1. users that are @mentioned in the text of the Message object;
    /// 2. if the bot's message is a reply (has reply_to_message_id), sender of the original message
    ///
    /// Example: A user votes in a poll, bot returns confirmation message
    /// in reply to the vote and removes the keyboard for that user,
    /// while still showing the keyboard with poll options to users who haven't voted yet
    pub fn selective(mut self, selective: bool) -> Self {
        self.selective = Some(selective);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ReplyMarkup;

    #[test]
    fn serialize() {
        let row = vec![
            KeyboardButton::new("test"),
            KeyboardButton::new("request contact").request_contact(),
            KeyboardButton::new("request location").request_location(),
            KeyboardButton::new("request quiz").request_poll(PollKind::Quiz),
            KeyboardButton::new("request regular poll").request_poll(PollKind::Regular),
            KeyboardButton::new("request any poll").request_poll(None),
        ];

        let markup = ReplyKeyboardMarkup::from(vec![row.clone()])
            .one_time_keyboard(true)
            .selective(true)
            .resize_keyboard(true);
        let data = serde_json::to_value(&ReplyMarkup::from(markup)).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "keyboard": [
                    [
                        {"text": "test"},
                        {"text": "request contact", "request_contact": true},
                        {"text": "request location", "request_location": true},
                        {"text": "request quiz", "request_poll": {"type": "quiz"}},
                        {"text": "request regular poll", "request_poll": {"type": "regular"}},
                        {"text": "request any poll", "request_poll": {}},
                    ]
                ],
                "resize_keyboard": true,
                "one_time_keyboard": true,
                "selective": true
            })
        );

        let markup: ReplyMarkup = ReplyKeyboardMarkup::default().row(row).into();
        let data = serde_json::to_value(&markup).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "keyboard": [
                    [
                        {"text": "test"},
                        {"text": "request contact","request_contact":true},
                        {"text": "request location","request_location":true},
                        {"text": "request quiz", "request_poll": {"type": "quiz"}},
                        {"text": "request regular poll", "request_poll": {"type": "regular"}},
                        {"text": "request any poll", "request_poll": {}},
                    ]
                ]
            })
        );

        let markup: ReplyMarkup = ReplyKeyboardRemove::default().selective(true).into();
        let j = serde_json::to_value(&markup).unwrap();
        assert_eq!(j, serde_json::json!({"remove_keyboard":true,"selective":true}));
    }
}
