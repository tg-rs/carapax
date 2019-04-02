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

impl UpdateHandler for DirectRateLimitHandler {
    fn handle(&self, _context: &mut Context, _update: &Update) -> HandlerFuture {
        if self.limiter.lock().unwrap().check().is_ok() {
            HandlerResult::Continue
        } else {
            HandlerResult::Stop
        }
        .into()
    }
}
