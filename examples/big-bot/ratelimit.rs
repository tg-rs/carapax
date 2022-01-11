use crate::{get_env, util::Module};
use carapax::{
    ratelimit::{
        nonzero, DirectRateLimitPredicate, Jitter, KeyChat, KeyChatUser, KeyUser, KeyedRateLimitPredicate, Quota,
    },
    types::Update,
    Dispatcher, PredicateExt,
};
use std::time::Duration;

pub struct RateLimitModule;

impl Module for RateLimitModule {
    fn add_handlers(&self, dispatcher: &mut Dispatcher) {
        let quota = Quota::with_period(Duration::from_secs(5))
            .expect("Failed to create quota")
            .allow_burst(nonzero!(1u32));
        let jitter = Jitter::up_to(Duration::from_secs(5));

        let strategy = get_env("CARAPAX_RATE_LIMIT_STRATEGY");
        match strategy.as_str() {
            "direct_discard" => {
                dispatcher.add_handler(update_handler.predicate(DirectRateLimitPredicate::discard(quota)))
            }
            "direct_wait" => dispatcher.add_handler(update_handler.predicate(DirectRateLimitPredicate::wait(quota))),
            "direct_wait_with_jitter" => dispatcher
                .add_handler(update_handler.predicate(DirectRateLimitPredicate::wait_with_jitter(quota, jitter))),
            "keyed_discard" => dispatcher
                .add_handler(update_handler.predicate(<KeyedRateLimitPredicate<KeyChat, _, _>>::discard(quota))),
            "keyed_wait" => {
                dispatcher.add_handler(update_handler.predicate(<KeyedRateLimitPredicate<KeyUser, _, _>>::wait(quota)))
            }
            "keyed_wait_with_jitter" => {
                dispatcher.add_handler(update_handler.predicate(
                    <KeyedRateLimitPredicate<KeyChatUser, _, _>>::wait_with_jitter(quota, jitter),
                ))
            }
            key => panic!("Unknown ratelimit strategy: {}", key),
        };
    }
}

async fn update_handler(update: Update) {
    log::info!("Got an update: {:?}", update)
}
