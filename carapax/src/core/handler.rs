use crate::core::{convert::TryFromUpdate, result::HandlerResult};
use async_trait::async_trait;

/// An update handler
#[async_trait]
pub trait Handler<C> {
    /// An object to handle (update, message, inline query, etc...)
    ///
    /// See [TryFromUpdate](trait.TryFromUpdate.html) for more information
    type Input: TryFromUpdate + Send + Sync;

    /// A result to return
    ///
    /// See [HandlerResult](enum.HandlerResult.html) for more information
    type Output: Into<HandlerResult>;

    /// Process an update
    ///
    /// # Arguments
    ///
    /// * context - A context which provides access to any type you have set before
    /// * input - An object obtained from update (update itself, message, etc...)
    async fn handle(&mut self, context: &C, input: Self::Input) -> Self::Output;
}
