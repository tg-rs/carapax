/// Discards updates when the rate limit is reached.
#[derive(Clone, Copy, Debug)]
pub struct MethodDiscard;

/// Allows update to pass as soon as the rate limiter allows it.
#[derive(Clone, Copy, Debug)]
pub struct MethodWait;
