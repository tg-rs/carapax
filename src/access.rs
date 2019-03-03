use crate::api::Api;
use crate::types::{Integer, Update};
use failure::Error;
use futures::{future, Future, Poll};

/// An access policy
///
/// Decides whether update is allowed or not
pub trait AccessPolicy {
    /// Return true if update is allowed and false otherwise
    fn is_granted(&mut self, api: &Api, update: &Update) -> AccessPolicyFuture;
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

/// An access rule - contains information about principal and grant
#[derive(Debug)]
pub struct AccessRule {
    principal: Principal,
    is_granted: bool,
}

impl AccessRule {
    /// Creates a new rule
    pub fn new(principal: Principal, is_granted: bool) -> Self {
        AccessRule {
            principal,
            is_granted,
        }
    }

    /// Creates a new rule with granted access
    pub fn allow(principal: Principal) -> Self {
        Self::new(principal, true)
    }

    /// Creates a new rule with forbidden access
    pub fn deny(principal: Principal) -> Self {
        Self::new(principal, false)
    }

    /// Whether rule accepts an update
    pub fn accepts(&self, update: &Update) -> bool {
        self.principal.accepts(update)
    }
}

/// Principal helps to decide should rule accept an update or not
#[derive(Debug)]
pub enum Principal {
    /// Accepts all updates
    All,
    /// Accepts updates only from a specified user
    User(PrincipalUser),
    /// Accepts updates only from a specified chat
    Chat(PrincipalChat),
    /// Accepts updates only from a user in chat
    ChatUser(PrincipalChat, PrincipalUser),
}

impl Principal {
    /// Creates a principal for user ID
    pub fn user_id(user_id: Integer) -> Self {
        Principal::User(PrincipalUser::Id(user_id))
    }

    /// Creates a principal for @username
    pub fn username<S: Into<String>>(username: S) -> Self {
        Principal::User(PrincipalUser::Username(username.into()))
    }

    /// Creates a principal for chat ID
    pub fn chat_id(chat_id: Integer) -> Self {
        Principal::Chat(PrincipalChat::Id(chat_id))
    }

    /// Creates a principal for chat @username
    pub fn chat_username<S: Into<String>>(username: S) -> Self {
        Principal::Chat(PrincipalChat::Username(username.into()))
    }
}

/// Represents a user
#[derive(Debug)]
pub enum PrincipalUser {
    /// Accepts updates only from a user with specified ID
    Id(Integer),
    /// Accepts updates only from a user with specified @username
    Username(String),
}

impl PrincipalUser {
    fn accepts(&self, update: &Update) -> bool {
        match self {
            PrincipalUser::Id(user_id) => update.get_user().map(|u| u.id == *user_id),
            PrincipalUser::Username(ref username) => update.get_user().and_then(|u| {
                if let Some(ref x) = u.username {
                    Some(x == username)
                } else {
                    None
                }
            }),
        }
        .unwrap_or(false)
    }
}

/// Represents a chat
#[derive(Debug)]
pub enum PrincipalChat {
    /// Accepts updates only from a chat with specified ID
    Id(Integer),
    /// Accepts updates only from a chat with specified @username
    Username(String),
}

impl PrincipalChat {
    fn accepts(&self, update: &Update) -> bool {
        match self {
            PrincipalChat::Id(chat_id) => update.get_chat_id().map(|x| x == *chat_id),
            PrincipalChat::Username(ref chat_username) => {
                update.get_chat_username().map(|x| x == chat_username)
            }
        }
        .unwrap_or(false)
    }
}

impl Principal {
    fn accepts(&self, update: &Update) -> bool {
        match self {
            Principal::User(principal) => principal.accepts(&update),
            Principal::Chat(principal) => principal.accepts(&update),
            Principal::ChatUser(chat_principal, user_principal) => {
                chat_principal.accepts(&update) && user_principal.accepts(&update)
            }
            Principal::All => true,
        }
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

impl AccessPolicy for InMemoryAccessPolicy {
    fn is_granted(&mut self, _api: &Api, update: &Update) -> AccessPolicyFuture {
        let mut result = false;
        for rule in &self.rules {
            if rule.accepts(&update) {
                result = rule.is_granted;
                log::info!("Granted access for: {:?}", rule);
                break;
            }
        }
        log::info!("No rules found, access forbidden");
        result.into()
    }
}
