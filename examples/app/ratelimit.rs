use carapax::{
    ratelimit::{
        nonzero, DirectRateLimitPredicate, Jitter, KeyChat, KeyChatUser, KeyUser, KeyedRateLimitPredicate, Quota,
    },
    Chain, PredicateExt,
};
use std::time::Duration;

pub fn setup(chain: Chain, strategy: &str) -> Chain {
    let quota = Quota::with_period(Duration::from_secs(5))
        .expect("Failed to create quota")
        .allow_burst(nonzero!(1u32));
    let jitter = Jitter::up_to(Duration::from_secs(5));
    let result = Chain::default();
    match strategy {
        "direct_discard" => result.add(chain.predicate(DirectRateLimitPredicate::discard(quota))),
        "direct_wait" => result.add(chain.predicate(DirectRateLimitPredicate::wait(quota))),
        "direct_wait_with_jitter" => {
            result.add(chain.predicate(DirectRateLimitPredicate::wait_with_jitter(quota, jitter)))
        }
        "keyed_discard" => result.add(chain.predicate(<KeyedRateLimitPredicate<KeyChat, _, _>>::discard(quota))),
        "keyed_wait" => result.add(chain.predicate(<KeyedRateLimitPredicate<KeyUser, _, _>>::wait(quota))),
        "keyed_wait_with_jitter" => result.add(chain.predicate(
            <KeyedRateLimitPredicate<KeyChatUser, _, _>>::wait_with_jitter(quota, jitter),
        )),
        key => panic!("Unknown ratelimit stragey: {}", key),
    }
}
