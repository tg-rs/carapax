use crate::{
    access::policy::AccessPolicy,
    core::{Handler, HandlerResult},
};
use async_trait::async_trait;
use tgbot::types::Update;

/// Access control handler
///
/// Helps to deny/allow updates from specific user/chat
pub struct AccessHandler<P> {
    policy: P,
}

impl<P> AccessHandler<P> {
    /// Creates a new handler with a specified policy
    pub fn new(policy: P) -> Self {
        AccessHandler { policy }
    }
}

#[async_trait]
impl<C, P> Handler<C> for AccessHandler<P>
where
    C: Send + Sync,
    P: AccessPolicy<C> + Send + Sync,
{
    type Input = Update;
    type Output = HandlerResult;

    async fn handle(&mut self, context: &C, update: Self::Input) -> Self::Output {
        if self.policy.is_granted(context, &update).await {
            HandlerResult::Continue
        } else {
            HandlerResult::Stop
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::policy::AccessPolicy;

    struct Policy {
        flag: bool,
    }

    impl Policy {
        fn new(flag: bool) -> Self {
            Self { flag }
        }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[async_trait]
    impl AccessPolicy<()> for Policy {
        async fn is_granted(&mut self, _context: &(), _update: &Update) -> bool {
            self.flag
        }
    }

    #[tokio::test]
    async fn handler() {
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
        let mut handler = AccessHandler::new(policy);
        match handler.handle(&(), update.clone()).await {
            HandlerResult::Continue => { /*ok*/ }
            result => panic!("Unexpected handler result: {:?}", result),
        }

        let policy = Policy::new(false);
        let mut handler = AccessHandler::new(policy);
        match handler.handle(&(), update).await {
            HandlerResult::Stop => { /*ok*/ }
            result => panic!("Unexpected handler result: {:?}", result),
        }
    }
}
