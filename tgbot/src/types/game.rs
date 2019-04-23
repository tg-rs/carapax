use crate::types::{
    animation::Animation,
    message::{RawMessageEntity, Text},
    photo_size::PhotoSize,
    primitive::Integer,
    user::User,
};
use serde::{de::Error, Deserialize, Deserializer};

/// Game
///
/// Use BotFather to create and edit games,
/// their short names will act as unique identifiers
#[derive(Clone, Debug)]
pub struct Game {
    /// Title of the game
    pub title: String,
    /// Description of the game
    pub description: String,
    /// Photo that will be displayed in the game message in chats
    pub photo: Vec<PhotoSize>,
    /// Brief description of the game or high scores included in the game message
    /// Can be automatically edited to include current high scores for the game
    /// when the bot calls setGameScore, or manually edited using editMessageText
    /// 0-4096 characters
    pub text: Option<Text>,
    /// Animation that will be displayed in the game message in chats
    /// Upload via BotFather
    pub animation: Option<Animation>,
}

impl<'de> Deserialize<'de> for Game {
    fn deserialize<D>(deserializer: D) -> Result<Game, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_game: RawGame = Deserialize::deserialize(deserializer)?;
        Ok(Game {
            title: raw_game.title,
            description: raw_game.description,
            photo: raw_game.photo,
            text: match raw_game.text {
                Some(data) => Some(Text::parse(data, raw_game.text_entities).map_err(D::Error::custom)?),
                None => None,
            },
            animation: raw_game.animation,
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawGame {
    title: String,
    description: String,
    photo: Vec<PhotoSize>,
    text: Option<String>,
    text_entities: Option<Vec<RawMessageEntity>>,
    animation: Option<Animation>,
}

/// One row of the high scores table for a game
#[derive(Clone, Debug, Deserialize)]
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: Integer,
    /// User
    pub user: User,
    /// Score
    pub score: Integer,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_game_full() {
        let game: Game = serde_json::from_value(serde_json::json!({
            "title": "title",
            "description": "description",
            "photo": [
                {
                    "file_id": "photo file id",
                    "width": 200,
                    "height": 200
                }
            ],
            "text": "text",
            "animation": {
                "file_id": "animation file id",
                "width": 200,
                "height": 200,
                "duration": 24
            }
        }))
        .unwrap();
        assert_eq!(game.title, "title");
        assert_eq!(game.description, "description");
        assert_eq!(game.photo.len(), 1);
        assert_eq!(game.photo[0].file_id, "photo file id");
        assert_eq!(game.text.unwrap().data, "text");
        assert_eq!(game.animation.unwrap().file_id, "animation file id");
    }

    #[test]
    fn deserialize_game_partial() {
        let game: Game = serde_json::from_value(serde_json::json!({
            "title": "title",
            "description": "description",
            "photo": []
        }))
        .unwrap();
        assert_eq!(game.title, "title");
        assert_eq!(game.description, "description");
        assert_eq!(game.photo.len(), 0);
        assert!(game.text.is_none());
        assert!(game.animation.is_none());
    }

    #[test]
    fn deserialize_game_high_score() {
        let score: GameHighScore = serde_json::from_value(serde_json::json!({
            "position": 1,
            "user": {
                "id": 2,
                "first_name": "test",
                "is_bot": false
            },
            "score": 3
        }))
        .unwrap();
        assert_eq!(score.position, 1);
        assert_eq!(score.user.id, 2);
        assert_eq!(score.score, 3);
    }
}
