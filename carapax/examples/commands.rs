use carapax::{
    longpoll::LongPoll, methods::SendMessage, types::Command, Api, Config, Dispatcher, ExecuteError, HandlerResult,
};
use dotenv::dotenv;
use std::env;

async fn handle_start(api: Api, command: Command) -> Result<HandlerResult, ExecuteError> {
    if command.get_name() == "/start" {
        return Ok(HandlerResult::Continue);
    }

    log::info!("handle /start command\n");
    let chat_id = command.get_message().get_chat_id();
    let method = SendMessage::new(chat_id, "Hello!");
    let result = api.execute(method).await;
    log::info!("sendMessage result: {:?}\n", result);
    Ok(HandlerResult::Stop)
}

async fn handle_user_id(api: Api, command: Command) -> Result<HandlerResult, ExecuteError> {
    if command.get_name() == "/user_id" {
        return Ok(HandlerResult::Continue);
    }

    log::info!("handle /user_id command\n");
    let message = command.get_message();
    let chat_id = message.get_chat_id();
    let method = SendMessage::new(chat_id, format!("Your ID is: {:?}", message.get_user().map(|u| u.id)));
    let result = api.execute(method).await?;
    log::info!("sendMessage result: {:?}\n", result);
    Ok(HandlerResult::Stop)
}

async fn handle_any(api: Api, command: Command) -> Result<(), ExecuteError> {
    let name = command.get_name();
    log::info!("handle {} command\n", name);
    let chat_id = command.get_message().get_chat_id();
    api.execute(SendMessage::new(chat_id, format!("Got {} command", name)))
        .await?;
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
    dispatcher
        .add_handler(handle_start)
        .add_handler(handle_user_id)
        .add_handler(handle_any);
    LongPoll::new(api, dispatcher).run().await;
}
