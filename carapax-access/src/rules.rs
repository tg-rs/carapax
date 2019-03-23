use carapax::prelude::*;

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

    /// Is access granted
    pub fn is_granted(&self) -> bool {
        self.is_granted
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
            Principal::ChatUser(chat_principal, user_principal) => {
                chat_principal.accepts(&update) && user_principal.accepts(&update)
            }
            Principal::All => true,
        }
    }
}
