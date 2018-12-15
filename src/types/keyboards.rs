use crate::types::message::Message;
use crate::types::user::User;

/// This object represents an incoming callback query
/// from a callback button in an inline keyboard.
/// If the button that originated the query was attached to a message sent by the bot,
/// the field message will be present. If the button was attached
/// to a message sent via the bot (in inline mode),
/// the field inline_message_id will be present.
/// Exactly one of the fields data or game_short_name will be present.
#[derive(Debug)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Message with the callback button that originated the query.
    /// Note that message content and message date
    /// will not be available if the message is too old
    pub message: Option<Message>,
    /// Identifier of the message sent via the bot
    /// in inline mode, that originated the query.
    pub inline_message_id: Option<String>,
    /// Global identifier, uniquely corresponding
    /// to the chat to which the message with the
    /// callback button was sent.
    /// Useful for high scores in games.
    pub chat_instance: Option<String>,
    /// Data associated with the callback button.
    /// Be aware that a bad client can send arbitrary data in this field.
    pub data: Option<String>,
    /// Short name of a Game to be returned,
    /// serves as the unique identifier for the game
    pub game_short_name: Option<String>,
}

/// This object represents one button of an inline keyboard. You must use exactly one of the optional fields.
#[derive(Debug)]
pub struct InlineKeyboardButton {
    /// Label text on the button
    pub text: String,
    /// HTTP or tg:// url to be opened when button is pressed
    pub url: Option<String>,
    /// Data to be sent in a callback query to the bot when button is pressed, 1-64 bytes
    pub callback_data: Option<String>,
    /// If set, pressing the button will prompt the user to select one of their chats,
    /// open that chat and insert the bot‘s username and
    /// the specified inline query in the input field.
    /// Can be empty, in which case just the bot’s username will be inserted.
    /// Note: This offers an easy way for users to start using your bot
    /// in inline mode when they are currently in a private chat with it.
    /// Especially useful when combined with switch_pm… actions – in this case the user
    /// will be automatically returned to the chat they switched from,
    /// skipping the chat selection screen.
    pub switch_inline_query: Option<String>,
    /// If set, pressing the button will insert the bot‘s username and
    /// the specified inline query in the current chat's input field.
    /// Can be empty, in which case only the bot’s username will be inserted.
    /// This offers a quick way for the user to open your bot in
    /// inline mode in the same chat – good for selecting something from multiple options.
    pub switch_inline_query_current_chat: Option<String>,
    /// Description of the game that will be launched when the user presses the button.
    /// NOTE: This type of button must always be the first button in the first row.
    // pub callback_game: Option<CallbackGame>,
    /// Specify True, to send a Pay button.
    /// NOTE: This type of button must always be the first button in the first row.
    pub pay: Option<bool>,
}

/// This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Debug)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of InlineKeyboardButton objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

/// This object represents one button of the reply keyboard.
/// For simple text buttons String can be used instead of this object to specify text of the button.
/// Optional fields are mutually exclusive.
#[derive(Debug)]
pub struct KeyboardButton {
    /// Text of the button.
    /// If none of the optional fields are used,
    /// it will be sent as a message when the button is pressed
    pub text: String,
    /// If True, the user's phone number will be sent as a
    /// contact when the button is pressed.
    /// Available in private chats only
    pub request_contact: Option<bool>,
    /// If True, the user's current location will be sent
    // when the button is pressed.
    /// Available in private chats only
    pub request_location: Option<bool>,
}

#[derive(Debug)]
/// This object represents a custom keyboard with reply options
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an Array of KeyboardButton objects
    pub keyboard: Vec<Vec<KeyboardButton>>,
    /// Requests clients to resize the keyboard vertically for optimal fit
    /// (e.g., make the keyboard smaller if there are just two rows of buttons).
    /// Defaults to false, in which case the custom keyboard
    /// is always of the same height as the app's standard keyboard.
    pub resize_keyboard: Option<bool>,
    /// Requests clients to hide the keyboard as soon as it's been used.
    /// The keyboard will still be available, but clients will automatically
    /// display the usual letter-keyboard in the chat – the user
    /// can press a special button in the input field to see the custom keyboard again.
    /// Defaults to false.
    pub one_time_keyboard: Option<bool>,
    /// Use this parameter if you want to show the keyboard to specific users only.
    /// Targets:
    /// 1) users that are @mentioned in the text of the Message object;
    /// 2) if the bot's message is a reply (has reply_to_message_id), sender of the original message.
    /// Example: A user requests to change the bot‘s language,
    /// bot replies to the request with a keyboard to select the new language.
    /// Other users in the group don’t see the keyboard.
    pub selective: Option<bool>,
}

/// Upon receiving a message with this object,
/// Telegram clients will remove the current custom keyboard and display the default letter-keyboard.
/// By default, custom keyboards are displayed until a new keyboard is sent by a bot.
/// An exception is made for one-time keyboards that are hidden immediately after
/// the user presses a button (see ReplyKeyboardMarkup).
#[derive(Debug)]
pub struct ReplyKeyboardRemove {
    /// Requests clients to remove the custom keyboard
    /// (user will not be able to summon this keyboard;
    /// if you want to hide the keyboard from sight but keep it accessible,
    /// use one_time_keyboard in ReplyKeyboardMarkup)
    pub remove_keyboard: bool,
    /// Use this parameter if you want to remove the keyboard for specific users only.
    /// Targets:
    /// 1) users that are @mentioned in the text of the Message object;
    /// 2) if the bot's message is a reply (has reply_to_message_id), sender of the original message.
    /// Example: A user votes in a poll, bot returns confirmation message
    /// in reply to the vote and removes the keyboard for that user,
    /// while still showing the keyboard with poll options to users who haven't voted yet.
    pub selective: Option<bool>,
}
