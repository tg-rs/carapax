use crate::{
    core::{HandlerInput, TryFromInput},
    types::Integer,
};
use futures_util::future::{ok, BoxFuture, Ready};
use seance::backend::SessionBackend;
use std::{convert::Infallible, error::Error, fmt};

pub use seance::{backend, Session, SessionCollector, SessionCollectorHandle, SessionError, SessionManager};

impl<B> TryFromInput for Session<B>
where
    B: SessionBackend + Send + 'static,
{
    type Error = CreateSessionError;
    type Future = BoxFuture<'static, Result<Option<Self>, Self::Error>>;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        Box::pin(async move {
            match input.context.get::<SessionManager<B>>() {
                Some(manager) => match SessionId::try_from_input(input.clone()).await {
                    Ok(Some(session_id)) => {
                        let session = manager.get_session(session_id.0);
                        Ok(Some(session))
                    }
                    Ok(None) => Err(CreateSessionError::SessionIdNotFound),
                    Err(_) => unreachable!(),
                },
                None => Err(CreateSessionError::ManagerNotFound),
            }
        })
    }
}

/// Represents an ID of a session
pub struct SessionId(String);

impl SessionId {
    /// Creates a new SessionID
    ///
    /// # Arguments
    ///
    /// * chat_id - ID of a chat
    /// * user_id - ID of a user
    pub fn new(chat_id: Integer, user_id: Integer) -> Self {
        Self(format!("{}-{}", chat_id, user_id))
    }
}

impl TryFromInput for SessionId {
    type Error = Infallible;
    type Future = Ready<Result<Option<Self>, Self::Error>>;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        let chat_id = input.update.get_chat_id();
        let user_id = input.update.get_user_id();
        ok(if let (Some(chat_id), Some(user_id)) = (chat_id, user_id) {
            Some(SessionId::new(chat_id, user_id))
        } else {
            None
        })
    }
}

/// An error when creating a session
#[derive(Debug)]
pub enum CreateSessionError {
    /// Session manager not found in context
    ManagerNotFound,
    /// Could not create session ID
    ///
    /// Chat or user ID is missing
    SessionIdNotFound,
}

impl fmt::Display for CreateSessionError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::CreateSessionError::*;
        write!(
            out,
            "{}",
            match self {
                ManagerNotFound => "Session manager not found in context",
                SessionIdNotFound => "Could not create session ID: chat or user ID is missing",
            }
        )
    }
}

impl Error for CreateSessionError {}
