use crate::core::{convert::TryFromUpdate, result::HandlerResult};
use async_trait::async_trait;
use tgbot::types::Update;

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

pub(crate) struct ConvertHandler<H>(H);

impl<H> ConvertHandler<H> {
    pub(crate) fn boxed(handler: H) -> Box<Self> {
        Box::new(Self(handler))
    }
}

#[async_trait]
impl<C, H, I> Handler<C> for ConvertHandler<H>
where
    C: Send + Sync,
    H: Handler<C, Input = I> + Send,
    I: TryFromUpdate + Send + Sync + 'static,
{
    type Input = Update;
    type Output = HandlerResult;

    async fn handle(&mut self, context: &C, input: Self::Input) -> Self::Output {
        match TryFromUpdate::try_from_update(input) {
            Ok(Some(input)) => self.0.handle(context, input).await.into(),
            Ok(None) => HandlerResult::Continue,
            Err(err) => HandlerResult::error(err),
        }
    }
}
