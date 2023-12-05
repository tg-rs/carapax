//! # Session
//!
//! Sessions provide a mechanism for storing a user-specific state in a storage such as filesystem or redis.
//!
//! Carapax utilizes the [seance](http://crates.io/crates/seance) crate for session support.
//! The required types are reexported from Carapax,
//! eliminating the need to add `seance` to your `Cargo.toml`.
//!
//! Every session is identified by a [SessionId](carapax::session::SessionId) struct,
//! which includes both a chat ID and a user ID.
//!
//! You can either get [`Session`] directly from the manager,
//! or use [`carapax::TryFromInput`] and specify `session: Session<B>` in handler arguments.
//! Where `B` is a [session backend](carapax::session::backend).
//! In both cases make sure that session manager is added to the context.
//!
//!
//! If an update lacks `chat_id` and/or `user_id`,
//! and the handler contains [`Session`] or [`carapax::session::SessionId`] in its arguments,
//! the handler will not be execute.
//! In such cases, you must obtain the session from the manager manually.
//!
//! Note that you need to enable either the `session-fs` or `session-redis` feature in `Cargo.toml`.
//! Alternatively, use the `session` feature if you have your own backend.
use carapax::{
    api::Client,
    session::{backend::fs::FilesystemBackend, Session},
    types::{ChatPeerId, Command, SendMessage},
    Chain, CommandExt, Ref,
};

use crate::error::AppError;

const KEY: &str = "example-session-key";

pub fn setup(chain: Chain) -> Chain {
    chain
        .with(get.with_command("/s_get"))
        .with(set.with_command("/s_set"))
        .with(expire.with_command("/s_expire"))
        .with(reset.with_command("/s_del"))
}

async fn get(
    client: Ref<Client>,
    mut session: Session<FilesystemBackend>,
    chat_id: ChatPeerId,
) -> Result<(), AppError> {
    log::info!("/s_get");
    let val: Option<String> = session.get(KEY).await?;
    client
        .execute(SendMessage::new(
            chat_id,
            format!("Value: {}", val.as_deref().unwrap_or("None")),
        ))
        .await?;
    Ok(())
}

async fn set(
    client: Ref<Client>,
    mut session: Session<FilesystemBackend>,
    chat_id: ChatPeerId,
    command: Command,
) -> Result<(), AppError> {
    let args = command.get_args();
    if args.is_empty() {
        client
            .execute(SendMessage::new(chat_id, "You need to provide a value"))
            .await?;
        return Ok(());
    }
    let val = &args[0];
    log::info!("/s_set {}", val);
    session.set(KEY, &val).await?;
    client.execute(SendMessage::new(chat_id, "OK")).await?;
    Ok(())
}

async fn expire(
    client: Ref<Client>,
    mut session: Session<FilesystemBackend>,
    chat_id: ChatPeerId,
    command: Command,
) -> Result<(), AppError> {
    let args = command.get_args();
    let seconds = if args.is_empty() {
        0
    } else {
        match args[0].parse::<u64>() {
            Ok(x) => x,
            Err(err) => {
                client
                    .execute(SendMessage::new(
                        chat_id,
                        format!("Number of seconds is invalid: {}", err),
                    ))
                    .await?;
                return Ok(());
            }
        }
    };
    log::info!("/s_expire {}", seconds);
    session.expire(KEY, seconds).await?;
    client.execute(SendMessage::new(chat_id, "OK")).await?;
    Ok(())
}

async fn reset(
    client: Ref<Client>,
    mut session: Session<FilesystemBackend>,
    chat_id: ChatPeerId,
) -> Result<(), AppError> {
    log::info!("/s_del");
    session.remove(KEY).await.unwrap();
    client.execute(SendMessage::new(chat_id, "OK")).await?;
    Ok(())
}
