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
    fn before(&mut self, _api: &Api, _update: &Update) -> MiddlewareFuture {
        MiddlewareResult::Continue.into()
    }

    /// Called after all handlers
    fn after(&mut self, _api: &Api, _update: &Update) -> MiddlewareFuture {
        MiddlewareResult::Continue.into()
    }
}

struct Rule {
    principal: Principal,
    is_granted: bool,
}

impl Rule {
    fn accepts(&self, update: &Update) -> bool {
        match self.principal {
            Principal::UserId(user_id) => update.get_user().map(|u| u.id == user_id),
            Principal::Username(ref username) => update.get_user().and_then(|u| {
                if let Some(ref x) = u.username {
                    Some(x == username)
                } else {
                    None
                }
            }),
            Principal::ChatId(chat_id) => update.get_chat_id().map(|x| x == chat_id),
            Principal::ChatUsername(ref chat_username) => {
                update.get_chat_username().map(|x| x == chat_username)
            }
            Principal::All => return true,
        }
        .unwrap_or(false)
    }
}

#[derive(Debug)]
enum Principal {
    All,
    UserId(Integer),
    Username(String),
    ChatId(Integer),
    ChatUsername(String),
}

/// Access control middleware
///
/// Helps to deny/allow updates from specific user/chat
///
/// If there are no rules matching an update, access will be forbidden.
#[derive(Default)]
pub struct AccessMiddleware {
    rules: Vec<Rule>,
}

impl AccessMiddleware {
    /// Allows all updates
    pub fn allow_all(mut self) -> Self {
        self.rules.push(Rule {
            is_granted: true,
            principal: Principal::All,
        });
        self
    }

    /// Denies all updates
    pub fn deny_all(mut self) -> Self {
        self.rules.push(Rule {
            is_granted: false,
            principal: Principal::All,
        });
        self
    }

    /// Allows updates from a user with ID
    pub fn allow_user_id(mut self, user_id: Integer) -> Self {
        self.rules.push(Rule {
            is_granted: true,
            principal: Principal::UserId(user_id),
        });
        self
    }

    /// Denies updates from a user with ID
    pub fn deny_user_id(mut self, user_id: Integer) -> Self {
        self.rules.push(Rule {
            is_granted: false,
            principal: Principal::UserId(user_id),
        });
        self
    }

    /// Allows updates from a user with @username
    pub fn allow_username<S: Into<String>>(mut self, username: S) -> Self {
        self.rules.push(Rule {
            is_granted: true,
            principal: Principal::Username(username.into()),
        });
        self
    }

    /// Denies updates from a user with @username
    pub fn deny_username<S: Into<String>>(mut self, username: S) -> Self {
        self.rules.push(Rule {
            is_granted: false,
            principal: Principal::Username(username.into()),
        });
        self
    }

    /// Allows updates from a chat with ID
    pub fn allow_chat_id(mut self, chat_id: Integer) -> Self {
        self.rules.push(Rule {
            is_granted: true,
            principal: Principal::ChatId(chat_id),
        });
        self
    }

    /// Denies updates from a chat with ID
    pub fn deny_chat_id(mut self, chat_id: Integer) -> Self {
        self.rules.push(Rule {
            is_granted: false,
            principal: Principal::ChatId(chat_id),
        });
        self
    }

    /// Allows updates from a chat with @username
    pub fn allow_chat_username<S: Into<String>>(mut self, username: S) -> Self {
        self.rules.push(Rule {
            is_granted: true,
            principal: Principal::ChatUsername(username.into()),
        });
        self
    }

    /// Denies updates from a chat with @username
    pub fn deny_chat_username<S: Into<String>>(mut self, username: S) -> Self {
        self.rules.push(Rule {
            is_granted: false,
            principal: Principal::ChatUsername(username.into()),
        });
        self
    }
}

impl Middleware for AccessMiddleware {
    fn before(&mut self, _api: &Api, update: &Update) -> MiddlewareFuture {
        for rule in &self.rules {
            if rule.accepts(&update) {
                return if rule.is_granted {
                    MiddlewareResult::Continue
                } else {
                    log::info!("Access denied for principal: {:?}", rule.principal);
                    MiddlewareResult::Stop
                }
                .into();
            }
        }
        log::info!("Access denied by default, no rules found");
        MiddlewareResult::Stop.into()
    }
}

/// Limits number of updates per time
pub struct RateLimitMiddleware {
    rate_limiter: RateLimiter,
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
            rate_limiter: RateLimiter::Direct(DirectRateLimiter::new(
                capacity,
                Duration::from_secs(seconds),
            )),
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
            rate_limiter: RateLimiter::Keyed {
                limiter: Arc::new(Mutex::new(KeyedRateLimiter::new(
                    capacity,
                    Duration::from_secs(seconds),
                ))),
                on_missing,
                key,
            },
        }
    }
}

impl Middleware for RateLimitMiddleware {
    fn before(&mut self, _api: &Api, update: &Update) -> MiddlewareFuture {
        let should_pass = match self.rate_limiter {
            RateLimiter::Direct(ref mut limiter) => limiter.check().is_ok(),
            RateLimiter::Keyed {
                ref limiter,
                key,
                on_missing,
            } => {
                let mut limiter = limiter.lock().unwrap();
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
        limiter: Arc<Mutex<KeyedRateLimiter<Integer, GCRA>>>,
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
