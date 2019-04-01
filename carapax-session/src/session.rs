use crate::store::SessionStore;
use carapax::core::types::Update;
use failure::Error;
use futures::Future;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

/// Actual session available in context
///
/// Each key passed to session will be prefixed with namespace obtained from an update
/// Format of the namespace is: `(user-id|chat-id)-(user-id|chat-id)`
#[derive(Clone)]
pub struct Session<S> {
    namespace: SessionKey,
    store: Arc<S>,
}

impl<S> Session<S>
where
    S: SessionStore,
{
    pub(crate) fn new(namespace: SessionKey, store: Arc<S>) -> Self {
        Self { namespace, store }
    }

    /// Get value of key
    ///
    /// If key not exists, None is returned
    pub fn get<O>(&self, key: &str) -> Box<Future<Item = Option<O>, Error = Error> + Send>
    where
        O: DeserializeOwned + Send + 'static,
    {
        self.store.get(self.namespace.with_part(key))
    }

    /// Set key to hold the given value
    pub fn set<I>(&self, key: &str, val: &I) -> Box<Future<Item = (), Error = Error> + Send>
    where
        I: Serialize,
    {
        self.store.set(self.namespace.with_part(key), val)
    }

    /// Set a timeout on key
    ///
    /// After the timeout has expired, the key will automatically be deleted
    pub fn expire(&self, key: &str, seconds: usize) -> Box<Future<Item = (), Error = Error> + Send> {
        self.store.expire(self.namespace.with_part(key), seconds)
    }

    /// Remove the specified key
    pub fn del(&self, key: &str) -> Box<Future<Item = (), Error = Error> + Send> {
        self.store.del(self.namespace.with_part(key))
    }
}

/// A session key used in store
#[derive(Clone)]
pub struct SessionKey(Vec<String>);

impl SessionKey {
    pub(crate) fn from_update(update: &Update) -> Self {
        Self(
            match (update.get_chat_id(), update.get_user().map(|x| x.id)) {
                (Some(chat_id), Some(user_id)) => [chat_id, user_id],
                (Some(chat_id), None) => [chat_id, chat_id],
                (None, Some(user_id)) => [user_id, user_id],
                (None, None) => unreachable!(), // There is always chat_id or user_id
            }
            .iter()
            .map(|x| x.to_string())
            .collect(),
        )
    }

    fn with_part<P: ToString>(&self, part: P) -> SessionKey {
        let mut parts = self.0.clone();
        parts.push(part.to_string());
        SessionKey(parts)
    }

    /// Converts key to a vector of it's parts
    pub fn into_inner(self) -> Vec<String> {
        self.0
    }
}

impl ToString for SessionKey {
    fn to_string(&self) -> String {
        self.0.join("-")
    }
}
