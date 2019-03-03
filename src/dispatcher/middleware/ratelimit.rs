use super::{Middleware, MiddlewareFuture, MiddlewareResult};
use crate::api::Api;
use crate::types::{Integer, Update};
use ratelimit_meter::{DirectRateLimiter, KeyedRateLimiter, GCRA};
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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
