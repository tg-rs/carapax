use crate::core::handler::HandlerError;
use std::error::Error;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    #[derive(Debug)]
    struct ExampleError;

    impl Error for ExampleError {}

    impl fmt::Display for ExampleError {
        fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(out, "Example error")
        }
    }

    #[test]
    fn convert_result() {
        assert!(matches!(true.into(), PredicateResult::True));
        assert!(matches!(false.into(), PredicateResult::False));
        assert!(matches!(Ok::<bool, ExampleError>(true).into(), PredicateResult::True));
        assert!(matches!(Ok::<bool, ExampleError>(false).into(), PredicateResult::False));
        assert!(matches!(
            Err::<bool, ExampleError>(ExampleError).into(),
            PredicateResult::Err(_)
        ));
    }
}
