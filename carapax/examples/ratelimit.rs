use carapax::{
    longpoll::LongPoll,
    ratelimit::{
        limit_all_chats, limit_all_users, nonzero, DirectRateLimitHandler, KeyedRateLimitHandler, RateLimitList,
    },
    types::{ChatId, Integer, Message, UserId},
    Api, Config, Dispatcher,
};
use dotenv::dotenv;
use std::{env, time::Duration};

async fn handle_message(message: Message) {
    log::info!("Got a new message: {:?}", message);
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let proxy = env::var("CARAPAX_PROXY").ok();
    let strategy = env::var("TGRS_RATE_LIMIT_STRATEGY").expect("TGRS_RATE_LIMIT_STRATEGY is not set");

    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }

    let api = Api::new(config).unwrap();

    // 1 update per 5 seconds
    let (capacity, interval) = (nonzero!(1u32), Duration::from_secs(5));

    // Allow update when key is missing
    let on_missing = true;

    let mut dispatcher = Dispatcher::new(api.clone());

    match strategy.as_str() {
        "direct" => {
            // Limit all updates
            dispatcher.add_handler(DirectRateLimitHandler::new(capacity, interval))
        }
        "all_users" => {
            // Limit updates per user ID for all users
            dispatcher.add_handler(KeyedRateLimitHandler::new(
                limit_all_chats,
                on_missing,
                capacity,
                interval,
            ))
        }
        "all_chats" => {
            // Limit updates per chat ID for all chats
            dispatcher.add_handler(KeyedRateLimitHandler::new(
                limit_all_users,
                on_missing,
                capacity,
                interval,
            ))
        }
        "list" => {
            // Limit updates for specific chat id or user id
            let user_id = env::var("TGRS_RATE_LIMIT_USER_ID").expect("TGRS_RATE_LIMIT_USER_ID is not set");
            let user_id = match user_id.parse::<Integer>() {
                Ok(user_id) => UserId::Id(user_id),
                Err(_) => UserId::Username(user_id),
            };
            let chat_id = env::var("TGRS_RATE_LIMIT_CHAT_ID").expect("TGRS_RATE_LIMIT_CHAT_ID is not set");
            let chat_id = match chat_id.parse::<Integer>() {
                Ok(chat_id) => ChatId::Id(chat_id),
                Err(_) => ChatId::Username(chat_id),
            };
            dispatcher.add_handler(KeyedRateLimitHandler::new(
                RateLimitList::default().with_user(user_id).with_chat(chat_id),
                on_missing,
                capacity,
                interval,
            ))
        }
        _ => panic!("Unknown rate limit strategy: {:?}", strategy),
    };

    dispatcher.add_handler(handle_message);

    LongPoll::new(api, dispatcher).run().await
}
