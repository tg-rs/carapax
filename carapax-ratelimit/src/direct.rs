use carapax::prelude::*;
use ratelimit_meter::{DirectRateLimiter, GCRA};
use std::{
    num::NonZeroU32,
    sync::{Arc, Mutex},
    time::Duration,
};

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

impl Handler for DirectRateLimitHandler {
    type Item = Update;
    type Result = HandlerResult;

    fn handle(&self, _context: &mut Context, _update: Self::Item) -> Self::Result {
        if self.limiter.lock().unwrap().check().is_ok() {
            HandlerResult::Continue
        } else {
            HandlerResult::Stop
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nonzero;
    use carapax::{core::types::Update, Context};

    #[test]
    fn handler() {
        let mut context = Context::default();
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
        let handler = DirectRateLimitHandler::new(nonzero!(1u32), Duration::from_secs(1000));
        let mut items = Vec::new();
        for _ in 0..10 {
            let result = handler.handle(&mut context, update.clone());
            items.push(result)
        }
        assert!(items.into_iter().any(|x| x == HandlerResult::Stop))
    }
}
