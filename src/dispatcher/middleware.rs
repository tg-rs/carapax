use crate::api::Api;
use crate::types::{Integer, Update};
use failure::Error;
use futures::{future, Async, Future, Poll};
use ratelimit_meter::{DirectRateLimiter, KeyedRateLimiter, GCRA};
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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
    fn before(&self, _api: &Api, _update: &Update) -> MiddlewareFuture {
        MiddlewareResult::Continue.into()
    }

    /// Called after all handlers
    fn after(&self, _api: &Api, _update: &Update) -> MiddlewareFuture {
        MiddlewareResult::Continue.into()
    }
}

/// Limits number of updates per time
pub struct RateLimitMiddleware {
    rate_limiter: Arc<Mutex<RateLimiter>>,
}

impl RateLimitMiddleware {
    /// Limit all updates
    ///
    /// # Arguments
    ///
    /// - capacity - Number of updates
    /// - seconds - Duration in seconds
    pub fn direct(capacity: NonZeroU32, seconds: u64) -> Self {
        RateLimitMiddleware {
            rate_limiter: Arc::new(Mutex::new(RateLimiter::Direct(DirectRateLimiter::new(
                capacity,
                Duration::from_secs(seconds),
            )))),
        }
    }

    /// Limit updates for each user or chat
    ///
    /// # Arguments
    ///
    /// - key - User or Chat
    /// - capacity - Number of updates
    /// - seconds - Duration in seconds
    /// - on_missing - Allow or deny update when user or chat not found
    ///                (got an update from channel or inline query, etc...)
    pub fn keyed(key: RateLimitKey, capacity: NonZeroU32, seconds: u64, on_missing: bool) -> Self {
        RateLimitMiddleware {
            rate_limiter: Arc::new(Mutex::new(RateLimiter::Keyed {
                limiter: KeyedRateLimiter::new(capacity, Duration::from_secs(seconds)),
                on_missing,
                key,
            })),
        }
    }
}

impl Middleware for RateLimitMiddleware {
    fn before(&self, _api: &Api, update: &Update) -> MiddlewareFuture {
        let should_pass = match *self.rate_limiter.lock().unwrap() {
            RateLimiter::Direct(ref mut limiter) => limiter.check().is_ok(),
            RateLimiter::Keyed {
                ref mut limiter,
                key,
                on_missing,
            } => {
                let val = match key {
                    RateLimitKey::Chat => update.get_chat_id(),
                    RateLimitKey::User => update.get_user().map(|u| u.id),
                };
                if let Some(val) = val {
                    limiter.check(val).is_ok()
                } else {
                    on_missing
                }
            }
        };
        if should_pass {
            MiddlewareResult::Continue
        } else {
            MiddlewareResult::Stop
        }
        .into()
    }
}

enum RateLimiter {
    Direct(DirectRateLimiter<GCRA>),
    Keyed {
        limiter: KeyedRateLimiter<Integer, GCRA>,
        key: RateLimitKey,
        on_missing: bool,
    },
}

/// Rate limit key
#[derive(Copy, Clone, Debug)]
pub enum RateLimitKey {
    /// Limit per chat
    Chat,
    /// Limit per user
    User,
}
