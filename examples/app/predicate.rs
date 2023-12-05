//! # Predicates
//!
//! [`carapax::Predicate`] is a decorator that helps determine whether a handler should run or not.
//! This is particularly useful for implementing functionalities like rate-limiting
//! or restricting certain users from triggering a handler.
//!
//! A predicate handler must implement the [`carapax::Handler`] trait,
//! and return a [`carapax::PredicateResult`]
//! or a type that can be converted into it:
//!
//! |   From              | To                                     |
//! |---------------------|----------------------------------------|
//! | `true`              | [`carapax::PredicateResult::True`]     |
//! | `false`             | [`carapax::PredicateResult::False`]    |
//! | `Result<T, E>::Ok`  | `PredicateResult::from::<T>()`         |
//! | `Result<T, E>::Err` | [`carapax::PredicateResult::Err`]      |
use carapax::{
    api::Client,
    types::{ChatPeerId, SendMessage, Text},
    Chain, PredicateExt, Ref,
};

use crate::error::AppError;

pub fn setup(chain: Chain) -> Chain {
    chain.with(pong.with_predicate(is_ping))
}

async fn is_ping(text: Text) -> bool {
    text.data == "ping"
}

async fn pong(client: Ref<Client>, chat_id: ChatPeerId) -> Result<(), AppError> {
    let method = SendMessage::new(chat_id, "pong");
    client.execute(method).await?;
    Ok(())
}
