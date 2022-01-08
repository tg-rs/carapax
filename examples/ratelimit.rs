use carapax::{
    ratelimit::{
        nonzero, DirectRateLimitPredicate, Jitter, KeyChat, KeyChatUser, KeyUser, KeyedRateLimitPredicate, Quota,
    },
    types::Update,
    PredicateExt,
};
use dotenv::dotenv;
use std::time::Duration;

async fn update_handler(update: Update) {
    log::info!("Got an update: {:?}", update)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let quota = Quota::with_period(Duration::from_secs(5))
        .expect("Failed to create quota")
        .allow_burst(nonzero!(1u32));
    let jitter = Jitter::up_to(Duration::from_secs(5));
    let strategy = helper::get_env("CARAPAX_RATE_LIMIT_STRATEGY");
    match strategy.as_str() {
        "direct_discard" => helper::run(update_handler.predicate(DirectRateLimitPredicate::discard(quota))).await,
        "direct_wait" => helper::run(update_handler.predicate(DirectRateLimitPredicate::wait(quota))).await,
        "direct_wait_with_jitter" => {
            helper::run(update_handler.predicate(DirectRateLimitPredicate::wait_with_jitter(quota, jitter))).await
        }
        "keyed_discard" => {
            helper::run(update_handler.predicate(<KeyedRateLimitPredicate<KeyChat, _, _>>::discard(quota))).await
        }
        "keyed_wait" => {
            helper::run(update_handler.predicate(<KeyedRateLimitPredicate<KeyUser, _, _>>::wait(quota))).await
        }
        "keyed_wait_with_jitter" => {
            helper::run(
                update_handler.predicate(<KeyedRateLimitPredicate<KeyChatUser, _, _>>::wait_with_jitter(
                    quota, jitter,
                )),
            )
            .await
        }
        key => panic!("Unknown ratelimit stragey: {}", key),
    }
}
