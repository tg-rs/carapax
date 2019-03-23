use crate::policy::AccessPolicy;
use carapax::prelude::*;
use futures::Future;

/// Access control middleware
///
/// Helps to deny/allow updates from specific user/chat
pub struct AccessHandler<P> {
    policy: P,
}

impl<P> AccessHandler<P> {
    /// Creates a middleware with specified policy
    pub fn new(policy: P) -> Self {
        AccessHandler { policy }
    }
}

impl<P> UpdateHandler for AccessHandler<P>
where
    P: AccessPolicy,
{
    fn handle(&self, context: &mut Context, update: &Update) -> HandlerFuture {
        HandlerFuture::new(self.policy.is_granted(context, &update).and_then(|result| {
            if result {
                Ok(HandlerResult::Continue)
            } else {
                Ok(HandlerResult::Stop)
            }
        }))
    }
}
