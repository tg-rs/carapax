use std::error::Error;

use crate::core::handler::HandlerError;

#[cfg(test)]
mod tests;

/// Represents a result of a predicate.
#[derive(Debug)]
pub enum PredicateResult {
    /// A decorated handler will be executed.
    True,
    /// A decorated handler was not executed.
    False,
    /// An error occurred during the predicate execution.
    Err(HandlerError),
}

impl From<bool> for PredicateResult {
    fn from(value: bool) -> Self {
        if value {
            PredicateResult::True
        } else {
            PredicateResult::False
        }
    }
}

impl<T, E> From<Result<T, E>> for PredicateResult
where
    T: Into<PredicateResult>,
    E: Error + Send + 'static,
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(value) => value.into(),
            Err(err) => PredicateResult::Err(HandlerError::new(err)),
        }
    }
}
