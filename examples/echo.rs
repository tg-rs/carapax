use carapax::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{ChatId, Text},
    Api, Context, Dispatcher, ExecuteError, Ref,
};
use dotenv::dotenv;
use std::env;

async fn echo(api: Ref<Api>, chat_id: ChatId, message: Text) -> Result<(), ExecuteError> {
    let method = SendMessage::new(chat_id, message.data);
    api.execute(method).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let api = Api::new(token).expect("Failed to create API");

    let mut context = Context::default();
    context.insert(api.clone());

    let mut dispatcher = Dispatcher::new(context);
    dispatcher.add_handler(echo);

    LongPoll::new(api, dispatcher).run().await
}
