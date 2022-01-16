use carapax::{
    ratelimit::{
        nonzero, DirectRateLimitPredicate, Jitter, KeyChat, KeyChatUser, KeyUser, KeyedRateLimitPredicate, Quota,
    },
    Dispatcher, DispatcherBuilder, PredicateExt,
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
            builder.add_handler(dispatcher.predicate(DirectRateLimitPredicate::discard(quota)));
        }
        "direct_wait" => {
            builder.add_handler(dispatcher.predicate(DirectRateLimitPredicate::wait(quota)));
        }
        "direct_wait_with_jitter" => {
            builder.add_handler(dispatcher.predicate(DirectRateLimitPredicate::wait_with_jitter(quota, jitter)));
        }
        "keyed_discard" => {
            builder.add_handler(dispatcher.predicate(<KeyedRateLimitPredicate<KeyChat, _, _>>::discard(quota)));
        }
        "keyed_wait" => {
            builder.add_handler(dispatcher.predicate(<KeyedRateLimitPredicate<KeyUser, _, _>>::wait(quota)));
        }
        "keyed_wait_with_jitter" => {
            builder.add_handler(
                dispatcher.predicate(<KeyedRateLimitPredicate<KeyChatUser, _, _>>::wait_with_jitter(
                    quota, jitter,
                )),
            );
        }
        key => panic!("Unknown ratelimit stragey: {}", key),
    }
    builder.build()
}
