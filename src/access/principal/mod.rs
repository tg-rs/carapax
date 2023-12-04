use serde::{Deserialize, Serialize};

use crate::types::{ChatId, Update, UserId};

#[cfg(test)]
mod tests;

/// Represents a principal entity that decides whether
/// an [`crate::access::AccessRule`] should accept an update.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Principal {
    /// Accepts all updates without any specific conditions.
    All,
    /// Accepts updates only from a specified user.
    User(UserId),
    /// Accepts updates only from a specified chat.
    Chat(ChatId),
    /// Accepts updates only from a user within a specific chat.
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
    /// Creates a principal for a specific user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - ID of the user.
    pub fn user<T>(user_id: T) -> Self
    where
        T: Into<UserId>,
    {
        Principal::User(user_id.into())
    }

    /// Creates a principal for a specific chat.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - ID of the chat.
    pub fn chat<T>(chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        Principal::Chat(chat_id.into())
    }

    /// Creates a principal for a user within a specific chat.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - ID of the chat.
    /// * `user_id` - ID of the user.
    pub fn chat_user<A, B>(chat_id: A, user_id: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<UserId>,
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
