use std::error::Error;

use crate::core::handler::HandlerError;

#[cfg(test)]
mod tests;

/// A predicate result
#[derive(Debug)]
pub enum PredicateResult {
    /// Decorated handler will run
    True,
    /// Decorated handler was not run
    False,
    /// An error has occurred in predicate
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
