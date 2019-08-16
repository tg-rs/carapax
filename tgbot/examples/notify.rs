use dotenv::dotenv;
use env_logger;
use futures::future;
use std::{
    env,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
    time::Duration,
};
use tgbot::{
    methods::SendMessage,
    types::{ChatId, Integer, Update},
    Api, Config,
};

#[allow(clippy::large_enum_variant)]
enum Notification {
    Hello,
    #[allow(dead_code)]
    Update(Update),
    Stop,
}

fn main() {
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
        config = config.proxy(proxy);
    }
    let api = Api::new(config).expect("Failed to create API");

    let (tx, rx) = channel();
    let rx = Arc::new(Mutex::new(rx));
    thread::spawn(move || {
        let rx = rx.clone();
        tokio::run(future::lazy(move || {
            let rx = rx.lock().unwrap();
            loop {
                let notification = rx.recv().unwrap();
                match notification {
                    Notification::Update(_update) => {
                        // you can handle update from telegram here
                        unimplemented!()
                    }
                    Notification::Hello => api.spawn(api.execute(SendMessage::new(chat_id.clone(), "Hello!"))),
                    Notification::Stop => break,
                }
            }
            Ok(())
        }))
    });

    let timeout = Duration::from_secs(1);
    for _ in 0..10 {
        tx.send(Notification::Hello).unwrap();
        thread::sleep(timeout);
    }
    // You also can send update got from telegram: tx.send(Notification::Update(update))
    tx.send(Notification::Stop).unwrap();
}
