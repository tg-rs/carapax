use seance::backend::SessionBackend;
use std::{
    convert::{TryFrom, TryInto},
    error::Error,
    fmt,
};
use tgbot::types::{Command, Integer, Message, Update};

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

    /// Returns a session by ID obtained from an Update/Message/Command
    ///
    /// Feel free to pass a reference to one of types mentioned above
    pub fn get_session<I>(&self, input: I) -> Result<Session<B>, I::Error>
    where
        I: TryInto<SessionId>,
    {
        Ok(self.inner.get_session(&input.try_into()?.0))
    }

    /// Returns a session by raw ID
    pub fn get_session_by_raw_id<I>(&self, id: I) -> Session<B>
    where
        I: AsRef<str>,
    {
        self.inner.get_session(id.as_ref())
    }
}

/// Session ID obtained from Update, Message, etc...
pub struct SessionId(String);

impl SessionId {
    /// Creates a new session ID
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique ID of chat
    /// * user_id - Unique ID of user in the chat
    pub fn new(chat_id: Integer, user_id: Integer) -> Self {
        Self(format!("{}-{}", chat_id, user_id))
    }
}

impl TryFrom<&Update> for SessionId {
    type Error = SessionIdError;

    fn try_from(update: &Update) -> Result<Self, Self::Error> {
        if let (Some(chat_id), Some(user_id)) = (update.get_chat_id(), update.get_user().map(|x| x.id)) {
            Ok(SessionId::new(chat_id, user_id))
        } else {
            Err(SessionIdError)
        }
    }
}

impl TryFrom<&Message> for SessionId {
    type Error = SessionIdError;

    fn try_from(message: &Message) -> Result<Self, Self::Error> {
        if let (chat_id, Some(user_id)) = (message.get_chat_id(), message.get_user().map(|x| x.id)) {
            Ok(SessionId::new(chat_id, user_id))
        } else {
            Err(SessionIdError)
        }
    }
}

impl TryFrom<&Command> for SessionId {
    type Error = SessionIdError;

    fn try_from(command: &Command) -> Result<Self, Self::Error> {
        SessionId::try_from(command.get_message())
    }
}

/// Session ID could not be created from update
///
/// This error happens when a received update
/// does not contain information about Chat ID and User ID.
///
/// Consider create a SessionId directly via SessionId::new
#[derive(Debug)]
pub struct SessionIdError;

impl Error for SessionIdError {}

impl fmt::Display for SessionIdError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "Could not obtain a session ID from update")
    }
}
