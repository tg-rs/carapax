use crate::types::{Integer, Update};

/// Allows to decide should rule accept an update or not
#[derive(Clone, Debug, PartialEq)]
pub enum Principal {
    /// Accepts all updates
    All,
    /// Accepts updates only from a specified user
    User(PrincipalUser),
    /// Accepts updates only from a specified chat
    Chat(PrincipalChat),
    /// Accepts updates only from a user in chat
    ChatUser(PrincipalChat, PrincipalUser),
}

impl From<PrincipalUser> for Principal {
    fn from(principal: PrincipalUser) -> Principal {
        Principal::User(principal)
    }
}

impl From<PrincipalChat> for Principal {
    fn from(principal: PrincipalChat) -> Principal {
        Principal::Chat(principal)
    }
}

impl From<(PrincipalChat, PrincipalUser)> for Principal {
    fn from(principal: (PrincipalChat, PrincipalUser)) -> Principal {
        Principal::ChatUser(principal.0, principal.1)
    }
}

impl Principal {
    /// Creates a principal for user
    ///
    /// # Arguments
    ///
    /// * principal - User principal
    pub fn user<P: Into<PrincipalUser>>(principal: P) -> Self {
        Principal::User(principal.into())
    }

    /// Creates a principal for chat
    ///
    /// # Arguments
    ///
    /// * principal - Chat principal
    pub fn chat<P: Into<PrincipalChat>>(principal: P) -> Self {
        Principal::Chat(principal.into())
    }

    /// Creates a principal for chat user
    ///
    /// # Arguments
    ///
    /// * principal - Chat user principal
    pub fn chat_user<C, U>(chat: C, user: U) -> Self
    where
        C: Into<PrincipalChat>,
        U: Into<PrincipalUser>,
    {
        Principal::ChatUser(chat.into(), user.into())
    }

    pub(super) fn accepts(&self, update: &Update) -> bool {
        match self {
            Principal::User(principal) => principal.accepts(update),
            Principal::Chat(principal) => principal.accepts(update),
            Principal::ChatUser(chat_principal, user_principal) => {
                chat_principal.accepts(update) && user_principal.accepts(update)
            }
            Principal::All => true,
        }
    }
}

/// Represents a user
#[derive(Clone, Debug, PartialEq)]
pub enum PrincipalUser {
    /// Accepts updates only from a user with specified ID
    Id(Integer),
    /// Accepts updates only from a user with specified @username
    Username(String),
}

impl From<Integer> for PrincipalUser {
    fn from(user_id: Integer) -> PrincipalUser {
        PrincipalUser::Id(user_id)
    }
}

impl From<String> for PrincipalUser {
    fn from(username: String) -> PrincipalUser {
        PrincipalUser::Username(username)
    }
}

impl<'a> From<&'a str> for PrincipalUser {
    fn from(username: &'a str) -> PrincipalUser {
        PrincipalUser::Username(String::from(username))
    }
}

impl PrincipalUser {
    fn accepts(&self, update: &Update) -> bool {
        match self {
            PrincipalUser::Id(user_id) => update.get_user().map(|u| u.id == *user_id),
            PrincipalUser::Username(ref username) => update
                .get_user()
                .and_then(|u| u.username.as_ref().map(|x| x == username)),
        }
        .unwrap_or(false)
    }
}

/// Represents a chat
#[derive(Clone, Debug, PartialEq)]
pub enum PrincipalChat {
    /// Accepts updates only from a chat with specified ID
    Id(Integer),
    /// Accepts updates only from a chat with specified @username
    Username(String),
}

impl From<Integer> for PrincipalChat {
    fn from(chat_id: Integer) -> PrincipalChat {
        PrincipalChat::Id(chat_id)
    }
}

impl From<String> for PrincipalChat {
    fn from(username: String) -> PrincipalChat {
        PrincipalChat::Username(username)
    }
}

impl<'a> From<&'a str> for PrincipalChat {
    fn from(username: &'a str) -> PrincipalChat {
        PrincipalChat::Username(String::from(username))
    }
}

impl PrincipalChat {
    fn accepts(&self, update: &Update) -> bool {
        match self {
            PrincipalChat::Id(chat_id) => update.get_chat_id().map(|x| x == *chat_id),
            PrincipalChat::Username(ref chat_username) => update.get_chat_username().map(|x| x == chat_username),
        }
        .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn principal() {
        assert_eq!(Principal::from(PrincipalUser::from(1)), Principal::user(1));
        assert_eq!(Principal::from(PrincipalChat::from(1)), Principal::chat(1));

        let principal = Principal::from((PrincipalChat::from(1), PrincipalUser::from(1)));
        assert_eq!(
            principal,
            Principal::chat_user(PrincipalChat::from(1), PrincipalUser::from(1))
        );
        assert!(principal.accepts(
            &serde_json::from_value::<Update>(serde_json::json!({
                "update_id": 1,
                "message": {
                    "message_id": 1,
                    "date": 1,
                    "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                    "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                    "text": "test"
                }
            }))
            .unwrap()
        ));
        assert!(!principal.accepts(
            &serde_json::from_value::<Update>(serde_json::json!({
                "update_id": 1,
                "message": {
                    "message_id": 1,
                    "date": 1,
                    "from": {"id": 2, "is_bot": false, "first_name": "test", "username": "username_user"},
                    "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                    "text": "test"
                }
            }))
            .unwrap()
        ));
        assert!(!principal.accepts(
            &serde_json::from_value::<Update>(serde_json::json!({
                "update_id": 1,
                "message": {
                    "message_id": 1,
                    "date": 1,
                    "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                    "chat": {"id": 2, "type": "supergroup", "title": "test", "username": "username_chat"},
                    "text": "test"
                }
            }))
            .unwrap()
        ));
    }

    #[test]
    fn principal_user() {
        let principal = PrincipalUser::from(1);
        assert_eq!(principal, PrincipalUser::Id(1));
        assert!(principal.accepts(
            &serde_json::from_value::<Update>(serde_json::json!({
                "update_id": 1,
                "message": {
                    "message_id": 1,
                    "date": 1,
                    "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                    "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                    "text": "test"
                }
            }))
            .unwrap()
        ));
        assert!(!principal.accepts(
            &serde_json::from_value::<Update>(serde_json::json!({
                "update_id": 1,
                "message": {
                    "message_id": 1,
                    "date": 1,
                    "from": {"id": 2, "is_bot": false, "first_name": "test", "username": "username_user"},
                    "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                    "text": "test"
                }
            }))
            .unwrap()
        ));

        assert_eq!(
            PrincipalUser::from(String::from("test")),
            PrincipalUser::Username(String::from("test"))
        );

        let principal = PrincipalUser::from("test");
        assert_eq!(principal, PrincipalUser::Username(String::from("test")));
        assert!(principal.accepts(
            &serde_json::from_value::<Update>(serde_json::json!({
                "update_id": 1,
                "message": {
                    "message_id": 1,
                    "date": 1,
                    "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "test"},
                    "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                    "text": "test"
                }
            }))
            .unwrap()
        ));
        assert!(!principal.accepts(
            &serde_json::from_value::<Update>(serde_json::json!({
                "update_id": 1,
                "message": {
                    "message_id": 1,
                    "date": 1,
                    "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                    "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                    "text": "test"
                }
            }))
            .unwrap()
        ));
        assert!(!principal.accepts(
            &serde_json::from_value::<Update>(serde_json::json!({
                "update_id": 1,
                "message": {
                    "message_id": 1,
                    "date": 1,
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                    "text": "test"
                }
            }))
            .unwrap()
        ));
    }

    #[test]
    fn principal_chat() {
        let principal = PrincipalChat::from(1);
        assert_eq!(principal, PrincipalChat::Id(1));
        assert!(principal.accepts(
            &serde_json::from_value::<Update>(serde_json::json!({
                "update_id": 1,
                "message": {
                    "message_id": 1,
                    "date": 1,
                    "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                    "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                    "text": "test"
                }
            }))
            .unwrap()
        ));
        assert!(!principal.accepts(
            &serde_json::from_value::<Update>(serde_json::json!({
                "update_id": 1,
                "message": {
                    "message_id": 1,
                    "date": 1,
                    "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                    "chat": {"id": 2, "type": "supergroup", "title": "test", "username": "username_chat"},
                    "text": "test"
                }
            }))
            .unwrap()
        ));

        assert_eq!(
            PrincipalChat::from(String::from("test")),
            PrincipalChat::Username(String::from("test"))
        );

        let principal = PrincipalChat::from("test");
        assert_eq!(principal, PrincipalChat::Username(String::from("test")));
        assert!(principal.accepts(
            &serde_json::from_value::<Update>(serde_json::json!({
                "update_id": 1,
                "message": {
                    "message_id": 1,
                    "date": 1,
                    "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                    "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "test"},
                    "text": "test"
                }
            }))
            .unwrap()
        ));
        assert!(!principal.accepts(
            &serde_json::from_value::<Update>(serde_json::json!({
                "update_id": 1,
                "message": {
                    "message_id": 1,
                    "date": 1,
                    "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                    "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                    "text": "test"
                }
            }))
            .unwrap()
        ));
        assert!(!principal.accepts(
            &serde_json::from_value::<Update>(serde_json::json!({
                "update_id": 1,
                "message": {
                    "message_id": 1,
                    "date": 1,
                    "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                    "chat": {"id": 1, "type": "supergroup", "title": "test"},
                    "text": "test"
                }
            }))
            .unwrap()
        ));
    }
}
