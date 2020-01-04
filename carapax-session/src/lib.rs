#![warn(missing_docs)]
//! A session utilities for carapax
use carapax::{
    types::{Message, Update},
    Command,
};
use seance::backend::SessionBackend;

pub use seance::{
    backend, Session, SessionCollector, SessionCollectorHandle, SessionError, SessionManager as BaseSessionManager,
};

/// A session manager
#[derive(Clone)]
pub struct SessionManager<B> {
    inner: BaseSessionManager<B>,
}

impl<B> SessionManager<B>
where
    B: SessionBackend,
{
    /// Creates a new manager
    ///
    /// # Arguments
    ///
    /// * backend - A session store backend
    pub fn new(backend: B) -> Self {
        Self {
            inner: BaseSessionManager::new(backend),
        }
    }

    /// Returns a session for an update/message/command
    pub fn get_session<I>(&self, input: I) -> Session<B>
    where
        I: Into<SessionId>,
    {
        self.inner.get_session(&input.into().0)
    }

    /// Returns a session by ID
    pub fn get_session_by_id<I>(&self, id: I) -> Session<B>
    where
        I: AsRef<str>,
    {
        self.inner.get_session(id.as_ref())
    }
}

/// Session ID obtained from Update, Message, etc...
pub struct SessionId(String);

impl From<&Update> for SessionId {
    fn from(update: &Update) -> SessionId {
        let (chat_id, user_id) = match (update.get_chat_id(), update.get_user().map(|x| x.id)) {
            (Some(chat_id), Some(user_id)) => (chat_id, user_id),
            (Some(chat_id), None) => (chat_id, chat_id),
            (None, Some(user_id)) => (user_id, user_id),
            (None, None) => unreachable!(), // There is always chat_id or user_id
        };
        SessionId(format!("{}-{}", chat_id, user_id))
    }
}

impl From<&Message> for SessionId {
    fn from(message: &Message) -> SessionId {
        let (chat_id, user_id) = match (message.get_chat_id(), message.get_user().map(|x| x.id)) {
            (chat_id, Some(user_id)) => (chat_id, user_id),
            (chat_id, None) => (chat_id, chat_id),
        };
        SessionId(format!("{}-{}", chat_id, user_id))
    }
}
impl From<&Command> for SessionId {
    fn from(command: &Command) -> SessionId {
        SessionId::from(command.get_message())
    }
}
