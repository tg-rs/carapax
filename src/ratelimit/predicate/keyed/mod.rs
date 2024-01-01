use std::{collections::HashSet, sync::Arc};

use governor::{clock::DefaultClock, state::keyed::DefaultKeyedStateStore, RateLimiter};
pub use governor::{Jitter, Quota};
#[allow(unused_imports)]
pub use nonzero_ext::nonzero;

use crate::{
    core::{Handler, PredicateResult},
    ratelimit::{
        jitter::NoJitter,
        key::Key,
        method::{MethodDiscard, MethodWait},
    },
};

#[cfg(test)]
mod tests;

/// A predicate with keyed rate limiter.
///
/// Each update will have it's own rate limit under key `K`.
#[derive(Clone)]
pub struct KeyedRateLimitPredicate<K, J, M>
where
    K: Key,
{
    limiter: Arc<RateLimiter<K, DefaultKeyedStateStore<K>, DefaultClock>>,
    jitter: J,
    _method: M,
    keys: HashSet<K>,
}

impl<K, J, M> KeyedRateLimitPredicate<K, J, M>
where
    K: Key,
{
    fn new(quota: Quota, jitter: J, method: M) -> Self {
        Self {
            limiter: Arc::new(RateLimiter::dashmap(quota)),
            jitter,
            _method: method,
            keys: Default::default(),
        }
    }

    /// Use this method when you need to run a predicate only for a specific key.
    ///
    /// If this method is not called, predicate will run for all updates.
    ///
    /// # Arguments
    ///
    /// * `key` - A key to filter by.
    pub fn with_key<T: Into<K>>(mut self, key: T) -> Self {
        self.keys.insert(key.into());
        self
    }

    fn has_key(&self, key: &K) -> bool {
        if self.keys.is_empty() {
            true
        } else {
            self.keys.contains(key)
        }
    }
}

impl<K> KeyedRateLimitPredicate<K, NoJitter, MethodDiscard>
where
    K: Key,
{
    /// Creates a new `KeyedRateLimitPredicate` with the discard method.
    ///
    /// Predicate will stop update propagation when the rate limit is reached.
    ///
    /// # Arguments
    ///
    /// * `quota` - A rate limiting quota.
    pub fn discard(quota: Quota) -> Self {
        Self::new(quota, NoJitter, MethodDiscard)
    }
}

impl<K> KeyedRateLimitPredicate<K, NoJitter, MethodWait>
where
    K: Key,
{
    /// Creates a new `KeyedRateLimitPredicate` with wait method.
    ///
    /// Predicate will pause update propagation when the rate limit is reached.
    ///
    /// # Arguments
    ///
    /// * `quota` - A rate limiting quota.
    pub fn wait(quota: Quota) -> Self {
        Self::new(quota, NoJitter, MethodWait)
    }
}

impl<K> KeyedRateLimitPredicate<K, Jitter, MethodWait>
where
    K: Key,
{
    /// Creates a new `KeyedRateLimitPredicate` with wait method and jitter.
    ///
    /// Predicate will pause update propagation when the rate limit is reached.
    ///
    /// # Arguments
    ///
    /// * `quota` - A rate limiting quota.
    /// * `jitter` - An interval specification for deviating from the nominal wait time.
    pub fn wait_with_jitter(quota: Quota, jitter: Jitter) -> Self {
        Self::new(quota, jitter, MethodWait)
    }
}

impl<K> Handler<K> for KeyedRateLimitPredicate<K, NoJitter, MethodDiscard>
where
    K: Key + Sync,
{
    type Output = PredicateResult;

    async fn handle(&self, input: K) -> Self::Output {
        if self.has_key(&input) {
            match self.limiter.check_key(&input) {
                Ok(_) => PredicateResult::True,
                Err(_) => {
                    log::info!("KeyedRateLimitPredicate: update discarded");
                    PredicateResult::False
                }
            }
        } else {
            PredicateResult::True
        }
    }
}

impl<K> Handler<K> for KeyedRateLimitPredicate<K, NoJitter, MethodWait>
where
    K: Key + Sync + 'static,
{
    type Output = PredicateResult;

    async fn handle(&self, input: K) -> Self::Output {
        if self.has_key(&input) {
            self.limiter.until_key_ready(&input).await;
            PredicateResult::True
        } else {
            PredicateResult::True
        }
    }
}

impl<K> Handler<K> for KeyedRateLimitPredicate<K, Jitter, MethodWait>
where
    K: Key + Sync + 'static,
{
    type Output = PredicateResult;

    async fn handle(&self, input: K) -> Self::Output {
        if self.has_key(&input) {
            self.limiter.until_key_ready_with_jitter(&input, self.jitter).await;
            PredicateResult::True
        } else {
            PredicateResult::True
        }
    }
}
