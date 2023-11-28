use std::time::Duration;

use carapax::{
    ratelimit::{
        nonzero, DirectRateLimitPredicate, Jitter, KeyChat, KeyChatUser, KeyUser, KeyedRateLimitPredicate, Quota,
    },
    Chain, PredicateExt,
};

pub fn setup(chain: Chain, strategy: &str) -> Chain {
    let quota = Quota::with_period(Duration::from_secs(5))
        .expect("Failed to create quota")
        .allow_burst(nonzero!(1u32));
    let jitter = Jitter::up_to(Duration::from_secs(5));
    let result = Chain::once();
    match strategy {
        "direct_discard" => result.with(chain.predicate(DirectRateLimitPredicate::discard(quota))),
        "direct_wait" => result.with(chain.predicate(DirectRateLimitPredicate::wait(quota))),
        "direct_wait_with_jitter" => {
            result.with(chain.predicate(DirectRateLimitPredicate::wait_with_jitter(quota, jitter)))
        }
        "keyed_discard" => result.with(chain.predicate(<KeyedRateLimitPredicate<KeyChat, _, _>>::discard(quota))),
        "keyed_wait" => result.with(chain.predicate(<KeyedRateLimitPredicate<KeyUser, _, _>>::wait(quota))),
        "keyed_wait_with_jitter" => result.with(chain.predicate(
            <KeyedRateLimitPredicate<KeyChatUser, _, _>>::wait_with_jitter(quota, jitter),
        )),
        key => panic!("Unknown ratelimit strategy: {}", key),
    }
}
