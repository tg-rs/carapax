use crate::{
    core::{Handler, HandlerResult, PredicateResult},
    ratelimit::{
        jitter::NoJitter,
        method::{MethodDiscard, MethodWait},
    },
};
use futures_util::future::{ready, BoxFuture, Ready};
use governor::{
    clock::DefaultClock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
    RateLimiter,
};
use std::sync::Arc;

pub use governor::{Jitter, Quota};
pub use nonzero_ext::nonzero;

/// A predicate with direct ratelimiter
///
/// Use this predicate when you need to limit all updates
#[derive(Clone)]
pub struct DirectRateLimitPredicate<J, M> {
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>>,
    jitter: J,
    _method: M,
}

impl DirectRateLimitPredicate<NoJitter, MethodDiscard> {
    /// Creates a new predicate with discard method
    ///
    /// Predicate will stop update propagation when the rate limit is reached
    ///
    /// # Arguments
    ///
    /// * quota - A rate-limiting quota
    pub fn discard(quota: Quota) -> Self {
        Self {
            limiter: Arc::new(RateLimiter::direct(quota)),
            jitter: NoJitter,
            _method: MethodDiscard,
        }
    }
}

impl DirectRateLimitPredicate<NoJitter, MethodWait> {
    /// Creates a new predicate with wait method
    ///
    /// Predicate will pause update propagation when the rate limit is reached
    ///
    /// # Arguments
    ///
    /// * quota - A rate-limiting quota
    pub fn wait(quota: Quota) -> Self {
        Self {
            limiter: Arc::new(RateLimiter::direct(quota)),
            jitter: NoJitter,
            _method: MethodWait,
        }
    }
}

impl DirectRateLimitPredicate<Jitter, MethodWait> {
    /// Creates a new predicate with wait method and jitter
    ///
    /// Predicate will pause update propagation when the rate limit is reached
    ///
    /// # Arguments
    ///
    /// * quota - A rate-limiting quota
    /// * jitter - An interval specification for deviating from the nominal wait time
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
    type Future = Ready<Self::Output>;

    fn handle(&self, (): ()) -> Self::Future {
        ready(match self.limiter.check() {
            Ok(_) => PredicateResult::True,
            Err(_) => {
                log::info!("DirectRateLimitPredicate: update discarded");
                PredicateResult::False(HandlerResult::Ok)
            }
        })
    }
}

impl Handler<()> for DirectRateLimitPredicate<NoJitter, MethodWait> {
    type Output = PredicateResult;
    type Future = BoxFuture<'static, Self::Output>;

    fn handle(&self, (): ()) -> Self::Future {
        let limiter = self.limiter.clone();
        Box::pin(async move {
            limiter.until_ready().await;
            PredicateResult::True
        })
    }
}

impl Handler<()> for DirectRateLimitPredicate<Jitter, MethodWait> {
    type Output = PredicateResult;
    type Future = BoxFuture<'static, Self::Output>;

    fn handle(&self, (): ()) -> Self::Future {
        let limiter = self.limiter.clone();
        let jitter = self.jitter;
        Box::pin(async move {
            limiter.until_ready_with_jitter(jitter).await;
            PredicateResult::True
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn direct() {
        let handler = DirectRateLimitPredicate::discard(Quota::per_minute(nonzero!(1u32)));
        assert!(
            matches!(handler.handle(()).await, PredicateResult::True),
            "[direct/discard/1] true"
        );
        assert!(
            matches!(handler.handle(()).await, PredicateResult::False(HandlerResult::Ok)),
            "[direct/discard/2] false"
        );

        let handler = DirectRateLimitPredicate::wait(
            Quota::with_period(Duration::from_millis(100))
                .unwrap()
                .allow_burst(nonzero!(1u32)),
        );
        assert!(
            matches!(handler.handle(()).await, PredicateResult::True),
            "[direct/wait/1] continue"
        );
        assert!(
            matches!(handler.handle(()).await, PredicateResult::True),
            "[direct/wait/2] continue"
        );

        let handler = DirectRateLimitPredicate::wait_with_jitter(
            Quota::with_period(Duration::from_millis(100))
                .unwrap()
                .allow_burst(nonzero!(1u32)),
            Jitter::new(Duration::from_secs(0), Duration::from_millis(100)),
        );
        assert!(
            matches!(handler.handle(()).await, PredicateResult::True),
            "[direct/wait_with_jitter/1] continue"
        );
        assert!(
            matches!(handler.handle(()).await, PredicateResult::True),
            "[direct/wait_with_jitter/2] continue"
        );
    }
}
