use crate::rules::AccessRule;
use carapax::prelude::*;
use failure::Error;
use futures::{future, Future, Poll};

/// An access policy
///
/// Decides whether update is allowed or not
pub trait AccessPolicy<C> {
    /// Return true if update is allowed and false otherwise
    fn is_granted(&mut self, context: &mut C, update: &Update) -> AccessPolicyFuture;
}

/// Access policy future
#[must_use = "futures do nothing unless polled"]
pub struct AccessPolicyFuture {
    inner: Box<Future<Item = bool, Error = Error> + Send>,
}

impl AccessPolicyFuture {
    /// Creates a future
    pub fn new<F>(f: F) -> Self
    where
        F: Future<Item = bool, Error = Error> + Send + 'static,
    {
        AccessPolicyFuture { inner: Box::new(f) }
    }
}

impl From<bool> for AccessPolicyFuture {
    fn from(flag: bool) -> AccessPolicyFuture {
        AccessPolicyFuture::new(future::ok(flag))
    }
}

impl Future for AccessPolicyFuture {
    type Item = bool;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

/// In-memory access policy
///
/// Stores all rules in a Vec
///
/// If there are no rules found for update, is_granted() will return false
/// You can use `Rule::allow(Principal::All)` as last rule in order to change this behaviour
#[derive(Default)]
pub struct InMemoryAccessPolicy {
    rules: Vec<AccessRule>,
}

impl InMemoryAccessPolicy {
    /// Creates a new policy
    pub fn new(rules: Vec<AccessRule>) -> Self {
        InMemoryAccessPolicy { rules }
    }

    /// Adds a rule to the end of the list
    pub fn push_rule(mut self, rule: AccessRule) -> Self {
        self.rules.push(rule);
        self
    }
}

impl<C> AccessPolicy<C> for InMemoryAccessPolicy {
    fn is_granted(&mut self, _context: &mut C, update: &Update) -> AccessPolicyFuture {
        let mut result = false;
        for rule in &self.rules {
            if rule.accepts(&update) {
                result = rule.is_granted();
                log::info!("Found rule: {:?}", rule);
                break;
            }
        }
        result.into()
    }
}
