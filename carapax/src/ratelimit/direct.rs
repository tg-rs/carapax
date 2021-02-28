use crate::core::{Handler, HandlerResult};
use futures_util::future::BoxFuture;
use ratelimit_meter::{DirectRateLimiter, GCRA};
use std::{num::NonZeroU32, sync::Arc, time::Duration};
use tokio::sync::Mutex;

/// Limits all updates
#[derive(Clone)]
pub struct DirectRateLimitHandler {
    limiter: Arc<Mutex<DirectRateLimiter<GCRA>>>,
}

impl DirectRateLimitHandler {
    /// Creates a new handler
    ///
    /// # Arguments
    ///
    /// - capacity - Number of updates
    /// - duration - Per time unit
    pub fn new(capacity: NonZeroU32, duration: Duration) -> Self {
        Self {
            limiter: Arc::new(Mutex::new(DirectRateLimiter::new(capacity, duration))),
        }
    }
}

impl Handler<(), BoxFuture<'static, HandlerResult>> for DirectRateLimitHandler {
    fn call(&self, _param: ()) -> BoxFuture<'static, HandlerResult> {
        let limiter = self.limiter.clone();
        Box::pin(async move {
            if limiter.lock().await.check().is_ok() {
                HandlerResult::Continue
            } else {
                HandlerResult::Stop
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nonzero_ext::nonzero;

    #[tokio::test]
    async fn handler() {
        let handler = DirectRateLimitHandler::new(nonzero!(1u32), Duration::from_secs(1000));
        let mut results = Vec::new();
        for _ in 0..10 {
            results.push(handler.call(()).await)
        }
        assert!(results.into_iter().any(|x| matches!(x, HandlerResult::Stop)));
    }
}
