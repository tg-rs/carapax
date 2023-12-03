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
        .with(get.command("/s_get"))
        .with(set.command("/s_set"))
        .with(expire.command("/s_expire"))
        .with(reset.command("/s_del"))
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
