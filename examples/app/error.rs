use std::{error::Error, fmt};

use carapax::{api::ExecuteError, session::SessionError};

#[derive(Debug)]
pub enum AppError {
    Execute(ExecuteError),
    Session(SessionError),
}

impl fmt::Display for AppError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Execute(err) => write!(out, "Execute error: {}", err),
            AppError::Session(err) => write!(out, "Session error: {}", err),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Execute(err) => Some(err),
            AppError::Session(err) => Some(err),
        }
    }
}

impl From<ExecuteError> for AppError {
    fn from(err: ExecuteError) -> Self {
        AppError::Execute(err)
    }
}

impl From<SessionError> for AppError {
    fn from(err: SessionError) -> Self {
        AppError::Session(err)
    }
}
