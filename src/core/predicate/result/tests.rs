use std::fmt;

use super::*;

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
