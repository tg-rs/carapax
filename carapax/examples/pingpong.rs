use carapax::{methods::SendMessage, types::Message, Api, App, Dispatcher, ExecuteError, HandlerExt};
use dotenv::dotenv;

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

    App::from_env()
        .with_dispatcher(|dispatcher| {
            dispatcher.add_handler(pingpong_handler.guard(is_ping));
        })
        .long_poll()
        .run()
        .await;
}
