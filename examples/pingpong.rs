use carapax::{
    methods::SendMessage,
    types::{Message, Text},
    Api, ExecuteError, PredicateExt, Ref,
};
use dotenv::dotenv;

async fn is_ping(text: Text) -> bool {
    text.data == "ping"
}

async fn pingpong_handler(api: Ref<Api>, message: Message) -> Result<(), ExecuteError> {
    let chat_id = message.get_chat_id();
    let method = SendMessage::new(chat_id, "pong");
    api.execute(method).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    helper::run(pingpong_handler.predicate(is_ping)).await;
}
