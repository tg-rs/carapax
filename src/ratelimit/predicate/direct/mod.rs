use std::sync::Arc;

use governor::{
    clock::DefaultClock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
    RateLimiter,
};
pub use governor::{Jitter, Quota};
pub use nonzero_ext::nonzero;

use crate::{
    core::{Handler, PredicateResult},
    ratelimit::{
        jitter::NoJitter,
        method::{MethodDiscard, MethodWait},
    },
};

#[cfg(test)]
mod tests;

/// A predicate with a direct rate limiter.
///
/// Use this predicate when you need to limit all updates.
#[derive(Clone)]
pub struct DirectRateLimitPredicate<J, M> {
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>>,
    jitter: J,
    _method: M,
}

impl DirectRateLimitPredicate<NoJitter, MethodDiscard> {
    /// Creates a new `DirectRateLimitPredicate` with the discard method.
    ///
    /// The predicate will stop update propagation when the rate limit is reached.
    ///
    /// # Arguments
    ///
    /// * `quota` - A rate limiting quota.
    pub fn discard(quota: Quota) -> Self {
        Self {
            limiter: Arc::new(RateLimiter::direct(quota)),
            jitter: NoJitter,
            _method: MethodDiscard,
        }
    }
}

impl DirectRateLimitPredicate<NoJitter, MethodWait> {
    /// Creates a new `DirectRateLimitPredicate` with the wait method.
    ///
    /// The predicate will pause update propagation when the rate limit is reached.
    ///
    /// # Arguments
    ///
    /// * `quota` - A rate limiting quota.
    pub fn wait(quota: Quota) -> Self {
        Self {
            limiter: Arc::new(RateLimiter::direct(quota)),
            jitter: NoJitter,
            _method: MethodWait,
        }
    }
}

impl DirectRateLimitPredicate<Jitter, MethodWait> {
    /// Creates a new `DirectRateLimitPredicate` with the wait method and jitter.
    ///
    /// Predicate will pause update propagation when the rate limit is reached.
    ///
    /// # Arguments
    ///
    /// * `quota` - A rate limiting quota.
    /// * `jitter` - An interval specification for deviating from the nominal wait time.
    pub fn wait_with_jitter(quota: Quota, jitter: Jitter) -> Self {
        Self {
            limiter: Arc::new(RateLimiter::direct(quota)),
            jitter,
            _method: MethodWait,
        }
    }
}

impl Handler<()> for DirectRateLimitPredicate<NoJitter, MethodDiscard> {
    type Output = PredicateResult;

    async fn handle(&self, (): ()) -> Self::Output {
        match self.limiter.check() {
            Ok(_) => PredicateResult::True,
            Err(_) => {
                log::info!("DirectRateLimitPredicate: update discarded");
                PredicateResult::False
            }
        }
    }
}

impl Handler<()> for DirectRateLimitPredicate<NoJitter, MethodWait> {
    type Output = PredicateResult;

    async fn handle(&self, (): ()) -> Self::Output {
        self.limiter.until_ready().await;
        PredicateResult::True
    }
}

impl Handler<()> for DirectRateLimitPredicate<Jitter, MethodWait> {
    type Output = PredicateResult;

    async fn handle(&self, (): ()) -> Self::Output {
        self.limiter.until_ready_with_jitter(self.jitter).await;
        PredicateResult::True
    }
}
