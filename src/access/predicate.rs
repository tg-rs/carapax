use crate::{
    access::policy::AccessPolicy,
    core::{Handler, HandlerInput, PredicateResult},
    HandlerError,
};
use futures_util::future::BoxFuture;

/// Allows to protect a handler with an access policy
#[derive(Clone)]
pub struct AccessPredicate<P> {
    policy: P,
}

impl<P> AccessPredicate<P> {
    /// Creates a new AccessPredicate
    ///
    /// # Arguments
    ///
    /// * policy - An access policy
    pub fn new(policy: P) -> Self {
        Self { policy }
    }
}

impl<P> Handler<HandlerInput> for AccessPredicate<P>
where
    P: AccessPolicy + Clone + Sync + 'static,
{
    type Output = PredicateResult;
    type Future = BoxFuture<'static, Self::Output>;

    fn handle(&self, input: HandlerInput) -> Self::Future {
        let policy = self.policy.clone();
        Box::pin(async move {
            let user = input.update.get_user().cloned();
            match policy.is_granted(input).await {
                Ok(true) => {
                    log::info!("Access granted for {:?}", user);
                    PredicateResult::True
                }
                Ok(false) => {
                    log::info!("Access forbidden for {:?}", user);
                    PredicateResult::False(Ok(()))
                }
                Err(err) => PredicateResult::False(Err(HandlerError::new(err))),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{core::HandlerInput, types::Update};
    use futures_util::future::{err, ok, Ready};
    use std::{error::Error, fmt};

    #[derive(Debug)]
    struct ErrorMock;

    impl Error for ErrorMock {}

    impl fmt::Display for ErrorMock {
        fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(out, "error")
        }
    }

    #[derive(Clone)]
    struct PolicyMock;

    impl AccessPolicy for PolicyMock {
        type Error = ErrorMock;
        type Future = Ready<Result<bool, Self::Error>>;

        fn is_granted(&self, input: HandlerInput) -> Self::Future {
            match input.update.get_user().map(|user| user.id) {
                Some(1) => ok(true),
                Some(2) => ok(false),
                Some(_) => err(ErrorMock),
                None => err(ErrorMock),
            }
        }
    }

    #[tokio::test]
    async fn access_predicate() {
        let policy = PolicyMock;
        let predicate = AccessPredicate::new(policy);

        let update_granted: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "message": {
                    "message_id": 1111,
                    "date": 0,
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "private", "first_name": "test"},
                    "text": "test",
                }
            }
        ))
        .unwrap();
        let input_granted = HandlerInput::from(update_granted);
        let result = predicate.handle(input_granted).await;
        assert!(matches!(result, PredicateResult::True));

        let update_forbidden: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "message": {
                    "message_id": 1111,
                    "date": 0,
                    "from": {"id": 2, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "private", "first_name": "test"},
                    "text": "test",
                }
            }
        ))
        .unwrap();
        let input_forbidden = HandlerInput::from(update_forbidden);
        let result = predicate.handle(input_forbidden).await;
        assert!(matches!(result, PredicateResult::False(Ok(()))));

        let update_error: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "message": {
                    "message_id": 1111,
                    "date": 0,
                    "from": {"id": 3, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "private", "first_name": "test"},
                    "text": "test",
                }
            }
        ))
        .unwrap();
        let input_error = HandlerInput::from(update_error);
        let result = predicate.handle(input_error).await;
        assert!(matches!(result, PredicateResult::False(Err(_))));
    }
}
