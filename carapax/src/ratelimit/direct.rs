use crate::core::{Handler, HandlerResult};
use async_trait::async_trait;
use ratelimit_meter::{DirectRateLimiter, GCRA};
use std::{num::NonZeroU32, sync::Arc, time::Duration};
use tgbot::types::Update;
use tokio::sync::Mutex;

/// Limits all updates
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

#[async_trait]
impl<C> Handler<C> for DirectRateLimitHandler
where
    C: Send + Sync,
{
    type Input = Update;
    type Output = HandlerResult;

    async fn handle(&mut self, _context: &C, _update: Self::Input) -> Self::Output {
        if self.limiter.lock().await.check().is_ok() {
            HandlerResult::Continue
        } else {
            HandlerResult::Stop
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nonzero_ext::nonzero;
    use tgbot::types::Update;

    #[tokio::test]
    async fn handler() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 1,
                "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                "text": "test"
            }
        }))
        .unwrap();
        let mut handler = DirectRateLimitHandler::new(nonzero!(1u32), Duration::from_secs(1000));
        let mut results = Vec::new();
        for _ in 0..10 {
            results.push(handler.handle(&(), update.clone()).await)
        }
        assert!(results.into_iter().any(|x| matches!(x, HandlerResult::Stop)));
    }
}
