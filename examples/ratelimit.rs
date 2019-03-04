use carapax::prelude::*;
use carapax::ratelimit::{nonzero, RateLimitMiddleware};
use dotenv::dotenv;
use env_logger;
use futures::Future;
use log;
use std::env;

struct Handler;

impl MessageHandler for Handler {
    fn handle(&mut self, api: &Api, message: &Message) -> HandlerFuture {
        log::info!("got a message: {:?}\n", message);
        if let Some(text) = message.get_text() {
            let chat_id = message.get_chat_id();
            let method = SendMessage::new(chat_id, text.data.clone());
            return HandlerFuture::new(api.execute(&method).then(|x| {
                log::info!("sendMessage result: {:?}\n", x);
                Ok(HandlerResult::Continue)
            }));
        }
        HandlerResult::Continue.into()
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let proxy = env::var("CARAPAX_PROXY").ok();
    let api = match proxy {
        Some(proxy) => Api::with_proxy(token, &proxy),
        None => Api::create(token),
    }
    .expect("Failed to create API");
    Dispatcher::new(api.clone())
        // take 1 update per 5 seconds
        .add_middleware(RateLimitMiddleware::direct(nonzero!(1u32), 5))
        .add_message_handler(Handler)
        .start_polling();
}
