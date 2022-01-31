use crate::error::AppError;
use carapax::{
    methods::SendMessage,
    types::{ChatId, Text},
    Api, Chain, PredicateExt, Ref,
};

pub fn setup(chain: Chain) -> Chain {
    chain.add(pong.predicate(is_ping))
}

async fn is_ping(text: Text) -> bool {
    text.data == "ping"
}

async fn pong(api: Ref<Api>, chat_id: ChatId) -> Result<(), AppError> {
    let method = SendMessage::new(chat_id, "pong");
    api.execute(method).await?;
    Ok(())
}
