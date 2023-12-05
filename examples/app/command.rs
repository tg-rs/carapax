//! # Commands
//!
//! By wrapping the [`greet`] handler with the [`carapax::CommandPredicate`],
//! it ensures that the handler is executed only when an incoming update
//! contains a message with the `/hello` command.
use carapax::{
    api::Client,
    types::{ChatPeerId, SendMessage, User},
    Chain, CommandExt, Ref,
};

use crate::error::AppError;

pub fn setup(chain: Chain) -> Chain {
    chain.with(greet.with_command("/hello"))
}

async fn greet(client: Ref<Client>, chat_id: ChatPeerId, user: User) -> Result<(), AppError> {
    let method = SendMessage::new(chat_id, format!("Hello, {}", user.first_name));
    client.execute(method).await?;
    Ok(())
}
