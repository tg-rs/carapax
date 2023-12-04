use std::{error::Error, fmt};

use seance::SessionError;

use crate::session::CreateSessionError;

/// An error when processing dialogue.
#[derive(Debug)]
pub enum DialogueError {
    /// Failed to obtain input for the dialogue handler.
    ConvertHandlerInput,
    /// Failed to create a session.
    CreateSession(CreateSessionError),
    /// Failed to load the dialogue state.
    LoadState(SessionError),
}

impl From<CreateSessionError> for DialogueError {
    fn from(err: CreateSessionError) -> Self {
        DialogueError::CreateSession(err)
    }
}

impl fmt::Display for DialogueError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::DialogueError::*;
        match self {
            ConvertHandlerInput => write!(out, "Could not obtain input for dialogue handler"),
            CreateSession(err) => write!(out, "{}", err),
            LoadState(err) => write!(out, "Failed to load dialogue state: {}", err),
        }
    }
}

impl Error for DialogueError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::DialogueError::*;
        match self {
            ConvertHandlerInput => None,
            CreateSession(err) => Some(err),
            LoadState(err) => Some(err),
        }
    }
}
