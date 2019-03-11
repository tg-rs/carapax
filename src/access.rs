use crate::middleware::{Middleware, MiddlewareFuture, MiddlewareResult};
use failure::Error;
use futures::{future, Future, Poll};
use tgbot::types::{Integer, Update};

/// Access control middleware
///
/// Helps to deny/allow updates from specific user/chat
pub struct AccessMiddleware<P> {
    policy: P,
}

impl<P> AccessMiddleware<P> {
    /// Creates a middleware with specified policy
    pub fn new(policy: P) -> Self {
        AccessMiddleware { policy }
    }
}

impl<C, P> Middleware<C> for AccessMiddleware<P>
where
    P: AccessPolicy<C>,
{
    fn before(&mut self, context: &mut C, update: &Update) -> MiddlewareFuture {
        MiddlewareFuture::new(self.policy.is_granted(context, &update).and_then(|result| {
            if result {
                Ok(MiddlewareResult::Continue)
            } else {
                Ok(MiddlewareResult::Stop)
            }
        }))
    }
}

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

/// An access rule - contains information about principal and grant
#[derive(Debug)]
pub struct AccessRule {
    principal: Principal,
    is_granted: bool,
}

impl AccessRule {
    /// Creates a new rule
    pub fn new<P: Into<Principal>>(principal: P, is_granted: bool) -> Self {
        AccessRule {
            principal: principal.into(),
            is_granted,
        }
    }

    /// Creates a new rule with granted access
    pub fn allow<P: Into<Principal>>(principal: P) -> Self {
        Self::new(principal, true)
    }

    /// Creates a new rule with forbidden access
    pub fn deny<P: Into<Principal>>(principal: P) -> Self {
        Self::new(principal, false)
    }

    /// Creates a new rule with granted access for all
    pub fn allow_all() -> Self {
        Self::allow(Principal::All)
    }

    /// Creates a new rule with forbidden access for all
    pub fn deny_all() -> Self {
        Self::deny(Principal::All)
    }

    /// Creates a new rule with granted access for user
    pub fn allow_user<P: Into<PrincipalUser>>(principal: P) -> Self {
        Self::allow(principal.into())
    }

    /// Creates a new rule with forbidden access for user
    pub fn deny_user<P: Into<PrincipalUser>>(principal: P) -> Self {
        Self::deny(principal.into())
    }

    /// Creates a new rule with granted access for chat
    pub fn allow_chat<P: Into<PrincipalChat>>(principal: P) -> Self {
        Self::allow(principal.into())
    }

    /// Creates a new rule with forbidden access for chat
    pub fn deny_chat<P: Into<PrincipalChat>>(principal: P) -> Self {
        Self::deny(principal.into())
    }

    /// Creates a new rule with granted access for chat user
    pub fn allow_chat_user<C, U>(chat: C, user: U) -> Self
    where
        C: Into<PrincipalChat>,
        U: Into<PrincipalUser>,
    {
        Self::allow((chat.into(), user.into()))
    }

    /// Creates a new rule with forbidden access for chat user
    pub fn deny_chat_user<C, U>(chat: C, user: U) -> Self
    where
        C: Into<PrincipalChat>,
        U: Into<PrincipalUser>,
    {
        Self::deny((chat.into(), user.into()))
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

impl From<PrincipalUser> for Principal {
    fn from(principal: PrincipalUser) -> Principal {
        Principal::User(principal)
    }
}

impl From<PrincipalChat> for Principal {
    fn from(principal: PrincipalChat) -> Principal {
        Principal::Chat(principal)
    }
}

impl From<(PrincipalChat, PrincipalUser)> for Principal {
    fn from(principal: (PrincipalChat, PrincipalUser)) -> Principal {
        Principal::ChatUser(principal.0, principal.1)
    }
}

impl Principal {
    /// Creates a principal for user
    pub fn user<P: Into<PrincipalUser>>(principal: P) -> Self {
        Principal::User(principal.into())
    }

    /// Creates a principal for chat
    pub fn chat<P: Into<PrincipalChat>>(principal: P) -> Self {
        Principal::Chat(principal.into())
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

impl From<Integer> for PrincipalUser {
    fn from(user_id: Integer) -> PrincipalUser {
        PrincipalUser::Id(user_id)
    }
}

impl From<String> for PrincipalUser {
    fn from(username: String) -> PrincipalUser {
        PrincipalUser::Username(username)
    }
}

impl<'a> From<&'a str> for PrincipalUser {
    fn from(username: &'a str) -> PrincipalUser {
        PrincipalUser::Username(String::from(username))
    }
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

impl From<Integer> for PrincipalChat {
    fn from(chat_id: Integer) -> PrincipalChat {
        PrincipalChat::Id(chat_id)
    }
}

impl From<String> for PrincipalChat {
    fn from(username: String) -> PrincipalChat {
        PrincipalChat::Username(username)
    }
}

impl<'a> From<&'a str> for PrincipalChat {
    fn from(username: &'a str) -> PrincipalChat {
        PrincipalChat::Username(String::from(username))
    }
}

impl PrincipalChat {
    fn accepts(&self, update: &Update) -> bool {
        match self {
            PrincipalChat::Id(chat_id) => update.get_chat_id().map(|x| x == *chat_id),
            PrincipalChat::Username(ref chat_username) => update.get_chat_username().map(|x| x == chat_username),
        }
        .unwrap_or(false)
    }
}

impl Principal {
    fn accepts(&self, update: &Update) -> bool {
        match self {
            Principal::User(principal) => principal.accepts(&update),
            Principal::Chat(principal) => principal.accepts(&update),
            Principal::ChatUser(chat_principal, user_principal) =>
                chat_principal.accepts(&update) && user_principal.accepts(&update),
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

impl<C> AccessPolicy<C> for InMemoryAccessPolicy {
    fn is_granted(&mut self, _context: &mut C, update: &Update) -> AccessPolicyFuture {
        let mut result = false;
        for rule in &self.rules {
            if rule.accepts(&update) {
                result = rule.is_granted;
                log::info!("Found rule: {:?}", rule);
                break;
            }
        }
        result.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    struct MockPolicy {
        result: bool,
    }

    impl AccessPolicy<()> for MockPolicy {
        fn is_granted(&mut self, _: &mut (), _update: &Update) -> AccessPolicyFuture {
            self.result.into()
        }
    }

    #[test]
    fn test_middleware() {
        let update: Update = from_str(
            r#"{
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username1"},
                "chat": {"id": 1, "type": "private", "first_name": "test", "username": "username1"},
                "text": "test middleware"
            }
        }"#,
        )
        .unwrap();
        for &result in &[true, false] {
            let policy = MockPolicy { result };
            let mut middleware = AccessMiddleware::new(policy);
            let middleware_result = middleware.before(&mut (), &update).wait().unwrap();
            if result {
                assert_eq!(middleware_result, MiddlewareResult::Continue);
            } else {
                assert_eq!(middleware_result, MiddlewareResult::Stop);
            }
        }
    }

    #[test]
    fn test_in_memory_policy() {
        macro_rules! check_access {
            ($rules:expr, $updates:expr) => {{
                for rules in $rules {
                    let mut policy = InMemoryAccessPolicy::new(rules);
                    for (flag, update) in $updates {
                        let update: Update = from_str(update).unwrap();
                        let is_granted = policy.is_granted(&mut (), &update).wait().unwrap();
                        assert_eq!(is_granted, *flag);
                    }
                }
            }};
        }

        check_access!(
            vec![
                vec![AccessRule::allow_user(1)],
                vec![AccessRule::allow_user("username1")],
                vec![AccessRule::deny_user(2), AccessRule::allow_all()],
                vec![AccessRule::deny_user("username2"), AccessRule::allow_all()],
            ],
            &[
                (
                    true,
                    r#"{
                        "update_id": 1,
                        "message": {
                            "message_id": 1,
                            "date": 0,
                            "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username1"},
                            "chat": {"id": 1, "type": "private", "first_name": "test", "username": "username1"},
                            "text": "test allowed for user #1"
                        }
                    }"#
                ),
                (
                    false,
                    r#"{
                        "update_id": 1,
                        "message": {
                            "message_id": 2,
                            "date": 1,
                            "from": {"id": 2, "is_bot": false, "first_name": "test", "username": "username2"},
                            "chat": {"id": 2, "type": "private", "first_name": "test", "username": "username2"},
                            "text": "test denied for user #2"
                        }
                    }"#
                )
            ]
        );

        check_access!(
            vec![
                vec![AccessRule::allow_chat(1)],
                vec![AccessRule::allow_chat("username1")],
                vec![AccessRule::deny_chat(2), AccessRule::allow_all()],
                vec![AccessRule::deny_chat("username2"), AccessRule::allow_all()],
            ],
            &[
                (
                    true,
                    r#"{
                        "update_id": 1,
                        "message": {
                            "message_id": 1,
                            "date": 0,
                            "from": {"id": 111, "is_bot": false, "first_name": "test"},
                            "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username1"},
                            "text": "test allowed for chat #1"
                        }
                    }"#
                ),
                (
                    false,
                    r#"{
                        "update_id": 1,
                        "message": {
                            "message_id": 2,
                            "date": 1,
                            "from": {"id": 111, "is_bot": false, "first_name": "test"},
                            "chat": {"id": 2, "type": "supergroup", "title": "test", "username": "username2"},
                            "text": "test denied for chat #2"
                        }
                    }"#
                )
            ]
        );

        check_access!(
            vec![
                vec![AccessRule::allow_chat_user(1, 2)],
                vec![AccessRule::allow_chat_user("username1", 2)],
                vec![AccessRule::allow_chat_user(1, "username2")],
                vec![AccessRule::allow_chat_user("username1", "username2")],
                vec![
                    AccessRule::deny_chat_user(1, 3),
                    AccessRule::deny_chat_user(4, 3),
                    AccessRule::allow_all()
                ],
                vec![
                    AccessRule::deny_chat_user("username1", "username3"),
                    AccessRule::deny_chat_user(4, 3),
                    AccessRule::allow_all()
                ],
            ],
            &[
                (
                    true,
                    r#"{
                        "update_id": 1,
                        "message": {
                            "message_id": 1,
                            "date": 0,
                            "from": {"id": 2, "is_bot": false, "first_name": "test", "username": "username2"},
                            "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username1"},
                            "text": "test allowed for user in chat"
                        }
                    }"#
                ),
                (
                    false,
                    r#"{
                        "update_id": 1,
                        "message": {
                            "message_id": 2,
                            "date": 1,
                            "from": {"id": 3, "is_bot": false, "first_name": "test", "username": "username3"},
                            "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username1"},
                            "text": "test denied for user in chat"
                        }
                    }"#
                ),
                (
                    false,
                    r#"{
                        "update_id": 1,
                        "message": {
                            "message_id": 2,
                            "date": 1,
                            "from": {"id": 3, "is_bot": false, "first_name": "test", "username": "username3"},
                            "chat": {"id": 4, "type": "supergroup", "title": "test", "username": "username4"},
                            "text": "test denied for chat and user"
                        }
                    }"#
                )
            ]
        );
    }
}
