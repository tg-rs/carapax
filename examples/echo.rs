use dotenv::dotenv;
use env_logger;
use futures::{future::Future, stream::Stream};
use log;
use std::env;
use tgbot::{
    methods::SendMessage,
    types::{Update, UpdateKind},
    Api, UpdateHandler, UpdateMethod, UpdatesStream,
};

struct Handler;

impl UpdateHandler for Handler {
    fn handle(&mut self, api: &Api, update: Update) {
        if let UpdateKind::Message(msg) = update.kind {
            log::info!("GOT A MESSAGE: {:?}\n", msg);
            if let Some(text) = msg.get_text() {
                let chat_id = msg.get_chat_id();
                let method = SendMessage::new(chat_id, text.data.clone());
                api.spawn(api.execute(&method));
            }
        }
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
    let proxy = env::var("TGBOT_PROXY").ok();
    let api = match proxy {
        Some(proxy) => Api::with_proxy(token, &proxy),
        None => Api::new(token),
    }
    .expect("Failed to create API");

    // use UpdateHandler
    api.get_updates(UpdateMethod::Polling, Handler);

    // use futures::stream::Stream
    tokio::run(
        UpdatesStream::new(api.clone())
            .for_each(move |update| {
                if let UpdateKind::Message(msg) = update.kind {
                    log::info!("GOT A MESSAGE: {:?}\n", msg);
                    if let Some(text) = msg.get_text() {
                        let chat_id = msg.get_chat_id();
                        let method = SendMessage::new(chat_id, text.data.clone());
                        api.spawn(api.execute(&method));
                    }
                }

                Ok(())
            })
            .then(|_| Ok(())),
    );
}
