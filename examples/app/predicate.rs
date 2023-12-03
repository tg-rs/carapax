use carapax::{
    api::Client,
    types::{ChatPeerId, SendMessage, Text},
    Chain, PredicateExt, Ref,
};

use crate::error::AppError;

pub fn setup(chain: Chain) -> Chain {
    chain.with(pong.predicate(is_ping))
}

async fn is_ping(text: Text) -> bool {
    text.data == "ping"
}

async fn pong(client: Ref<Client>, chat_id: ChatPeerId) -> Result<(), AppError> {
    let method = SendMessage::new(chat_id, "pong");
    client.execute(method).await?;
    Ok(())
}
