use carapax::{
    ratelimit::{
        nonzero, DirectRateLimitPredicate, Jitter, KeyChat, KeyChatUser, KeyUser, KeyedRateLimitPredicate, Quota,
    },
    Dispatcher, DispatcherBuilder, Predicate,
};
use std::time::Duration;

pub fn setup(dispatcher: Dispatcher, strategy: &str) -> Dispatcher {
    let quota = Quota::with_period(Duration::from_secs(5))
        .expect("Failed to create quota")
        .allow_burst(nonzero!(1u32));
    let jitter = Jitter::up_to(Duration::from_secs(5));
    let mut builder = DispatcherBuilder::default();
    match strategy {
        "direct_discard" => {
            builder.add_handler(Predicate::new(DirectRateLimitPredicate::discard(quota), dispatcher));
        }
        "direct_wait" => {
            builder.add_handler(Predicate::new(DirectRateLimitPredicate::wait(quota), dispatcher));
        }
        "direct_wait_with_jitter" => {
            builder.add_handler(Predicate::new(
                DirectRateLimitPredicate::wait_with_jitter(quota, jitter),
                dispatcher,
            ));
        }
        "keyed_discard" => {
            builder.add_handler(Predicate::new(
                <KeyedRateLimitPredicate<KeyChat, _, _>>::discard(quota),
                dispatcher,
            ));
        }
        "keyed_wait" => {
            builder.add_handler(Predicate::new(
                <KeyedRateLimitPredicate<KeyUser, _, _>>::wait(quota),
                dispatcher,
            ));
        }
        "keyed_wait_with_jitter" => {
            builder.add_handler(Predicate::new(
                <KeyedRateLimitPredicate<KeyChatUser, _, _>>::wait_with_jitter(quota, jitter),
                dispatcher,
            ));
        }
        key => panic!("Unknown ratelimit stragey: {}", key),
    }
    builder.build()
}
