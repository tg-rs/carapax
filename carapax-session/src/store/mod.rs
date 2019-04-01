use crate::session::SessionKey;
use failure::Error;
use futures::Future;
use serde::{de::DeserializeOwned, Serialize};

/// File system session store
///
/// Available with enabled "fs-store" feature
#[cfg(feature = "fs-store")]
pub mod fs;

/// Redis session store
///
/// Available with enabled "redis-store" feature
#[cfg(feature = "redis-store")]
pub mod redis;

/// Methods for accessing a store
pub trait SessionStore {
    /// Get value of key
    ///
    /// If key not exists, None is returned
    fn get<O>(&self, key: SessionKey) -> Box<Future<Item = Option<O>, Error = Error> + Send>
    where
        O: DeserializeOwned + Send + 'static;

    /// Set key to hold the given value
    fn set<I>(&self, key: SessionKey, val: &I) -> Box<Future<Item = (), Error = Error> + Send>
    where
        I: Serialize;

    /// Set a timeout on key
    ///
    /// After the timeout has expired, the key will automatically be deleted
    fn expire(&self, key: SessionKey, seconds: usize) -> Box<Future<Item = (), Error = Error> + Send>;

    /// Remove the specified key
    fn del(&self, key: SessionKey) -> Box<Future<Item = (), Error = Error> + Send>;
}
