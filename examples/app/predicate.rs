use carapax::{
    api::Client,
    types::{ChatId, SendMessage, Text},
    Chain, PredicateExt, Ref,
};

use crate::error::AppError;

pub fn setup(chain: Chain) -> Chain {
    chain.add(pong.predicate(is_ping))
}

async fn is_ping(text: Text) -> bool {
    text.data == "ping"
}

async fn pong(client: Ref<Client>, chat_id: ChatId) -> Result<(), AppError> {
    let method = SendMessage::new(chat_id, "pong");
    client.execute(method).await?;
    Ok(())
}
