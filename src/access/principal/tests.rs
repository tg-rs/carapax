use crate::types::Integer;

use super::*;

#[test]
fn convert_principal() {
    assert_eq!(Principal::from(UserId::from(1)), Principal::user(1));
    assert_eq!(Principal::from(ChatId::from(1)), Principal::chat(1));
    assert_eq!(
        Principal::from((ChatId::from(1), UserId::from(1))),
        Principal::chat_user(ChatId::from(1), UserId::from(1))
    );
}

#[test]
fn user_id_resolver() {
    let user_id = UserId::from(1);
    assert!(user_id.accepts(&UpdateBuilder::default().user_id(1).build()));
    assert!(!user_id.accepts(&UpdateBuilder::default().user_id(2).build()));

    let user_id = UserId::from("test");
    assert!(user_id.accepts(&UpdateBuilder::default().user_username("test").build()));
    assert!(!user_id.accepts(&UpdateBuilder::default().user_username("username_user").build()));
    assert!(!user_id.accepts(&UpdateBuilder::default().build()));
}

#[test]
fn chat_id_resolver() {
    let chat_id = ChatId::from(1);
    assert!(chat_id.accepts(&UpdateBuilder::default().chat_id(1).build()));
    assert!(!chat_id.accepts(&UpdateBuilder::default().chat_id(2).build()));

    let chat_id = ChatId::from("test");
    assert!(chat_id.accepts(&UpdateBuilder::default().chat_username("test").build()));
    assert!(!chat_id.accepts(&UpdateBuilder::default().chat_username("username_chat").build()));
    assert!(!chat_id.accepts(&UpdateBuilder::default().build()));
}

#[test]
fn chat_id_and_user_id_resolvers() {
    let principal = Principal::from((ChatId::from(1), UserId::from(1)));
    assert_eq!(principal, Principal::chat_user(ChatId::from(1), UserId::from(1)));
    assert!(principal.accepts(&UpdateBuilder::default().user_id(1).chat_id(1).build()));
    assert!(!principal.accepts(&UpdateBuilder::default().build()));
}

#[derive(Default)]
struct UpdateBuilder {
    user_id: Integer,
    user_username: Option<String>,
    chat_id: Integer,
    chat_username: Option<String>,
}

impl UpdateBuilder {
    fn user_id(mut self, user_id: Integer) -> Self {
        self.user_id = user_id;
        self
    }

    fn user_username<T>(mut self, user_username: T) -> Self
    where
        T: Into<String>,
    {
        self.user_username = Some(user_username.into());
        self
    }

    fn chat_id(mut self, chat_id: Integer) -> Self {
        self.chat_id = chat_id;
        self
    }

    fn chat_username<T>(mut self, chat_username: T) -> Self
    where
        T: Into<String>,
    {
        self.chat_username = Some(chat_username.into());
        self
    }

    fn build(self) -> Update {
        serde_json::from_value::<Update>(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 1,
                "from": {"id": self.user_id, "is_bot": false, "first_name": "test", "username": self.user_username},
                "chat": {"id": self.chat_id, "type": "supergroup", "title": "test", "username": self.chat_username},
                "text": "test"
            }
        }))
        .unwrap()
    }
}
