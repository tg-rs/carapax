use carapax::{
    longpoll::LongPoll, methods::SendMessage, types::Message, Api, Config, Dispatcher, ExecuteError, Handler,
};
use dotenv::dotenv;
use std::env;

async fn is_ping(message: Message) -> bool {
    message.get_text().map(AsRef::as_ref) == Some("ping")
}

async fn pingpong_handler(api: Api, message: Message) -> Result<(), ExecuteError> {
    let chat_id = message.get_chat_id();
    let method = SendMessage::new(chat_id, "pong");
    api.execute(method).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let proxy = env::var("CARAPAX_PROXY").ok();
    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }
    let api = Api::new(config).expect("Failed to create API");
    let mut dispatcher = Dispatcher::new(api.clone());
    dispatcher.add_handler(pingpong_handler.guard(is_ping));
    LongPoll::new(api, dispatcher).run().await
}
