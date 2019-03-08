use dotenv::dotenv;
use env_logger;
use log;
use std::env;
use tgbot::dispatcher::{DispatcherBuilder, Handler, HandlerFuture, MessageHandler};
use tgbot::types::Message;
use tgbot::webhook;
use tgbot::Api;

struct LogMessageHandler;

impl MessageHandler<Api> for LogMessageHandler {
    fn handle(&mut self, _api: &Api, message: &Message) -> HandlerFuture {
        log::info!("got a message: {:?}\n", message);
        ().into()
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
    let dispatcher = DispatcherBuilder::new()
        .add_handler(Handler::message(LogMessageHandler))
        .build(api);
    webhook::run_server(([127, 0, 0, 1], 8080), "/", dispatcher);
}
