use carapax::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{ChatId, User},
    Api, CommandExt, Config, Context, Dispatcher, ExecuteError, Ref,
};
use dotenv::dotenv;
use std::env;

async fn hello_handler(api: Ref<Api>, chat_id: ChatId, user: User) -> Result<(), ExecuteError> {
    let method = SendMessage::new(chat_id, format!("Hello, {}", user.first_name));
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
    let mut context = Context::default();
    context.insert(api.clone());
    let mut dispatcher = Dispatcher::new(context);
    dispatcher.add_handler(hello_handler.command("/hello"));
    LongPoll::new(api, dispatcher).run().await
}
