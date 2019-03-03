use super::{Middleware, MiddlewareFuture, MiddlewareResult};
use crate::access::AccessPolicy;
use crate::api::Api;
use crate::types::Update;
use futures::Future;

/// Access control middleware
///
/// Helps to deny/allow updates from specific user/chat
pub struct AccessMiddleware<P> {
    policy: P,
}

impl<P> AccessMiddleware<P> {
    /// Creates a middleware with specified policy
    pub fn new(policy: P) -> Self {
        AccessMiddleware { policy }
    }
}

impl<P> Middleware for AccessMiddleware<P>
where
    P: AccessPolicy,
{
    fn before(&mut self, api: &Api, update: &Update) -> MiddlewareFuture {
        MiddlewareFuture::new(self.policy.is_granted(&api, &update).and_then(|result| {
            if result {
                Ok(MiddlewareResult::Continue)
            } else {
                Ok(MiddlewareResult::Stop)
            }
        }))
    }
}
