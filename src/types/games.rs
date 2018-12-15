use crate::types::animation::Animation;
use crate::types::message_entity::MessageEntity;
use crate::types::photo_size::PhotoSize;
use crate::types::primitive::Integer;
use crate::types::user::User;

/// This object represents a game.
/// Use BotFather to create and edit games,
/// their short names will act as unique identifiers.
#[derive(Debug)]
pub struct Game {
    /// Title of the game
    pub title: String,
    /// Description of the game
    pub description: String,
    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,
    /// Brief description of the game or high scores included in the game message.
    /// Can be automatically edited to include current high scores for the game
    /// when the bot calls setGameScore, or manually edited using editMessageText.
    /// 0-4096 characters.
    pub text: Option<String>,
    /// Special entities that appear in text, such as usernames, URLs, bot commands, etc.
    pub text_entities: Option<Vec<MessageEntity>>,
    /// Animation that will be displayed in the game message in chats.
    /// Upload via BotFather
    pub animation: Option<Animation>,
}

/// This object represents one row of the high scores table for a game.
#[derive(Debug)]
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: Integer,
    /// User
    pub user: User,
    /// Score
    pub score: Integer,
}
