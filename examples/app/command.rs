use carapax::{
    api::Client,
    types::{ChatId, SendMessage, User},
    Chain, CommandExt, Ref,
};

use crate::error::AppError;

pub fn setup(chain: Chain) -> Chain {
    chain.add(greet.command("/hello"))
}

async fn greet(client: Ref<Client>, chat_id: ChatId, user: User) -> Result<(), AppError> {
    let method = SendMessage::new(chat_id, format!("Hello, {}", user.first_name));
    client.execute(method).await?;
    Ok(())
}
