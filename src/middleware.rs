use crate::policy::AccessPolicy;
use carapax::prelude::*;
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

impl<C, P> Middleware<C> for AccessMiddleware<P>
where
    P: AccessPolicy<C>,
{
    fn before(&mut self, context: &mut C, update: &Update) -> MiddlewareFuture {
        MiddlewareFuture::new(self.policy.is_granted(context, &update).and_then(|result| {
            if result {
                Ok(MiddlewareResult::Continue)
            } else {
                Ok(MiddlewareResult::Stop)
            }
        }))
    }
}
