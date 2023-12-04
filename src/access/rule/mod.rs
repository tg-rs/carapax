use crate::{
    access::principal::Principal,
    types::{ChatId, Update, UserId},
};

#[cfg(test)]
mod tests;

/// Represents an access rule containing information about a principal and access grant status.
#[derive(Debug)]
pub struct AccessRule {
    principal: Principal,
    is_granted: bool,
}

impl AccessRule {
    /// Creates a new `AccessRule`.
    ///
    /// # Arguments
    ///
    /// * `principal` - A principal.
    /// * `is_granted` - A flag indicating whether access is granted (`true`) or denied (`false`).
    pub fn new<T>(principal: T, is_granted: bool) -> Self
    where
        T: Into<Principal>,
    {
        AccessRule {
            principal: principal.into(),
            is_granted,
        }
    }

    /// Creates a new `AccessRule` with granted access for a principal.
    ///
    /// # Arguments
    ///
    /// * `value` - The principal.
    pub fn allow<T>(value: T) -> Self
    where
        T: Into<Principal>,
    {
        Self::new(value, true)
    }

    /// Creates a new `AccessRule` with forbidden access for a principal.
    ///
    /// # Arguments
    ///
    /// * `value` - The principal.
    pub fn deny<T>(value: T) -> Self
    where
        T: Into<Principal>,
    {
        Self::new(value, false)
    }

    /// Creates a new `AccessRule` with granted access for all principals.
    pub fn allow_all() -> Self {
        Self::allow(Principal::All)
    }

    /// Creates a new `AccessRule` with denied access for all principals.
    pub fn deny_all() -> Self {
        Self::deny(Principal::All)
    }

    /// Creates a new `AccessRule` with granted access for a specific user.
    ///
    /// # Arguments
    ///
    /// * `value` - Identifier of the user.
    pub fn allow_user<T>(value: T) -> Self
    where
        T: Into<UserId>,
    {
        Self::allow(value.into())
    }

    /// Creates a new `AccessRule` with denied access for a specific user.
    ///
    /// # Arguments
    ///
    /// * `value` - Identifier of the user.
    pub fn deny_user<T>(value: T) -> Self
    where
        T: Into<UserId>,
    {
        Self::deny(value.into())
    }

    /// Creates a new `AccessRule` with granted access for a specific chat.
    ///
    /// # Arguments
    ///
    /// * `value` - Identifier of the chat.
    pub fn allow_chat<T>(value: T) -> Self
    where
        T: Into<ChatId>,
    {
        Self::allow(value.into())
    }

    /// Creates a new `AccessRule` with denied access for a specific chat.
    ///
    /// # Arguments
    ///
    /// * `value` - Identifier of the chat.
    pub fn deny_chat<T>(value: T) -> Self
    where
        T: Into<ChatId>,
    {
        Self::deny(value.into())
    }

    /// Creates a new `AccessRule` with granted access for a user within a specific chat.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Identifier of the chat.
    /// * `user_id` - Identifier of the user.
    pub fn allow_chat_user<A, B>(chat_id: A, user_id: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<UserId>,
    {
        Self::allow((chat_id.into(), user_id.into()))
    }

    /// Creates a new `AccessRule` with denied access for a user within a specific chat.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Identifier of the chat.
    /// * `user_id` - Identifier of the user.
    pub fn deny_chat_user<A, B>(chat_id: A, user_id: B) -> Self
    where
        A: Into<ChatId>,
        B: Into<UserId>,
    {
        Self::deny((chat_id.into(), user_id.into()))
    }

    /// Indicates whether the `AccessRule` accepts an [`Update`].
    ///
    /// # Arguments
    ///
    /// * `update` - The update to be evaluated by the access rule.
    ///
    /// Returns `true` if `AccessRule` accepts an update and `false` otherwise.
    pub fn accepts(&self, update: &Update) -> bool {
        self.principal.accepts(update)
    }

    /// Indicates whether access is granted by the rule.
    ///
    /// Returns `true` if access is granted and `false` otherwise.
    pub fn is_granted(&self) -> bool {
        self.is_granted
    }
}
