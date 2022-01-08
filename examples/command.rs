use carapax::{
    methods::SendMessage,
    types::{ChatId, User},
    Api, CommandExt, ExecuteError, Ref,
};
use dotenv::dotenv;

async fn hello_handler(api: Ref<Api>, chat_id: ChatId, user: User) -> Result<(), ExecuteError> {
    let method = SendMessage::new(chat_id, format!("Hello, {}", user.first_name));
    api.execute(method).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    helper::run(hello_handler.command("/hello")).await;
}
