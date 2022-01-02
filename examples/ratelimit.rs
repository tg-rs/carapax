use carapax::{
    longpoll::LongPoll,
    ratelimit::{
        nonzero, DirectRateLimitPredicate, Jitter, KeyChat, KeyChatUser, KeyUser, KeyedRateLimitPredicate, Quota,
    },
    types::Update,
    Api, Config, Context, Dispatcher, PredicateExt,
};
use dotenv::dotenv;
use std::{env, time::Duration};

async fn update_handler(update: Update) {
    log::info!("Got an update: {:?}", update)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let proxy = env::var("CARAPAX_PROXY").ok();
    let strategy = env::var("CARAPAX_RATE_LIMIT_STRATEGY").expect("CARAPAX_RATE_LIMIT_STRATEGY is not set");
    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }
    let api = Api::new(config).expect("Failed to create API");
    let mut dispatcher = Dispatcher::new(Context::default());
    let quota = Quota::with_period(Duration::from_secs(5))
        .expect("Failed to create quota")
        .allow_burst(nonzero!(1u32));
    let jitter = Jitter::up_to(Duration::from_secs(5));
    match strategy.as_str() {
        "direct_discard" => {
            dispatcher.add_handler(update_handler.predicate(DirectRateLimitPredicate::discard(quota)));
        }
        "direct_wait" => {
            dispatcher.add_handler(update_handler.predicate(DirectRateLimitPredicate::wait(quota)));
        }
        "direct_wait_with_jitter" => {
            dispatcher.add_handler(update_handler.predicate(DirectRateLimitPredicate::wait_with_jitter(quota, jitter)));
        }
        "keyed_discard" => {
            dispatcher.add_handler(update_handler.predicate(<KeyedRateLimitPredicate<KeyChat, _, _>>::discard(quota)));
        }
        "keyed_wait" => {
            dispatcher.add_handler(update_handler.predicate(<KeyedRateLimitPredicate<KeyUser, _, _>>::wait(quota)));
        }
        "keyed_wait_with_jitter" => {
            dispatcher.add_handler(update_handler.predicate(
                <KeyedRateLimitPredicate<KeyChatUser, _, _>>::wait_with_jitter(quota, jitter),
            ));
        }
        key => panic!("Unknown ratelimit stragey: {}", key),
    }

    LongPoll::new(api, dispatcher).run().await
}
