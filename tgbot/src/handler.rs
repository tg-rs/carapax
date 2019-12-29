use crate::types::Update;
use async_trait::async_trait;

/// An update handler
#[async_trait]
pub trait UpdateHandler {
    /// Handles an update
    ///
    /// # Arguments
    ///
    /// * update - A received update
    async fn handle(&mut self, update: Update);
}
