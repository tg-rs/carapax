use crate::{
    core::{Handler, PredicateResult},
    ratelimit::{
        jitter::NoJitter,
        key::Key,
        method::{MethodDiscard, MethodWait},
    },
};
use futures_util::future::{ready, BoxFuture, Either, Ready};
use governor::{clock::DefaultClock, state::keyed::DefaultKeyedStateStore, RateLimiter};
use std::{collections::HashSet, sync::Arc};

pub use governor::{Jitter, Quota};
pub use nonzero_ext::nonzero;

/// A predicate with keyed ratelimiter
///
/// Each update will have it's own rate limit under key `K`
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

    /// Use this method when you need to run a predicate only for a specific key
    ///
    /// If this method is not called, predicate will run for all updates
    ///
    /// # Arguments
    ///
    /// * key - A key to filter by
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
    /// Creates a new predicate with discard method
    ///
    /// Predicate will stop update propagation when the rate limit is reached
    ///
    /// # Arguments
    ///
    /// * quota - A rate-limiting quota
    pub fn discard(quota: Quota) -> Self {
        Self::new(quota, NoJitter, MethodDiscard)
    }
}

impl<K> KeyedRateLimitPredicate<K, NoJitter, MethodWait>
where
    K: Key,
{
    /// Creates a new predicate with wait method
    ///
    /// Predicate will pause update propagation when the rate limit is reached
    ///
    /// # Arguments
    ///
    /// * quota - A rate-limiting quota
    pub fn wait(quota: Quota) -> Self {
        Self::new(quota, NoJitter, MethodWait)
    }
}

impl<K> KeyedRateLimitPredicate<K, Jitter, MethodWait>
where
    K: Key,
{
    /// Creates a new predicate with wait method and jitter
    ///
    /// Predicate will pause update propagation when the rate limit is reached
    ///
    /// # Arguments
    ///
    /// * quota - A rate-limiting quota
    /// * jitter - An interval specification for deviating from the nominal wait time
    pub fn wait_with_jitter(quota: Quota, jitter: Jitter) -> Self {
        Self::new(quota, jitter, MethodWait)
    }
}

impl<K> Handler<K> for KeyedRateLimitPredicate<K, NoJitter, MethodDiscard>
where
    K: Key + Sync,
{
    type Output = PredicateResult;
    type Future = Ready<Self::Output>;

    fn handle(&self, input: K) -> Self::Future {
        ready(if self.has_key(&input) {
            match self.limiter.check_key(&input) {
                Ok(_) => PredicateResult::True,
                Err(_) => {
                    log::info!("KeyedRateLimitPredicate: update discarded");
                    PredicateResult::False(Ok(()))
                }
            }
        } else {
            PredicateResult::True
        })
    }
}

impl<K> Handler<K> for KeyedRateLimitPredicate<K, NoJitter, MethodWait>
where
    K: Key + Sync + 'static,
{
    type Output = PredicateResult;
    type Future = Either<Ready<Self::Output>, BoxFuture<'static, Self::Output>>;

    fn handle(&self, input: K) -> Self::Future {
        if self.has_key(&input) {
            let limiter = self.limiter.clone();
            Either::Right(Box::pin(async move {
                limiter.until_key_ready(&input).await;
                PredicateResult::True
            }))
        } else {
            Either::Left(ready(PredicateResult::True))
        }
    }
}

impl<K> Handler<K> for KeyedRateLimitPredicate<K, Jitter, MethodWait>
where
    K: Key + Sync + 'static,
{
    type Output = PredicateResult;
    type Future = Either<Ready<Self::Output>, BoxFuture<'static, Self::Output>>;

    fn handle(&self, input: K) -> Self::Future {
        if self.has_key(&input) {
            let limiter = self.limiter.clone();
            let jitter = self.jitter;
            Either::Right(Box::pin(async move {
                limiter.until_key_ready_with_jitter(&input, jitter).await;
                PredicateResult::True
            }))
        } else {
            Either::Left(ready(PredicateResult::True))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ratelimit::key::{KeyChat, KeyChatUser, KeyUser};
    use std::time::Duration;

    #[tokio::test]
    async fn keyed() {
        macro_rules! test_key {
            ($key_type:ty, $first_key:expr, $second_key:expr) => {
                // discard
                let handler: KeyedRateLimitPredicate<$key_type, _, _> =
                    KeyedRateLimitPredicate::discard(Quota::per_minute(nonzero!(1u32)));
                assert!(
                    matches!(handler.handle($first_key).await, PredicateResult::True),
                    "[keyed/discard] handle({:?}) -> continue",
                    $first_key
                );
                assert!(
                    matches!(handler.handle($first_key).await, PredicateResult::False(Ok(()))),
                    "[keyed/discard] handle({:?}) -> stop",
                    $first_key
                );
                assert!(
                    matches!(handler.handle($second_key).await, PredicateResult::True),
                    "[keyed/discard] handle({:?}) -> continue",
                    $second_key
                );
                assert!(
                    matches!(handler.handle($second_key).await, PredicateResult::False(Ok(()))),
                    "[keyed/discard] handle({:?}) -> stop",
                    $second_key
                );

                // discard a specific key only
                let handler: KeyedRateLimitPredicate<$key_type, _, _> =
                    KeyedRateLimitPredicate::discard(Quota::per_minute(nonzero!(1u32))).with_key($second_key);
                assert!(
                    matches!(handler.handle($first_key).await, PredicateResult::True),
                    "[keyed/discard/with_key] handle({:?}) -> continue",
                    $first_key,
                );
                assert!(
                    matches!(handler.handle($first_key).await, PredicateResult::True),
                    "[keyed/discard/with_key] handle({:?}) -> continue",
                    $first_key,
                );
                assert!(
                    matches!(handler.handle($second_key).await, PredicateResult::True),
                    "[keyed/discard/with_key] handle({:?}) -> continue",
                    $first_key,
                );
                assert!(
                    matches!(handler.handle($second_key).await, PredicateResult::False(Ok(()))),
                    "[keyed/discard/with_key] handle({:?}) -> stop",
                    $first_key,
                );

                // wait
                let handler: KeyedRateLimitPredicate<$key_type, _, _> = KeyedRateLimitPredicate::wait(
                    Quota::with_period(Duration::from_millis(100))
                        .unwrap()
                        .allow_burst(nonzero!(1u32)),
                );
                for key in [$first_key, $first_key, $second_key, $second_key] {
                    assert!(
                        matches!(handler.handle(key).await, PredicateResult::True),
                        "[keyed/wait] handle({:?}) -> continue",
                        key
                    );
                }

                // wait a specific key only
                let handler: KeyedRateLimitPredicate<$key_type, _, _> = KeyedRateLimitPredicate::wait(
                    Quota::with_period(Duration::from_millis(100))
                        .unwrap()
                        .allow_burst(nonzero!(1u32)),
                )
                .with_key($second_key);
                for key in [$first_key, $first_key, $second_key, $second_key] {
                    assert!(
                        matches!(handler.handle(key).await, PredicateResult::True),
                        "[keyed/wait/with_key] handle({:?}) -> continue",
                        key
                    );
                }

                // wait with jitter
                let handler: KeyedRateLimitPredicate<$key_type, _, _> = KeyedRateLimitPredicate::wait_with_jitter(
                    Quota::with_period(Duration::from_millis(100))
                        .unwrap()
                        .allow_burst(nonzero!(1u32)),
                    Jitter::new(Duration::from_millis(0), Duration::from_millis(100)),
                );
                for key in [$first_key, $first_key, $second_key, $second_key] {
                    assert!(
                        matches!(handler.handle(key).await, PredicateResult::True),
                        "[keyed/wait_with_jitter] handle({:?}) -> continue",
                        key
                    );
                }

                // wait with jitter a specific key only
                let handler: KeyedRateLimitPredicate<$key_type, _, _> = KeyedRateLimitPredicate::wait_with_jitter(
                    Quota::with_period(Duration::from_millis(100))
                        .unwrap()
                        .allow_burst(nonzero!(1u32)),
                    Jitter::new(Duration::from_millis(0), Duration::from_millis(100)),
                )
                .with_key($second_key);
                for key in [$first_key, $first_key, $second_key, $second_key] {
                    assert!(
                        matches!(handler.handle(key).await, PredicateResult::True),
                        "[keyed/wait_with_jitter/with_key] handle({:?}) -> continue",
                        key
                    );
                }
            };
        }
        let chat_1 = KeyChat::from(1);
        let chat_2 = KeyChat::from(2);
        test_key!(KeyChat, chat_1, chat_2);
        let user_1 = KeyUser::from(1);
        let user_2 = KeyUser::from(2);
        test_key!(KeyUser, user_1, user_2);
        let chat_user_1 = KeyChatUser::from((1, 1));
        let chat_user_2 = KeyChatUser::from((1, 2));
        test_key!(KeyChatUser, chat_user_1, chat_user_2);
    }
}
