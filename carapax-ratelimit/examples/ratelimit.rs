use carapax::prelude::*;
use carapax_ratelimit::{
    limit_all_chats, limit_all_users, nonzero, DirectRateLimitHandler, KeyedRateLimitHandler, RateLimitList,
};
use dotenv::dotenv;
use env_logger;
use std::{env, time::Duration};

fn handle_message(_context: &mut Context, message: &Message) -> HandlerFuture {
    log::info!("Got a new message: {:?}", message);
    HandlerResult::Continue.into()
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();
    let strategy = env::var("TGRS_RATE_LIMIT_STRATEGY").expect("TGRS_RATE_LIMIT_STRATEGY is not set");

    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy);
    }

    let api = Api::new(config).unwrap();

    // 1 update per 5 seconds
    let (capacity, interval) = (nonzero!(1u32), Duration::from_secs(5));

    // Allow update when key is missing
    let on_missing = true;

    let rate_limit_handler = match strategy.as_str() {
        "direct" => {
            // Limit all updates
            Handler::update(DirectRateLimitHandler::new(capacity, interval))
        }
        "all_users" => {
            // Limit updates per user ID for all users
            Handler::update(KeyedRateLimitHandler::new(
                limit_all_chats,
                on_missing,
                capacity,
                interval,
            ))
        }
        "all_chats" => {
            // Limit updates per chat ID for all chats
            Handler::update(KeyedRateLimitHandler::new(
                limit_all_users,
                on_missing,
                capacity,
                interval,
            ))
        }
        "list" => {
            // Limit updates for specific chat id or user id
            let user_id = env::var("TG_RS_RATE_LIMIT_USER_ID").expect("TG_RS_RATE_LIMIT_USER_ID is not set");
            let user_id = match user_id.parse::<Integer>() {
                Ok(user_id) => UserId::Id(user_id),
                Err(_) => UserId::Username(user_id),
            };
            let chat_id = env::var("TG_RS_RATE_LIMIT_CHAT_ID").expect("TG_RS_RATE_LIMIT_CHAT_ID is not set");
            let chat_id = match chat_id.parse::<Integer>() {
                Ok(chat_id) => ChatId::Id(chat_id),
                Err(_) => ChatId::Username(chat_id),
            };
            Handler::update(KeyedRateLimitHandler::new(
                RateLimitList::default().with_user(user_id).with_chat(chat_id),
                on_missing,
                capacity,
                interval,
            ))
        }
        _ => panic!("Unknown rate limit strategy: {:?}", strategy),
    };

    tokio::run(
        App::new()
            .add_handler(rate_limit_handler)
            .add_handler(Handler::message(handle_message))
            .run(api.clone(), UpdateMethod::poll(UpdatesStream::new(api))),
    )
}
