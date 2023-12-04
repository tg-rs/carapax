use std::{convert::Infallible, error::Error, future::Future, sync::Arc};

use futures_util::future::{ok, Ready};

use crate::{access::rule::AccessRule, core::HandlerInput};

#[cfg(test)]
mod tests;

/// Decides whether [`HandlerInput`] should be processed or not.
pub trait AccessPolicy: Send {
    /// An error that may be returned by the [`Self::is_granted`] method.
    type Error: Error + Send;
    /// A future representing the result of the [`Self::is_granted`] method.
    type Future: Future<Output = Result<bool, Self::Error>> + Send;

    /// Determines if access is granted for the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to be processed by the access policy.
    ///
    /// The [`Self::Future`] resolves to `true` if access is allowed, and `false` otherwise.
    fn is_granted(&self, input: HandlerInput) -> Self::Future;
}

/// In-memory access policy implementation.
///
/// If there are no rules found, [`AccessPolicy::is_granted`] will return `false`.
/// You can use [`AccessRule::allow_all`] as the last rule to modify this behavior.
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
