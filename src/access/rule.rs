use crate::{
    access::principal::{Principal, PrincipalChat, PrincipalUser},
    types::Update,
};

/// Contains information about principal and grant
#[derive(Debug)]
pub struct AccessRule {
    principal: Principal,
    is_granted: bool,
}

impl AccessRule {
    /// Creates a new rule
    ///
    /// # Arguments
    ///
    /// * principal - A principal
    /// * is_granted - Whether access granted or not
    pub fn new<P: Into<Principal>>(principal: P, is_granted: bool) -> Self {
        AccessRule {
            principal: principal.into(),
            is_granted,
        }
    }

    /// Creates a new rule with granted access
    ///
    /// # Arguments
    ///
    /// * principal - A principal
    pub fn allow<P: Into<Principal>>(principal: P) -> Self {
        Self::new(principal, true)
    }

    /// Creates a new rule with forbidden access
    ///
    /// # Arguments
    ///
    /// * principal - A principal
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

    /// Creates a new rule with granted access for a user
    pub fn allow_user<P: Into<PrincipalUser>>(principal: P) -> Self {
        Self::allow(principal.into())
    }

    /// Creates a new rule with forbidden access for a user
    pub fn deny_user<P: Into<PrincipalUser>>(principal: P) -> Self {
        Self::deny(principal.into())
    }

    /// Creates a new rule with granted access for a chat
    pub fn allow_chat<P: Into<PrincipalChat>>(principal: P) -> Self {
        Self::allow(principal.into())
    }

    /// Creates a new rule with forbidden access for a chat
    pub fn deny_chat<P: Into<PrincipalChat>>(principal: P) -> Self {
        Self::deny(principal.into())
    }

    /// Creates a new rule with granted access for a chat user
    pub fn allow_chat_user<C, U>(chat: C, user: U) -> Self
    where
        C: Into<PrincipalChat>,
        U: Into<PrincipalUser>,
    {
        Self::allow((chat.into(), user.into()))
    }

    /// Creates a new rule with forbidden access for a chat user
    pub fn deny_chat_user<C, U>(chat: C, user: U) -> Self
    where
        C: Into<PrincipalChat>,
        U: Into<PrincipalUser>,
    {
        Self::deny((chat.into(), user.into()))
    }

    /// Returns `true` if rule accepts an update and `false` otherwise
    pub fn accepts(&self, update: &Update) -> bool {
        self.principal.accepts(update)
    }

    /// Returns `true` if access is granted and `false` otherwise
    pub fn is_granted(&self) -> bool {
        self.is_granted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tgbot::types::Update;

    #[test]
    fn access_rule_new() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 1,
                "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                "text": "test"
            }
        }))
        .unwrap();

        let principal_chat = Principal::from(PrincipalChat::from(1));
        let principal_user = Principal::from(PrincipalUser::from(1));

        let rule = AccessRule::new(principal_user.clone(), true);
        assert_eq!(rule.principal, principal_user);
        assert!(rule.is_granted());
        assert!(rule.accepts(&update));

        let rule = AccessRule::new(principal_chat.clone(), false);
        assert_eq!(rule.principal, principal_chat);
        assert!(!rule.is_granted());
        assert!(rule.accepts(&update));
    }

    #[test]
    fn access_rule_allow_deny() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 1,
                "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                "text": "test"
            }
        }))
        .unwrap();

        let principal_chat = Principal::from(PrincipalChat::from(1));
        let principal_user = Principal::from(PrincipalUser::from(1));

        let rule = AccessRule::allow(principal_user.clone());
        assert_eq!(rule.principal, principal_user);
        assert!(rule.is_granted());
        assert!(rule.accepts(&update));

        let rule = AccessRule::deny(principal_chat.clone());
        assert_eq!(rule.principal, principal_chat);
        assert!(!rule.is_granted());
        assert!(rule.accepts(&update));
    }

    #[test]
    fn access_rule_principal_all() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 1,
                "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                "text": "test"
            }
        }))
        .unwrap();

        let rule = AccessRule::allow_all();
        assert_eq!(rule.principal, Principal::All);
        assert!(rule.is_granted());
        assert!(rule.accepts(&update));

        let rule = AccessRule::deny_all();
        assert_eq!(rule.principal, Principal::All);
        assert!(!rule.is_granted());
        assert!(rule.accepts(&update));
    }

    #[test]
    fn access_rule_principal_user() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 1,
                "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                "text": "test"
            }
        }))
        .unwrap();

        let principal_user = Principal::from(PrincipalUser::from(1));

        let rule = AccessRule::allow_user(1);
        assert_eq!(rule.principal, principal_user);
        assert!(rule.is_granted());
        assert!(rule.accepts(&update));

        let rule = AccessRule::deny_user(1);
        assert_eq!(rule.principal, principal_user);
        assert!(!rule.is_granted());
        assert!(rule.accepts(&update));
    }

    #[test]
    fn access_rule_principal_chat() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 1,
                "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                "text": "test"
            }
        }))
        .unwrap();

        let principal_chat = Principal::from(PrincipalChat::from(1));

        let rule = AccessRule::allow_chat(1);
        assert_eq!(rule.principal, principal_chat);
        assert!(rule.is_granted());
        assert!(rule.accepts(&update));

        let rule = AccessRule::deny_chat(1);
        assert_eq!(rule.principal, principal_chat);
        assert!(!rule.is_granted());
        assert!(rule.accepts(&update));
    }

    #[test]
    fn access_rule_principal_chat_user() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 1,
                "from": {"id": 1, "is_bot": false, "first_name": "test", "username": "username_user"},
                "chat": {"id": 1, "type": "supergroup", "title": "test", "username": "username_chat"},
                "text": "test"
            }
        }))
        .unwrap();

        let rule = AccessRule::allow_chat_user(1, 1);
        assert_eq!(
            rule.principal,
            Principal::from((PrincipalChat::from(1), PrincipalUser::from(1)))
        );
        assert!(rule.is_granted());
        assert!(rule.accepts(&update));

        let rule = AccessRule::deny_chat_user(1, 1);
        assert_eq!(
            rule.principal,
            Principal::from((PrincipalChat::from(1), PrincipalUser::from(1)))
        );
        assert!(!rule.is_granted());
        assert!(rule.accepts(&update));
    }
}
