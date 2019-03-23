use carapax::prelude::*;
use carapax_ratelimit::{nonzero, RateLimitHandler, RateLimitKey};
use dotenv::dotenv;
use env_logger;
use std::env;

fn handle_message(_context: &mut Context, message: &Message) -> HandlerFuture {
    log::info!("Got a new message: {:?}", message);
    HandlerResult::Continue.into()
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();

    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy);
    }

    let api = Api::new(config).unwrap();

    let capacity = nonzero!(1u32); // updates
    let interval = 5; // seconds

    // Keyed
    let key = RateLimitKey::Chat; // Limit per chat_id, use User for user_id
    let on_missing = true; // Allow update when chat_id or user_id is missing

    tokio::run(
        App::new()
            .add_handler(Handler::update(RateLimitHandler::direct(capacity, interval)))
            .add_handler(Handler::update(RateLimitHandler::keyed(
                key, capacity, interval, on_missing,
            )))
            .add_handler(Handler::message(handle_message))
            .run(api.clone(), UpdateMethod::poll(UpdatesStream::new(api))),
    )
}
