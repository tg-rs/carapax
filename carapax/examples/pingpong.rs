use carapax::{
    context::Context, handler, longpoll::LongPoll, methods::SendMessage, types::Message, Api, Config, Dispatcher,
    ExecuteError,
};
use dotenv::dotenv;
use std::{convert::Infallible, env};

async fn is_ping(_context: &mut Context, message: &Message) -> Result<bool, Infallible> {
    Ok(message.get_text().map(|text| text.data == "ping").unwrap_or(false))
}

// Handler will not run if message text not equals "ping"
#[handler(predicate=is_ping)]
async fn pingpong_handler(context: &mut Context, message: Message) -> Result<(), ExecuteError> {
    let chat_id = message.get_chat_id();
    let method = SendMessage::new(chat_id, "pong");
    let api = context.get::<Api>();
    api.execute(method).await?;
    Ok(())
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
    let mut context = Context::default();
    context.set(api.clone());
    let mut dispatcher = Dispatcher::new(context);
    dispatcher.add_handler(pingpong_handler);
    LongPoll::new(api, dispatcher).run().await
}
