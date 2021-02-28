use crate::core::{Handler, HandlerResult};
use futures_util::future::BoxFuture;
use ratelimit_meter::{KeyedRateLimiter, GCRA};
use std::{hash::Hash, num::NonZeroU32, sync::Arc, time::Duration};
use tgbot::types::{ChatId, Integer, Update, UserId};
use tokio::sync::Mutex;

/// Limits updates accroding to given rules
#[derive(Clone)]
pub struct KeyedRateLimitHandler<K: RateLimitKey> {
    limiter: Arc<Mutex<KeyedRateLimiter<K::Key, GCRA>>>,
    key: K,
    on_missing: bool,
}

impl<K> KeyedRateLimitHandler<K>
where
    K: RateLimitKey,
{
    /// Creates a new handler
    ///
    /// # Arguments
    ///
    /// - key - A key to filter updates
    /// - capacity - Number of updates
    /// - duration - Per time unit
    /// - on_missing - Allow or deny update when key is missing
    pub fn new(key: K, on_missing: bool, capacity: NonZeroU32, duration: Duration) -> Self {
        Self {
            limiter: Arc::new(Mutex::new(KeyedRateLimiter::new(capacity, duration))),
            key,
            on_missing,
        }
    }
}

/// A key to filter updates
pub trait RateLimitKey {
    /// Type of the key
    type Key: Clone + Eq + Hash + Send;

    /// Returns a key from given update
    ///
    /// Considered missing when None is returned
    fn get_key(&self, update: &Update) -> Option<Self::Key>;
}

impl<F, K> RateLimitKey for F
where
    F: Fn(&Update) -> Option<K>,
    K: Clone + Eq + Hash + Send,
{
    type Key = K;

    fn get_key(&self, update: &Update) -> Option<Self::Key> {
        (self)(update)
    }
}

/// Limit updates per user ID for all users
pub fn limit_all_users(update: &Update) -> Option<Integer> {
    update.get_user().map(|u| u.id)
}

/// Limit updates per chat ID for all chats
pub fn limit_all_chats(update: &Update) -> Option<Integer> {
    update.get_chat_id()
}

/// Limit updates per user or chat for given IDs
///
/// Key considered missing for every update that not found in list
#[derive(Clone, Debug, Default)]
pub struct RateLimitList {
    users: Vec<UserId>,
    chats: Vec<ChatId>,
}

impl RateLimitList {
    /// User IDs to limit
    pub fn with_users(mut self, users: Vec<UserId>) -> Self {
        self.users = users;
        self
    }

    /// Chat IDs to limit
    pub fn with_chats(mut self, chats: Vec<ChatId>) -> Self {
        self.chats = chats;
        self
    }

    /// User ID to limit
    pub fn with_user<I>(mut self, user_id: I) -> Self
    where
        I: Into<UserId>,
    {
        self.users.push(user_id.into());
        self
    }

    /// Chat ID to limit
    pub fn with_chat<I>(mut self, chat_id: I) -> Self
    where
        I: Into<ChatId>,
    {
        self.chats.push(chat_id.into());
        self
    }

    fn has_chat_id(&self, chat_id: Integer) -> bool {
        self.chats
            .iter()
            .any(|i| if let ChatId::Id(i) = i { *i == chat_id } else { false })
    }

    fn has_chat_username(&self, username: &str) -> bool {
        self.chats.iter().any(|i| {
            if let ChatId::Username(i) = i {
                i == username
            } else {
                false
            }
        })
    }

    fn has_user_id(&self, user_id: Integer) -> bool {
        self.users
            .iter()
            .any(|i| if let UserId::Id(i) = i { *i == user_id } else { false })
    }

    fn has_username(&self, username: &str) -> bool {
        self.users.iter().any(|i| {
            if let UserId::Username(i) = i {
                i == username
            } else {
                false
            }
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[doc(hidden)]
pub enum RateLimitListKey {
    Id(Integer),
    Username(String),
}

impl RateLimitKey for RateLimitList {
    type Key = RateLimitListKey;

    fn get_key(&self, update: &Update) -> Option<Self::Key> {
        if let Some(chat_id) = update.get_chat_id() {
            if self.has_chat_id(chat_id) {
                return Some(RateLimitListKey::Id(chat_id));
            }
        }
        if let Some(username) = update.get_chat_username() {
            if self.has_chat_username(username) {
                return Some(RateLimitListKey::Username(String::from(username)));
            }
        }
        if let Some(user) = update.get_user() {
            if self.has_user_id(user.id) {
                return Some(RateLimitListKey::Id(user.id));
            }
            if let Some(ref username) = user.username {
                if self.has_username(username) {
                    return Some(RateLimitListKey::Username(username.clone()));
                }
            }
        }
        None
    }
}

impl<K> Handler<Update, BoxFuture<'static, HandlerResult>> for KeyedRateLimitHandler<K>
where
    K: RateLimitKey + Send + Sync,
    K::Key: 'static,
{
    fn call(&self, update: Update) -> BoxFuture<'static, HandlerResult> {
        enum Ret<K: Clone + Eq + Hash> {
            OnMissing(bool),
            Check {
                limiter: Arc<Mutex<KeyedRateLimiter<K, GCRA>>>,
                key: K,
            },
        }

        let ret = if let Some(key) = self.key.get_key(&update) {
            Ret::Check {
                limiter: self.limiter.clone(),
                key,
            }
        } else {
            Ret::OnMissing(self.on_missing)
        };

        Box::pin(async move {
            let should_pass = match ret {
                Ret::Check { limiter, key } => {
                    let mut limiter = limiter.lock().await;
                    limiter.check(key).is_ok()
                }
                Ret::OnMissing(on_missing) => on_missing,
            };

            if should_pass {
                HandlerResult::Continue
            } else {
                HandlerResult::Stop
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nonzero_ext::nonzero;

    #[tokio::test]
    async fn handler_key_found() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 1,
                "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                "text": "test"
            }
        }))
        .unwrap();
        let mut results = Vec::new();
        let handler = KeyedRateLimitHandler::new(limit_all_users, true, nonzero!(1u32), Duration::from_secs(1000));
        for _ in 0..10 {
            results.push(handler.call(update.clone()).await);
        }
        assert!(results.into_iter().any(|x| matches!(x, HandlerResult::Stop)))
    }

    #[tokio::test]
    async fn handler_key_not_found() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 1,
                "from": {"id": 2, "is_bot": false, "first_name": "test", "username": "username_user"},
                "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                "text": "test"
            }
        }))
        .unwrap();
        for on_missing in &[true, false] {
            let handler = KeyedRateLimitHandler::new(
                RateLimitList::default().with_user(1),
                *on_missing,
                nonzero!(1u32),
                Duration::from_secs(1000),
            );
            let result = handler.call(update.clone()).await;
            match result {
                HandlerResult::Continue => assert_eq!(*on_missing, true),
                HandlerResult::Stop => assert_eq!(*on_missing, false),
                result => panic!("unexpected result: {:?}", result),
            };
        }
    }

    #[test]
    fn limit_users() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 1,
                "from": {"id": 2, "is_bot": false, "first_name": "test", "username": "username_user"},
                "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                "text": "test"
            }
        }))
        .unwrap();
        assert_eq!(
            limit_all_users(&update).unwrap(),
            update.get_user().map(|u| u.id).unwrap()
        );
    }

    #[test]
    fn limit_chats() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 1,
                "from": {"id": 2, "is_bot": false, "first_name": "test", "username": "username_user"},
                "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                "text": "test"
            }
        }))
        .unwrap();
        assert_eq!(limit_all_chats(&update).unwrap(), update.get_chat_id().unwrap());
    }

    #[test]
    fn rate_limit_list() {
        let list = RateLimitList::default()
            .with_users(vec![UserId::from(1), UserId::from("username1")])
            .with_chats(vec![ChatId::from(1), ChatId::from("username1")])
            .with_user(2)
            .with_chat(2)
            .with_user("username2")
            .with_chat("username2");

        for (update, key) in vec![
            // chat id = 1
            (
                serde_json::json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 2,
                        "date": 3,
                        "from": {"id": 4, "is_bot": false, "first_name": "test", "username": "username"},
                        "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username"},
                        "text": "test"
                    }
                }),
                Some(RateLimitListKey::Id(1)),
            ),
            // chat username = username1
            (
                serde_json::json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 2,
                        "date": 3,
                        "from": {"id": 4, "is_bot": false, "first_name": "test", "username": "username"},
                        "chat": {"id": 5, "type": "supergroup", "title": "test", "username": "username1"},
                        "text": "test"
                    }
                }),
                Some(RateLimitListKey::Username(String::from("username1"))),
            ),
            // user id = 1
            (
                serde_json::json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 2,
                        "date": 3,
                        "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username"},
                        "chat": {"id": 5, "type": "supergroup", "title": "test", "username": "username"},
                        "text": "test"
                    }
                }),
                Some(RateLimitListKey::Id(1)),
            ),
            // user username = username1
            (
                serde_json::json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 2,
                        "date": 3,
                        "from": {"id": 4, "is_bot": false, "first_name": "test", "username": "username1"},
                        "chat": {"id": 5, "type": "supergroup", "title": "test", "username": "username"},
                        "text": "test"
                    }
                }),
                Some(RateLimitListKey::Username(String::from("username1"))),
            ),
            // key not found
            (
                serde_json::json!({
                    "update_id": 1,
                    "message": {
                        "message_id": 2,
                        "date": 3,
                        "from": {"id": 4, "is_bot": false, "first_name": "test", "username": "username"},
                        "chat": {"id": 5, "type": "supergroup", "title": "test", "username": "username"},
                        "text": "test"
                    }
                }),
                None,
            ),
        ] {
            let update: Update = serde_json::from_value(update).unwrap();
            assert_eq!(list.get_key(&update), key);
        }
    }
}
