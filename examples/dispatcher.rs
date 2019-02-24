use dotenv::dotenv;
use env_logger;
use futures::future::lazy;
use futures::{Future, Stream};
use log;
use std::env;
use tgbot::dispatcher::{Dispatcher, HandlerFuture, HandlerResult, MessageHandler};
use tgbot::methods::SendMessage;
use tgbot::types::{Message, MessageData, MessageKind};
use tgbot::Api;

struct Handler;

impl MessageHandler for Handler {
    fn handle(&self, api: &Api, message: &Message) -> HandlerFuture {
        log::info!("got a message: {:?}\n", message);
        if let MessageKind::Private { ref chat, .. } = message.kind {
            if let MessageData::Text(ref text) = message.data {
                let method = SendMessage::new(chat.id, text.data.clone());
                return HandlerFuture::new(api.execute(&method).then(|x| {
                    log::info!("sendMessage result: {:?}\n", x);
                    Ok(HandlerResult::Continue)
                }));
            }
        }
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

    tokio::run(lazy(move || {
        let mut dispatcher = Dispatcher::new(api.clone());
        dispatcher.add_message_handler(Handler);
        api.get_updates()
            .for_each(move |update| {
                tokio::spawn(dispatcher.dispatch(&update).then(|_| Ok(())));
                Ok(())
            })
            .then(|_| Ok(()))
    }));
}
