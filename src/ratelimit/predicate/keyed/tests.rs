use std::time::Duration;

use crate::ratelimit::key::{KeyChat, KeyChatUser, KeyUser};

use super::*;

#[tokio::test]
async fn keyed() {
    macro_rules! test_key {
        ($key_type:ty, $first_key:expr, $second_key:expr) => {
            // discard
            let handler: KeyedRateLimitPredicate<$key_type, _, _> =
                KeyedRateLimitPredicate::discard(Quota::per_minute(nonzero!(1u32)));
            assert!(
                matches!(handler.handle($first_key).await, PredicateResult::True),
                "[keyed/discard] handle({:?}) -> continue",
                $first_key
            );
            assert!(
                matches!(handler.handle($first_key).await, PredicateResult::False),
                "[keyed/discard] handle({:?}) -> stop",
                $first_key
            );
            assert!(
                matches!(handler.handle($second_key).await, PredicateResult::True),
                "[keyed/discard] handle({:?}) -> continue",
                $second_key
            );
            assert!(
                matches!(handler.handle($second_key).await, PredicateResult::False),
                "[keyed/discard] handle({:?}) -> stop",
                $second_key
            );

            // discard a specific key only
            let handler: KeyedRateLimitPredicate<$key_type, _, _> =
                KeyedRateLimitPredicate::discard(Quota::per_minute(nonzero!(1u32))).with_key($second_key);
            assert!(
                matches!(handler.handle($first_key).await, PredicateResult::True),
                "[keyed/discard/with_key] handle({:?}) -> continue",
                $first_key,
            );
            assert!(
                matches!(handler.handle($first_key).await, PredicateResult::True),
                "[keyed/discard/with_key] handle({:?}) -> continue",
                $first_key,
            );
            assert!(
                matches!(handler.handle($second_key).await, PredicateResult::True),
                "[keyed/discard/with_key] handle({:?}) -> continue",
                $first_key,
            );
            assert!(
                matches!(handler.handle($second_key).await, PredicateResult::False),
                "[keyed/discard/with_key] handle({:?}) -> stop",
                $first_key,
            );

            // wait
            let handler: KeyedRateLimitPredicate<$key_type, _, _> = KeyedRateLimitPredicate::wait(
                Quota::with_period(Duration::from_millis(100))
                    .unwrap()
                    .allow_burst(nonzero!(1u32)),
            );
            for key in [$first_key, $first_key, $second_key, $second_key] {
                assert!(
                    matches!(handler.handle(key).await, PredicateResult::True),
                    "[keyed/wait] handle({:?}) -> continue",
                    key
                );
            }

            // wait a specific key only
            let handler: KeyedRateLimitPredicate<$key_type, _, _> = KeyedRateLimitPredicate::wait(
                Quota::with_period(Duration::from_millis(100))
                    .unwrap()
                    .allow_burst(nonzero!(1u32)),
            )
            .with_key($second_key);
            for key in [$first_key, $first_key, $second_key, $second_key] {
                assert!(
                    matches!(handler.handle(key).await, PredicateResult::True),
                    "[keyed/wait/with_key] handle({:?}) -> continue",
                    key
                );
            }

            // wait with jitter
            let handler: KeyedRateLimitPredicate<$key_type, _, _> = KeyedRateLimitPredicate::wait_with_jitter(
                Quota::with_period(Duration::from_millis(100))
                    .unwrap()
                    .allow_burst(nonzero!(1u32)),
                Jitter::new(Duration::from_millis(0), Duration::from_millis(100)),
            );
            for key in [$first_key, $first_key, $second_key, $second_key] {
                assert!(
                    matches!(handler.handle(key).await, PredicateResult::True),
                    "[keyed/wait_with_jitter] handle({:?}) -> continue",
                    key
                );
            }

            // wait with jitter a specific key only
            let handler: KeyedRateLimitPredicate<$key_type, _, _> = KeyedRateLimitPredicate::wait_with_jitter(
                Quota::with_period(Duration::from_millis(100))
                    .unwrap()
                    .allow_burst(nonzero!(1u32)),
                Jitter::new(Duration::from_millis(0), Duration::from_millis(100)),
            )
            .with_key($second_key);
            for key in [$first_key, $first_key, $second_key, $second_key] {
                assert!(
                    matches!(handler.handle(key).await, PredicateResult::True),
                    "[keyed/wait_with_jitter/with_key] handle({:?}) -> continue",
                    key
                );
            }
        };
    }
    let chat_1 = KeyChat::from(1);
    let chat_2 = KeyChat::from(2);
    test_key!(KeyChat, chat_1, chat_2);
    let user_1 = KeyUser::from(1);
    let user_2 = KeyUser::from(2);
    test_key!(KeyUser, user_1, user_2);
    let chat_user_1 = KeyChatUser::from((1, 1));
    let chat_user_2 = KeyChatUser::from((1, 2));
    test_key!(KeyChatUser, chat_user_1, chat_user_2);
}
