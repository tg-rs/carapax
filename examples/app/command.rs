use crate::error::AppError;
use carapax::{
    methods::SendMessage,
    types::{ChatId, User},
    Api, ChainBuilder, CommandExt, Ref,
};

pub fn setup(builder: &mut ChainBuilder) {
    builder.add_handler(greet.command("/hello"));
}

async fn greet(api: Ref<Api>, chat_id: ChatId, user: User) -> Result<(), AppError> {
    let method = SendMessage::new(chat_id, format!("Hello, {}", user.first_name));
    api.execute(method).await?;
    Ok(())
}
