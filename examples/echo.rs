use std::env;

use dotenvy::dotenv;

use carapax::{
    api::{Client, ExecuteError},
    handler::LongPoll,
    types::{ChatId, SendMessage, Text},
    App, Context, Ref,
};

async fn echo(client: Ref<Client>, chat_id: ChatId, message: Text) -> Result<(), ExecuteError> {
    let method = SendMessage::new(chat_id, message.data);
    client.execute(method).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let client = Client::new(token).expect("Failed to create API");

    let mut context = Context::default();
    context.insert(client.clone());

    let app = App::new(context, echo);
    LongPoll::new(client, app).run().await
}
