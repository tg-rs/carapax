use carapax::{
    handler, longpoll::LongPoll, methods::SendMessage, Api, Command, Config, Dispatcher, ExecuteError, HandlerResult,
};
use dotenv::dotenv;
use env_logger;
use log;
use std::env;

#[handler(command = "/start")]
async fn handle_start(api: &Api, command: Command) -> Result<HandlerResult, ExecuteError> {
    log::info!("handle /start command\n");
    let chat_id = command.get_message().get_chat_id();
    let method = SendMessage::new(chat_id, "Hello!");
    let result = api.execute(method).await;
    log::info!("sendMessage result: {:?}\n", result);
    Ok(HandlerResult::Stop)
}

#[handler(command = "/user_id")]
async fn handle_user_id(api: &Api, command: Command) -> Result<(), ExecuteError> {
    log::info!("handle /user_id command\n");
    let message = command.get_message();
    let chat_id = message.get_chat_id();
    let method = SendMessage::new(chat_id, format!("Your ID is: {:?}", message.get_user().map(|u| u.id)));
    let result = api.execute(method).await?;
    log::info!("sendMessage result: {:?}\n", result);
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
    let mut dispatcher = Dispatcher::new(api.clone());
    dispatcher.add_handler(handle_start);
    dispatcher.add_handler(handle_user_id);
    LongPoll::new(api, dispatcher).run().await;
}
