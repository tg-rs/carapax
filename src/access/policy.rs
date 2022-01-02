use crate::{access::rule::AccessRule, core::HandlerInput};
use futures_util::future::{ok, Ready};
use std::{convert::Infallible, error::Error, future::Future, sync::Arc};

/// Decides whether input should be processed or not
pub trait AccessPolicy: Send {
    /// An error returned by `is_granted` method
    type Error: Error + Send;
    /// A future returned by `is_granted` method
    type Future: Future<Output = Result<bool, Self::Error>> + Send;

    /// Returns `true` if access is allowed and `false` otherwise
    fn is_granted(&self, input: HandlerInput) -> Self::Future;
}

/// In-memory access policy
///
/// If there are no rules found, `is_granted()` will return `false`.
/// You can use [`allow_all()`](struct.AccessRule.html#method.allow_all)
/// as a last rule in order to change this behaviour.
#[derive(Default, Clone)]
pub struct InMemoryAccessPolicy {
    rules: Arc<Vec<AccessRule>>,
}

impl<T> From<T> for InMemoryAccessPolicy
where
    T: IntoIterator<Item = AccessRule>,
{
    fn from(rules: T) -> Self {
        Self {
            rules: Arc::new(rules.into_iter().collect()),
        }
    }
}

impl AccessPolicy for InMemoryAccessPolicy {
    type Error = Infallible;
    type Future = Ready<Result<bool, Self::Error>>;

    fn is_granted(&self, input: HandlerInput) -> Self::Future {
        let mut result = false;
        let rules = Arc::clone(&self.rules);
        for rule in rules.iter() {
            if rule.accepts(&input.update) {
                result = rule.is_granted();
                log::info!("Found rule: {:?} (is_granted={:?})", rule, result);
                break;
            }
        }
        ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Update;

    #[tokio::test]
    async fn in_memory_access_policy() {
        let policy = InMemoryAccessPolicy::from(vec![AccessRule::allow_user(1)]);

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
        assert!(policy.is_granted(input_granted).await.unwrap());

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
        assert!(!policy.is_granted(input_forbidden).await.unwrap());
    }
}
