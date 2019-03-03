use super::{Middleware, MiddlewareFuture, MiddlewareResult};
use crate::api::Api;
use crate::types::{Integer, Update};

struct Rule {
    principal: Principal,
    is_granted: bool,
}

impl Rule {
    fn accepts(&self, update: &Update) -> bool {
        match self.principal {
            Principal::UserId(user_id) => update.get_user().map(|u| u.id == user_id),
            Principal::Username(ref username) => update.get_user().and_then(|u| {
                if let Some(ref x) = u.username {
                    Some(x == username)
                } else {
                    None
                }
            }),
            Principal::ChatId(chat_id) => update.get_chat_id().map(|x| x == chat_id),
            Principal::ChatUsername(ref chat_username) => {
                update.get_chat_username().map(|x| x == chat_username)
            }
            Principal::All => return true,
        }
        .unwrap_or(false)
    }
}

#[derive(Debug)]
enum Principal {
    All,
    UserId(Integer),
    Username(String),
    ChatId(Integer),
    ChatUsername(String),
}

/// Access control middleware
///
/// Helps to deny/allow updates from specific user/chat
///
/// If there are no rules matching an update, access will be forbidden.
#[derive(Default)]
pub struct AccessMiddleware {
    rules: Vec<Rule>,
}

impl AccessMiddleware {
    /// Allows all updates
    pub fn allow_all(mut self) -> Self {
        self.rules.push(Rule {
            is_granted: true,
            principal: Principal::All,
        });
        self
    }

    /// Denies all updates
    pub fn deny_all(mut self) -> Self {
        self.rules.push(Rule {
            is_granted: false,
            principal: Principal::All,
        });
        self
    }

    /// Allows updates from a user with ID
    pub fn allow_user_id(mut self, user_id: Integer) -> Self {
        self.rules.push(Rule {
            is_granted: true,
            principal: Principal::UserId(user_id),
        });
        self
    }

    /// Denies updates from a user with ID
    pub fn deny_user_id(mut self, user_id: Integer) -> Self {
        self.rules.push(Rule {
            is_granted: false,
            principal: Principal::UserId(user_id),
        });
        self
    }

    /// Allows updates from a user with @username
    pub fn allow_username<S: Into<String>>(mut self, username: S) -> Self {
        self.rules.push(Rule {
            is_granted: true,
            principal: Principal::Username(username.into()),
        });
        self
    }

    /// Denies updates from a user with @username
    pub fn deny_username<S: Into<String>>(mut self, username: S) -> Self {
        self.rules.push(Rule {
            is_granted: false,
            principal: Principal::Username(username.into()),
        });
        self
    }

    /// Allows updates from a chat with ID
    pub fn allow_chat_id(mut self, chat_id: Integer) -> Self {
        self.rules.push(Rule {
            is_granted: true,
            principal: Principal::ChatId(chat_id),
        });
        self
    }

    /// Denies updates from a chat with ID
    pub fn deny_chat_id(mut self, chat_id: Integer) -> Self {
        self.rules.push(Rule {
            is_granted: false,
            principal: Principal::ChatId(chat_id),
        });
        self
    }

    /// Allows updates from a chat with @username
    pub fn allow_chat_username<S: Into<String>>(mut self, username: S) -> Self {
        self.rules.push(Rule {
            is_granted: true,
            principal: Principal::ChatUsername(username.into()),
        });
        self
    }

    /// Denies updates from a chat with @username
    pub fn deny_chat_username<S: Into<String>>(mut self, username: S) -> Self {
        self.rules.push(Rule {
            is_granted: false,
            principal: Principal::ChatUsername(username.into()),
        });
        self
    }
}

impl Middleware for AccessMiddleware {
    fn before(&mut self, _api: &Api, update: &Update) -> MiddlewareFuture {
        for rule in &self.rules {
            if rule.accepts(&update) {
                return if rule.is_granted {
                    MiddlewareResult::Continue
                } else {
                    log::info!("Access denied for principal: {:?}", rule.principal);
                    MiddlewareResult::Stop
                }
                .into();
            }
        }
        log::info!("Access denied by default, no rules found");
        MiddlewareResult::Stop.into()
    }
}
