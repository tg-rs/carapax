use dotenv::dotenv;
use env_logger;
use log;
use std::env;
use tgbot::dispatcher::{Dispatcher, HandlerFuture, HandlerResult, MessageHandler};
use tgbot::types::Message;
use tgbot::webhook::{self, WebhookDispatcher};
use tgbot::Api;

struct Handler;

impl MessageHandler for Handler {
    fn handle(&mut self, _api: &Api, message: &Message) -> HandlerFuture {
        log::info!("got a message: {:?}\n", message);
        HandlerResult::Continue.into()
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
    let proxy = env::var("TGBOT_PROXY").ok();
    let api = match proxy {
        Some(proxy) => Api::with_proxy(token, &proxy),
        None => Api::create(token),
    }
    .expect("Failed to create API");
    let dispatcher = Dispatcher::new(api.clone()).add_message_handler(Handler);
    let update_handler = WebhookDispatcher::new(dispatcher);
    webhook::run_server(([127, 0, 0, 1], 8080), "/", update_handler);
}
