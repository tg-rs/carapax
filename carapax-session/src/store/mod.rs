use crate::session::SessionKey;
use carapax::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;

#[cfg(any(feature = "fs-store", feature = "redis-store"))]
mod data;

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
#[async_trait]
pub trait SessionStore {
    /// An error occurred when accessing store
    type Error: Error + Send + Sync;

    /// Get value of key
    ///
    /// If key not exists, None is returned
    async fn get<O>(&mut self, key: SessionKey) -> Result<Option<O>, Self::Error>
    where
        O: DeserializeOwned + Send + Sync;

    /// Set key to hold the given value
    async fn set<I>(&mut self, key: SessionKey, val: &I) -> Result<(), Self::Error>
    where
        I: Serialize + Send + Sync;

    /// Set a timeout on key
    ///
    /// After the timeout has expired, the key will automatically be deleted
    async fn expire(&mut self, key: SessionKey, seconds: usize) -> Result<(), Self::Error>;

    /// Remove the specified key
    async fn del(&mut self, key: SessionKey) -> Result<(), Self::Error>;
}
