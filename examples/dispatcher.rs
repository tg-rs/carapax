use dotenv::dotenv;
use env_logger;
use futures::Future;
use log;
use std::env;
use tgbot::dispatcher::{DispatcherBuilder, Handler, HandlerFuture, MessageHandler};
use tgbot::methods::SendMessage;
use tgbot::types::Message;
use tgbot::Api;

struct EchoHandler;

impl MessageHandler<Api> for EchoHandler {
    fn handle(&mut self, api: &Api, message: &Message) -> HandlerFuture {
        log::info!("got a message: {:?}\n", message);
        if let Some(text) = message.get_text() {
            let chat_id = message.get_chat_id();
            let method = SendMessage::new(chat_id, text.data.clone());
            return HandlerFuture::new(api.execute(&method).then(|x| {
                log::info!("sendMessage result: {:?}\n", x);
                Ok(())
            }));
        }
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
    DispatcherBuilder::new()
        .add_handler(Handler::message(EchoHandler))
        .build(api.clone())
        .start_polling(api);
}
