use dotenv::dotenv;
use env_logger;
use futures::Future;
use log;
use std::env;
use tgbot::{
    handle_updates,
    methods::{SendAnimation, SendDocument},
    types::{MessageData, Update, UpdateKind},
    Api, Config, UpdateHandler, UpdateMethod,
};

const GIF_URL: &str = "https://66.media.tumblr.com/3b2ae39de623518901cdbfe87ffde31c/tumblr_mjq1rm7O6Q1racqsfo1_400.gif";

struct Handler {
    api: Api,
}

impl UpdateHandler for Handler {
    fn handle(&mut self, update: Update) {
        log::info!("got an update: {:?}\n", update);

        macro_rules! execute {
            ($method:expr) => {
                self.api.spawn(self.api.execute($method).then(|x| {
                    log::info!("sendMessage result: {:?}\n", x);
                    Ok::<(), ()>(())
                }));
            };
        }

        if let UpdateKind::Message(message) = update.kind {
            let chat_id = message.get_chat_id();
            if let MessageData::Document { data, .. } = message.data {
                // Resend document by file id (you also can send a document using URL)
                execute!(SendDocument::new(chat_id, data.file_id));
            } else if let Some(text) = message.get_text() {
                match text.data.as_str() {
                    // Send animation by URL (you also can send animation using a file_id)
                    "/gif" => execute!(SendAnimation::new(chat_id, GIF_URL)),
                    // The same way for other file types...
                    // Note that currently you are unable to upload a file from disk
                    // (this will be changed in future)
                    _ => {}
                };
            }
        }
    }
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
    let api = Api::new(config).expect("Failed to create API");
    tokio::run(handle_updates(UpdateMethod::poll(api.clone()), Handler { api }));
}
