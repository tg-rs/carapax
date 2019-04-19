use carapax::prelude::*;
use ratelimit_meter::{KeyedRateLimiter, GCRA};
use std::{
    hash::Hash,
    num::NonZeroU32,
    sync::{Arc, Mutex},
    time::Duration,
};

/// Limits updates accroding to given rules
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
    type Key: Clone + Eq + Hash;

    /// Returns a key from given update
    ///
    /// Considered missing when None is returned
    fn get_key(&self, update: &Update) -> Option<Self::Key>;
}

impl<F, K> RateLimitKey for F
where
    F: Fn(&Update) -> Option<K>,
    K: Clone + Eq + Hash,
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

impl<K> UpdateHandler for KeyedRateLimitHandler<K>
where
    K: RateLimitKey,
{
    fn handle(&self, _context: &mut Context, update: &Update) -> HandlerFuture {
        let should_pass = if let Some(key) = self.key.get_key(update) {
            self.limiter.lock().unwrap().check(key).is_ok()
        } else {
            self.on_missing
        };
        if should_pass {
            HandlerResult::Continue
        } else {
            HandlerResult::Stop
        }
        .into()
    }
}
