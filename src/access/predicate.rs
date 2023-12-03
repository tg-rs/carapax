use std::fmt;

use futures_util::future::BoxFuture;

use crate::{
    access::policy::AccessPolicy,
    core::{Handler, HandlerInput},
    types::{ChatPeerId, ChatUsername, Update, UserPeerId, UserUsername},
};

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
    type Output = Result<bool, P::Error>;
    type Future = BoxFuture<'static, Self::Output>;

    fn handle(&self, input: HandlerInput) -> Self::Future {
        let policy = self.policy.clone();
        Box::pin(async move {
            let debug_principal = DebugPrincipal::from(&input.update);
            policy.is_granted(input).await.map(|value| {
                log::info!(
                    "Access for {:?} is {}",
                    debug_principal,
                    if value { "granted" } else { "forbidden" }
                );
                value
            })
        })
    }
}

struct DebugPrincipal {
    user_id: Option<UserPeerId>,
    user_username: Option<UserUsername>,
    chat_id: Option<ChatPeerId>,
    chat_username: Option<ChatUsername>,
}

impl From<&Update> for DebugPrincipal {
    fn from(update: &Update) -> Self {
        DebugPrincipal {
            user_id: update.get_user_id(),
            user_username: update.get_user_username().cloned(),
            chat_id: update.get_chat_id(),
            chat_username: update.get_chat_username().cloned(),
        }
    }
}

impl fmt::Debug for DebugPrincipal {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_struct = out.debug_struct("Principal");
        macro_rules! debug_field {
            ($field_name:ident) => {
                if let Some(ref $field_name) = self.$field_name {
                    debug_struct.field(stringify!($field_name), &$field_name);
                }
            };
        }
        debug_field!(user_id);
        debug_field!(user_username);
        debug_field!(chat_id);
        debug_field!(chat_username);
        debug_struct.finish()
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fmt};

    use futures_util::future::{err, ok, Ready};

    use crate::{
        core::HandlerInput,
        types::{Integer, Update},
    };

    use super::*;

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
            match input.update.get_user().map(|user| Integer::from(user.id)) {
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
        assert!(matches!(result, Ok(true)));

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
        assert!(matches!(result, Ok(false)));

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
        assert!(matches!(result, Err(_)));
    }
}
