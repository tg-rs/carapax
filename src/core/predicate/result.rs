use crate::core::handler::HandlerResult;
use std::error::Error;

/// A predicate result
#[derive(Debug)]
pub enum PredicateResult {
    /// Decorated handler will run
    True,
    /// Decorated handler will not run
    ///
    /// `HandlerResult` allows to decide, will next handler run or not.
    False(HandlerResult),
}

impl From<bool> for PredicateResult {
    fn from(value: bool) -> Self {
        if value {
            PredicateResult::True
        } else {
            PredicateResult::False(HandlerResult::Ok)
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
            Err(err) => PredicateResult::False(HandlerResult::Err(Box::new(err))),
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
        assert!(matches!(false.into(), PredicateResult::False(HandlerResult::Ok)));
        assert!(matches!(Ok::<bool, ExampleError>(true).into(), PredicateResult::True));
        assert!(matches!(
            Ok::<bool, ExampleError>(false).into(),
            PredicateResult::False(HandlerResult::Ok)
        ));
        assert!(matches!(
            Err::<bool, ExampleError>(ExampleError).into(),
            PredicateResult::False(HandlerResult::Err(_))
        ));
    }
}
