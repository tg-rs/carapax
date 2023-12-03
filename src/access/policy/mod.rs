use std::{convert::Infallible, error::Error, future::Future, sync::Arc};

use futures_util::future::{ok, Ready};

use crate::{access::rule::AccessRule, core::HandlerInput};

#[cfg(test)]
mod tests;

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
