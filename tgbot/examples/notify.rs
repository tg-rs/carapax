use dotenv::dotenv;
use env_logger;
use std::{env, time::Duration};
use tgbot::{
    methods::SendMessage,
    types::{ChatId, Integer, Update},
    Api, Config,
};
use tokio::{spawn, sync::mpsc, time::delay_for};

#[allow(clippy::large_enum_variant)]
enum Notification {
    Hello,
    #[allow(dead_code)]
    Update(Update),
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();
    let chat_id = env::var("TGRS_NOTIFICATION_CHAT_ID").expect("TGRS_NOTIFICATION_CHAT_ID is not set");
    let chat_id = match chat_id.parse::<Integer>() {
        Ok(chat_id) => ChatId::Id(chat_id),
        Err(_) => ChatId::Username(chat_id),
    };
    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }
    let api = Api::new(config).expect("Failed to create API");

    let (mut tx, mut rx) = mpsc::channel(100);

    spawn(async move {
        let timeout = Duration::from_secs(1);
        for _ in 0..10usize {
            if tx.send(Notification::Hello).await.is_err() {
                println!("Receiver dropped");
                return;
            }
            delay_for(timeout).await;
        }
    });

    while let Some(notification) = rx.recv().await {
        match notification {
            Notification::Update(_update) => {
                // you can handle update from telegram here
                unimplemented!()
            }
            Notification::Hello => {
                api.execute(SendMessage::new(chat_id.clone(), "Hello!")).await.unwrap();
            }
        }
    }
}
