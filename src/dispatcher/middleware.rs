use crate::context::Context;
use failure::Error;
use futures::{future, Future, Poll};
use tgbot::types::Update;

/// Result of a middleware
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MiddlewareResult {
    /// Continue propagation
    ///
    /// Next middleware and all handlers (if exists) will run after current has finished
    Continue,
    /// Stop propagation
    ///
    /// Next middleware and all handlers (if exists) will not run after current has finished
    Stop,
}

/// A middleware future
#[must_use = "futures do nothing unless polled"]
pub struct MiddlewareFuture {
    inner: Box<Future<Item = MiddlewareResult, Error = Error> + Send>,
}

impl MiddlewareFuture {
    /// Creates a new middleware future
    pub fn new<F>(f: F) -> MiddlewareFuture
    where
        F: Future<Item = MiddlewareResult, Error = Error> + 'static + Send,
    {
        MiddlewareFuture { inner: Box::new(f) }
    }
}

impl<E> From<Result<MiddlewareResult, E>> for MiddlewareFuture
where
    E: Into<Error>,
{
    fn from(result: Result<MiddlewareResult, E>) -> Self {
        MiddlewareFuture::new(future::result(result.map_err(Into::into)))
    }
}

impl From<MiddlewareResult> for MiddlewareFuture {
    fn from(result: MiddlewareResult) -> MiddlewareFuture {
        MiddlewareFuture::new(future::ok(result))
    }
}

impl Future for MiddlewareFuture {
    type Item = MiddlewareResult;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

/// Middleware handler
pub trait Middleware {
    /// Called before all handlers
    fn before(&mut self, _context: &Context, _update: &Update) -> MiddlewareFuture {
        MiddlewareResult::Continue.into()
    }

    /// Called after all handlers
    fn after(&mut self, _context: &Context, _update: &Update) -> MiddlewareFuture {
        MiddlewareResult::Continue.into()
    }
}
