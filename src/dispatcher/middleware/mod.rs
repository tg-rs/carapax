use crate::api::Api;
use crate::types::Update;
use failure::Error;
use futures::{future, Async, Future, Poll};

mod access;
mod ratelimit;

pub use self::access::*;
pub use self::ratelimit::*;

/// Result of a middleware
#[derive(Copy, Clone, Debug)]
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

#[must_use = "futures do nothing unless polled"]
pub(super) struct IterMiddlewareFuture {
    items: Vec<MiddlewareFuture>,
    current: usize,
}

impl IterMiddlewareFuture {
    pub(super) fn new(items: Vec<MiddlewareFuture>) -> IterMiddlewareFuture {
        IterMiddlewareFuture { items, current: 0 }
    }
}

impl Future for IterMiddlewareFuture {
    type Item = (MiddlewareResult, usize);
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let items_len = self.items.len();
        if items_len == 0 {
            return Ok(Async::Ready((MiddlewareResult::Continue, 0)));
        }
        if self.current >= items_len {
            return Ok(Async::Ready((MiddlewareResult::Continue, self.current)));
        }
        let f = &mut self.items[self.current];
        match f.poll() {
            Ok(Async::Ready(MiddlewareResult::Continue)) => {
                self.current += 1;
                Ok(Async::NotReady)
            }
            Ok(Async::Ready(MiddlewareResult::Stop)) => {
                Ok(Async::Ready((MiddlewareResult::Stop, self.current + 1)))
            }
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(err) => Err(err),
        }
    }
}

/// Middleware handler
pub trait Middleware {
    /// Called before all handlers
    fn before(&mut self, _api: &Api, _update: &Update) -> MiddlewareFuture {
        MiddlewareResult::Continue.into()
    }

    /// Called after all handlers
    fn after(&mut self, _api: &Api, _update: &Update) -> MiddlewareFuture {
        MiddlewareResult::Continue.into()
    }
}
