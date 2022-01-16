use crate::error::AppError;
use carapax::{
    methods::SendMessage,
    types::{ChatId, Text},
    Api, ChainBuilder, PredicateExt, Ref,
};

pub fn setup(builder: &mut ChainBuilder) {
    builder.add_handler(pong.predicate(is_ping));
}

async fn is_ping(text: Text) -> bool {
    text.data == "ping"
}

async fn pong(api: Ref<Api>, chat_id: ChatId) -> Result<(), AppError> {
    let method = SendMessage::new(chat_id, "pong");
    api.execute(method).await?;
    Ok(())
}
