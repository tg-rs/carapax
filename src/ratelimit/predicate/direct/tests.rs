use std::time::Duration;

use super::*;

#[tokio::test]
async fn direct() {
    let handler = DirectRateLimitPredicate::discard(Quota::per_minute(nonzero!(1u32)));
    assert!(
        matches!(handler.handle(()).await, PredicateResult::True),
        "[direct/discard/1] true"
    );
    assert!(
        matches!(handler.handle(()).await, PredicateResult::False),
        "[direct/discard/2] false"
    );

    let handler = DirectRateLimitPredicate::wait(
        Quota::with_period(Duration::from_millis(100))
            .unwrap()
            .allow_burst(nonzero!(1u32)),
    );
    assert!(
        matches!(handler.handle(()).await, PredicateResult::True),
        "[direct/wait/1] continue"
    );
    assert!(
        matches!(handler.handle(()).await, PredicateResult::True),
        "[direct/wait/2] continue"
    );

    let handler = DirectRateLimitPredicate::wait_with_jitter(
        Quota::with_period(Duration::from_millis(100))
            .unwrap()
            .allow_burst(nonzero!(1u32)),
        Jitter::new(Duration::from_secs(0), Duration::from_millis(100)),
    );
    assert!(
        matches!(handler.handle(()).await, PredicateResult::True),
        "[direct/wait_with_jitter/1] continue"
    );
    assert!(
        matches!(handler.handle(()).await, PredicateResult::True),
        "[direct/wait_with_jitter/2] continue"
    );
}
