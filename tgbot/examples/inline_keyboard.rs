use async_trait::async_trait;
use dotenv::dotenv;
use env_logger;
use log;
use serde::{Deserialize, Serialize};
use std::env;
use tgbot::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{InlineKeyboardButton, Message, Update, UpdateKind},
    Api, Config, UpdateHandler,
};

struct Handler {
    api: Api,
}

#[derive(Deserialize, Serialize)]
struct CallbackData {
    value: String,
}

impl CallbackData {
    fn new<S: Into<String>>(value: S) -> Self {
        Self { value: value.into() }
    }
}

async fn handle_update(api: &Api, update: Update) -> Option<Message> {
    match update.kind {
        UpdateKind::Message(message) => {
            let chat_id = message.get_chat_id();
            if let Some(commands) = message.commands {
                let command = &commands[0];
                if command.command == "/start" {
                    let callback_data = CallbackData::new("hello!");
                    let method = SendMessage::new(chat_id, "keyboard example").reply_markup(vec![vec![
                        // You also can use with_callback_data in order to pass a plain string
                        InlineKeyboardButton::with_callback_data_struct("button", &callback_data).unwrap(),
                    ]]);
                    return Some(api.execute(method).await.unwrap());
                }
            }
        }
        UpdateKind::CallbackQuery(query) => {
            if let Some(ref message) = query.message {
                let chat_id = message.get_chat_id();
                // or query.data if you have passed a plain string
                let data = query.parse_data::<CallbackData>().unwrap().unwrap();
                let method = SendMessage::new(chat_id, data.value);
                return Some(api.execute(method).await.unwrap());
            }
        }
        _ => {}
    }
    None
}

#[async_trait]
impl UpdateHandler for Handler {
    type Error = ();

    async fn handle(&mut self, update: Update) -> Result<(), Self::Error> {
        log::info!("Got an update: {:?}", update);
        handle_update(&self.api, update).await.unwrap();
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();
    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }
    let api = Api::new(config).expect("Failed to create API");
    LongPoll::new(api.clone(), Handler { api }).run().await;
}
