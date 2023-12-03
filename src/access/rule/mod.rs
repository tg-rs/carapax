use crate::{
    access::principal::Principal,
    types::{ChatId, Update, UserId},
};

#[cfg(test)]
mod tests;

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
    pub fn allow_user<P: Into<UserId>>(principal: P) -> Self {
        Self::allow(principal.into())
    }

    /// Creates a new rule with forbidden access for a user
    pub fn deny_user<P: Into<UserId>>(principal: P) -> Self {
        Self::deny(principal.into())
    }

    /// Creates a new rule with granted access for a chat
    pub fn allow_chat<P: Into<ChatId>>(principal: P) -> Self {
        Self::allow(principal.into())
    }

    /// Creates a new rule with forbidden access for a chat
    pub fn deny_chat<P: Into<ChatId>>(principal: P) -> Self {
        Self::deny(principal.into())
    }

    /// Creates a new rule with granted access for a chat user
    pub fn allow_chat_user<C, U>(chat: C, user: U) -> Self
    where
        C: Into<ChatId>,
        U: Into<UserId>,
    {
        Self::allow((chat.into(), user.into()))
    }

    /// Creates a new rule with forbidden access for a chat user
    pub fn deny_chat_user<C, U>(chat: C, user: U) -> Self
    where
        C: Into<ChatId>,
        U: Into<UserId>,
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
