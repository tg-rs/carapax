use crate::policy::AccessPolicy;
use carapax::prelude::*;
use futures::Future;

/// Access control handler
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

impl<P> Handler for AccessHandler<P>
where
    P: AccessPolicy,
{
    type Input = Update;
    type Output = HandlerFuture;

    fn handle(&self, context: &mut Context, update: Self::Input) -> Self::Output {
        HandlerFuture::new(self.policy.is_granted(context, &update).and_then(|result| {
            if result {
                Ok(HandlerResult::Continue)
            } else {
                Ok(HandlerResult::Stop)
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{AccessPolicy, AccessPolicyFuture};
    use carapax::Context;

    struct Policy {
        flag: bool,
    }

    impl Policy {
        fn new(flag: bool) -> Self {
            Self { flag }
        }
    }

    impl AccessPolicy for Policy {
        fn is_granted(&self, _context: &mut Context, _update: &Update) -> AccessPolicyFuture {
            self.flag.into()
        }
    }

    #[test]
    fn handler() {
        let mut context = Context::default();
        let update: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "message": {
                    "message_id": 1,
                    "date": 0,
                    "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username1"},
                    "chat": {"id": 1, "type": "private", "first_name": "test", "username": "username1"},
                    "text": "test middleware"
                }
            }
        ))
        .unwrap();

        let policy = Policy::new(true);
        let handler = AccessHandler::new(policy);
        let result = handler.handle(&mut context, update.clone()).wait().unwrap();
        assert_eq!(result, HandlerResult::Continue);

        let policy = Policy::new(false);
        let handler = AccessHandler::new(policy);
        let result = handler.handle(&mut context, update).wait().unwrap();
        assert_eq!(result, HandlerResult::Stop);
    }
}
