use std::{error::Error, fmt, future::Future, sync::Arc};

use crate::{
    core::{context::Context, convert::TryFromInput},
    types::Update,
};

#[cfg(test)]
mod tests;

/// Allows to handle a specific [`HandlerInput`].
pub trait Handler<I>: Clone + Send
where
    I: TryFromInput,
{
    /// A future output returned by [`Self::handle`] method.
    ///
    /// Use [`HandlerResult`] or any type that can be converted into it
    /// if you want to use the handler in [`crate::App`].
    ///
    /// It is possible to use any other type, e.g. if you want to use it in a decorator.
    /// But finally you need to convert it into [`HandlerResult`].
    type Output: Send;

    /// Handles a specific input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to handle.
    ///
    /// See [`TryFromInput`] trait implementations for a list of supported types.
    fn handle(&self, input: I) -> impl Future<Output = Self::Output> + Send;
}

macro_rules! impl_fn {
    ($($I:ident),+) => {
        #[allow(non_snake_case)]
        impl<X, $($I,)+ R> Handler<($($I,)+)> for X
        where
            X: Fn($($I,)+) -> R + Clone + Send + Sync,
            ($($I,)+): TryFromInput,
            R: Future + Send,
            R::Output: Send
        {
            type Output = R::Output;

            async fn handle(&self, ($($I,)+): ($($I,)+)) -> Self::Output {
                (self)($($I,)+).await
            }
        }
    };
}

impl_fn!(A);
impl_fn!(A, B);
impl_fn!(A, B, C);
impl_fn!(A, B, C, D);
impl_fn!(A, B, C, D, E);
impl_fn!(A, B, C, D, E, F);
impl_fn!(A, B, C, D, E, F, G);
impl_fn!(A, B, C, D, E, F, G, H);
impl_fn!(A, B, C, D, E, F, G, H, I);
impl_fn!(A, B, C, D, E, F, G, H, I, J);

/// An input for a [`Handler`] trait implementations.
#[derive(Clone, Debug)]
pub struct HandlerInput {
    /// An Update received from Telegram API.
    pub update: Update,
    /// A context with shared state.
    pub context: Arc<Context>,
}

impl From<Update> for HandlerInput {
    fn from(update: Update) -> Self {
        HandlerInput {
            update,
            context: Arc::new(Default::default()),
        }
    }
}

/// An error returned by a [`Handler`] trait implementation.
pub struct HandlerError(Box<dyn Error + Send>);

impl HandlerError {
    /// Creates a new `HandlerError`.
    ///
    /// # Arguments
    ///
    /// * `err` - The actual error.
    pub fn new<E>(err: E) -> Self
    where
        E: Error + Send + 'static,
    {
        Self(Box::new(err))
    }
}

impl fmt::Debug for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl Error for HandlerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}

/// A result returned by a [`Handler`] trait implementation.
pub type HandlerResult = Result<(), HandlerError>;

/// Converts objects into the [`HandlerResult`].
pub trait IntoHandlerResult {
    /// Returns the converted object.
    fn into_result(self) -> HandlerResult;
}

impl IntoHandlerResult for () {
    fn into_result(self) -> HandlerResult {
        Ok(self)
    }
}

impl<E> IntoHandlerResult for Result<(), E>
where
    E: Error + Send + 'static,
{
    fn into_result(self) -> HandlerResult {
        self.map_err(HandlerError::new)
    }
}
