use dotenv::dotenv;
use env_logger;
use futures::Future;
use log;
use std::env;
use tgbot::{
    handle_updates,
    methods::SendMessage,
    types::{Update, UpdateKind},
    Api, UpdateHandler, UpdateMethod,
};

struct Handler {
    api: Api,
}

impl UpdateHandler for Handler {
    fn handle(&mut self, update: Update) {
        log::info!("got an update: {:?}\n", update);
        if let UpdateKind::Message(message) = update.kind {
            if let Some(text) = message.get_text() {
                let chat_id = message.get_chat_id();
                let method = SendMessage::new(chat_id, text.data.clone());
                self.api.spawn(self.api.execute(&method).then(|x| {
                    log::info!("sendMessage result: {:?}\n", x);
                    Ok::<(), ()>(())
                }));
            }
        }
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
    let proxy = env::var("TGBOT_PROXY").ok();
    let api = Api::new(token, proxy).expect("Failed to create API");
    tokio::run(handle_updates(UpdateMethod::poll(api.clone()), Handler { api }));
}
