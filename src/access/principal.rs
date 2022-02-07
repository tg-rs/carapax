use crate::types::{ChatId, Update, UserId};
use serde::{Deserialize, Serialize};

/// Allows to decide should rule accept an update or not
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Principal {
    /// Accepts all updates
    All,
    /// Accepts updates only from a specified user
    User(UserId),
    /// Accepts updates only from a specified chat
    Chat(ChatId),
    /// Accepts updates only from a user in chat
    ChatUser(ChatId, UserId),
}

impl From<UserId> for Principal {
    fn from(user_id: UserId) -> Principal {
        Principal::User(user_id)
    }
}

impl From<ChatId> for Principal {
    fn from(chat_id: ChatId) -> Principal {
        Principal::Chat(chat_id)
    }
}

impl From<(ChatId, UserId)> for Principal {
    fn from((chat_id, user_id): (ChatId, UserId)) -> Principal {
        Principal::ChatUser(chat_id, user_id)
    }
}

impl Principal {
    /// Creates a principal for user
    ///
    /// # Arguments
    ///
    /// * user_id - ID of the user
    pub fn user<T: Into<UserId>>(user_id: T) -> Self {
        Principal::User(user_id.into())
    }

    /// Creates a principal for chat
    ///
    /// # Arguments
    ///
    /// * chat_id - ID of the chat
    pub fn chat<T: Into<ChatId>>(chat_id: T) -> Self {
        Principal::Chat(chat_id.into())
    }

    /// Creates a principal for chat user
    ///
    /// # Arguments
    ///
    /// * chat_id - ID of the chat
    /// * user_id - ID of the user
    pub fn chat_user<C, U>(chat_id: C, user_id: U) -> Self
    where
        C: Into<ChatId>,
        U: Into<UserId>,
    {
        Principal::ChatUser(chat_id.into(), user_id.into())
    }

    pub(super) fn accepts(&self, update: &Update) -> bool {
        match self {
            Principal::User(user_id) => user_id.accepts(update),
            Principal::Chat(chat_id) => chat_id.accepts(update),
            Principal::ChatUser(chat_id, user_id) => chat_id.accepts(update) && user_id.accepts(update),
            Principal::All => true,
        }
    }
}

trait Resolver {
    fn accepts(&self, update: &Update) -> bool;
}

impl Resolver for UserId {
    fn accepts(&self, update: &Update) -> bool {
        match self {
            UserId::Id(user_id) => update.get_user().map(|u| u.id == *user_id),
            UserId::Username(ref username) => update
                .get_user()
                .and_then(|u| u.username.as_ref().map(|x| x == username)),
        }
        .unwrap_or(false)
    }
}

impl Resolver for ChatId {
    fn accepts(&self, update: &Update) -> bool {
        match self {
            ChatId::Id(chat_id) => update.get_chat_id().map(|x| x == *chat_id),
            ChatId::Username(ref chat_username) => update.get_chat_username().map(|x| x == chat_username),
        }
        .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Integer;

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
}
