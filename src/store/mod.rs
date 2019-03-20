use failure::Error;
use futures::Future;
use serde::{de::DeserializeOwned, Serialize};

/// Redis session store
///
/// Available with enabled "redis-store" feature
#[cfg(feature = "redis-store")]
pub mod redis;

mod session;
pub use self::session::Session;

/// Methods for accessing a store
pub trait SessionStore {
    /// Get value of key
    ///
    /// If key not exists, None is returned
    fn get<O>(&self, key: &str) -> Box<Future<Item = Option<O>, Error = Error> + Send>
    where
        O: DeserializeOwned + Send + 'static;

    /// Set key to hold the string value
    fn set<I>(&self, key: &str, val: &I) -> Box<Future<Item = (), Error = Error> + Send>
    where
        I: Serialize;

    /// Set a timeout on key
    ///
    /// After the timeout has expired, the key will automatically be deleted
    fn expire(&self, key: &str, seconds: usize) -> Box<Future<Item = (), Error = Error> + Send>;

    /// Remove the specified key
    fn del(&self, key: &str) -> Box<Future<Item = (), Error = Error> + Send>;
}
