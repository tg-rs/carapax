use crate::types::Update;
use async_trait::async_trait;
use std::fmt::Debug;

/// An update handler
#[async_trait]
pub trait UpdateHandler {
    /// Error produced by handler
    type Error: Debug + Send + Sync;

    /// Handles an update
    ///
    /// # Arguments
    ///
    /// * update - A received update
    async fn handle(&mut self, update: Update) -> Result<(), Self::Error>;
}
