//! # Ratelimit
//!
//! Carapax provides a ratelimit support using [governor](https://crates.io/crates/governor) crate.
//!
//! There are two type of predicates: [`DirectRateLimitPredicate`] and [`KeyedRateLimitPredicate`].
//!
//! Direct is used when you need to apply ratelimit for all incoming updates.
//! Keyed - when you need to limit updates per chat and/or user.
//!
//! Once the limit is reached you can choose to either [discard](carapax::ratelimit::MethodDiscard) the updates,
//! or [wait](carapax::ratelimit::MethodWait) for the next available time slot.
//!
//! Both types of predicates can be used [with](Jitter)
//! or [without](carapax::ratelimit::NoJitter) jitter.
//!
//! Note that you need to enable the `ratelimit` feature in `Cargo.toml`.
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
        "direct_discard" => result.with(chain.with_predicate(DirectRateLimitPredicate::discard(quota))),
        "direct_wait" => result.with(chain.with_predicate(DirectRateLimitPredicate::wait(quota))),
        "direct_wait_with_jitter" => {
            result.with(chain.with_predicate(DirectRateLimitPredicate::wait_with_jitter(quota, jitter)))
        }
        "keyed_discard" => result.with(chain.with_predicate(<KeyedRateLimitPredicate<KeyChat, _, _>>::discard(quota))),
        "keyed_wait" => result.with(chain.with_predicate(<KeyedRateLimitPredicate<KeyUser, _, _>>::wait(quota))),
        "keyed_wait_with_jitter" => result.with(chain.with_predicate(
            <KeyedRateLimitPredicate<KeyChatUser, _, _>>::wait_with_jitter(quota, jitter),
        )),
        key => panic!("Unknown ratelimit strategy: {}", key),
    }
}
