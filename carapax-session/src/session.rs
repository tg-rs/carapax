use crate::store::SessionStore;
use carapax::core::types::Update;
use failure::Error;
use futures::Future;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt, sync::Arc, time::Duration};

/// Actual session available in context
#[derive(Clone)]
pub struct Session<S> {
    namespace: String,
    store: Arc<S>,
}

impl<S> Session<S>
where
    S: SessionStore,
{
    pub(crate) fn new<N: Into<String>>(namespace: N, store: Arc<S>) -> Self {
        Self {
            namespace: namespace.into(),
            store,
        }
    }

    fn build_key(&self, key: &str) -> SessionKey {
        SessionKey::new(self.namespace.clone(), key)
    }

    /// Get value of key
    ///
    /// If key not exists, None is returned
    pub fn get<O>(&self, key: &str) -> Box<Future<Item = Option<O>, Error = Error> + Send>
    where
        O: DeserializeOwned + Send + 'static,
    {
        self.store.get(self.build_key(key))
    }

    /// Set key to hold the given value
    pub fn set<I>(&self, key: &str, val: &I) -> Box<Future<Item = (), Error = Error> + Send>
    where
        I: Serialize,
    {
        self.store.set(self.build_key(key), val)
    }

    /// Set a timeout on key
    ///
    /// After the timeout has expired, the key will automatically be deleted
    pub fn expire(&self, key: &str, seconds: usize) -> Box<Future<Item = (), Error = Error> + Send> {
        self.store.expire(self.build_key(key), seconds)
    }

    /// Remove the specified key
    pub fn del(&self, key: &str) -> Box<Future<Item = (), Error = Error> + Send> {
        self.store.del(self.build_key(key))
    }
}

/// A session key used in store
#[derive(Debug, Clone)]
pub struct SessionKey {
    namespace: String,
    name: String,
}

impl SessionKey {
    fn new<A, B>(namespace: A, name: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            namespace: namespace.into(),
            name: name.into(),
        }
    }

    /// Namespace for a key
    ///
    /// Format: `(user-id|chat-id)-(user-id|chat-id)`
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Key name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for SessionKey {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}-{}", self.namespace, self.name)
    }
}

pub(crate) fn namespace_from_update(update: &Update) -> String {
    let (chat_id, user_id) = match (update.get_chat_id(), update.get_user().map(|x| x.id)) {
        (Some(chat_id), Some(user_id)) => (chat_id, user_id),
        (Some(chat_id), None) => (chat_id, chat_id),
        (None, Some(user_id)) => (user_id, user_id),
        (None, None) => unreachable!(), // There is always chat_id or user_id
    };
    format!("{}-{}", chat_id, user_id)
}

/// Defines a lifetime for each session
#[derive(Clone, Copy, Debug)]
pub enum SessionLifetime {
    /// Session will live forever
    ///
    /// Default variant
    Forever,
    /// Session will expire at given duration
    Duration(Duration),
}

impl Default for SessionLifetime {
    fn default() -> Self {
        SessionLifetime::Forever
    }
}

impl From<Duration> for SessionLifetime {
    fn from(duration: Duration) -> Self {
        SessionLifetime::Duration(duration)
    }
}

impl From<u64> for SessionLifetime {
    fn from(seconds: u64) -> Self {
        SessionLifetime::Duration(Duration::from_secs(seconds))
    }
}
