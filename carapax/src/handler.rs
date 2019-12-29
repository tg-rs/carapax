use crate::convert::TryFromUpdate;
use async_trait::async_trait;
use std::{error::Error, fmt};
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
    async fn handle(&mut self, context: &mut C, input: Self::Input) -> Self::Output;
}

/// Result of a handler
#[derive(Debug)]
pub enum HandlerResult {
    /// Continue propagation
    ///
    /// Next handler (if exists) will run after current has finished
    Continue,
    /// Stop propagation
    ///
    /// Next handler (if exists) will not run after current has finished
    Stop,
    /// An error has occurred, stop propagation
    ///
    /// Next handler (if exists) will not run after current has finished
    Error(HandlerError),
}

/// An error returned by handler
#[derive(Debug)]
pub struct HandlerError(Box<dyn Error + Send + Sync>);

impl HandlerError {
    /// Creates a new error
    pub fn new<E>(err: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self(Box::new(err))
    }
}

impl Error for HandlerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.0.as_ref())
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}", self.0)
    }
}

impl From<()> for HandlerResult {
    fn from(_: ()) -> Self {
        HandlerResult::Continue
    }
}

impl<E> From<Result<(), E>> for HandlerResult
where
    E: Error + Send + Sync + 'static,
{
    fn from(result: Result<(), E>) -> Self {
        match result {
            Ok(()) => HandlerResult::Continue,
            Err(err) => HandlerResult::Error(HandlerError::new(err)),
        }
    }
}

impl From<HandlerError> for HandlerResult {
    fn from(err: HandlerError) -> Self {
        HandlerResult::Error(err)
    }
}

pub(crate) struct BoxedHandler<H>(H);

impl<H> BoxedHandler<H> {
    pub(crate) fn new(handler: H) -> Box<Self> {
        Box::new(Self(handler))
    }
}

#[async_trait]
impl<C, H, I> Handler<C> for BoxedHandler<H>
where
    C: Send,
    H: Handler<C, Input = I> + Send,
    I: TryFromUpdate + Send + Sync + 'static,
{
    type Input = Update;
    type Output = HandlerResult;

    async fn handle(&mut self, context: &mut C, input: Self::Input) -> Self::Output {
        match TryFromUpdate::try_from_update(input) {
            Ok(Some(input)) => self.0.handle(context, input).await.into(),
            Ok(None) => HandlerResult::Continue,
            Err(err) => HandlerResult::Error(HandlerError::new(err)),
        }
    }
}
