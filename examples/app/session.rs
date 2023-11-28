use carapax::{
    api::Client,
    session::{backend::fs::FilesystemBackend, Session},
    types::{ChatId, Command, SendMessage},
    Chain, CommandExt, Ref,
};

use crate::error::AppError;

const KEY: &str = "example-session-key";

pub fn setup(chain: Chain) -> Chain {
    chain
        .add(get.command("/sget"))
        .add(set.command("/sset"))
        .add(expire.command("/sexpire"))
        .add(reset.command("/sdel"))
}

async fn get(client: Ref<Client>, mut session: Session<FilesystemBackend>, chat_id: ChatId) -> Result<(), AppError> {
    log::info!("/sget");
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
    chat_id: ChatId,
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
    log::info!("/sset {}", val);
    session.set(KEY, &val).await?;
    client.execute(SendMessage::new(chat_id, "OK")).await?;
    Ok(())
}

async fn expire(
    client: Ref<Client>,
    mut session: Session<FilesystemBackend>,
    chat_id: ChatId,
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
    log::info!("/sexpire {}", seconds);
    session.expire(KEY, seconds).await?;
    client.execute(SendMessage::new(chat_id, "OK")).await?;
    Ok(())
}

async fn reset(client: Ref<Client>, mut session: Session<FilesystemBackend>, chat_id: ChatId) -> Result<(), AppError> {
    log::info!("/sdel");
    session.remove(KEY).await.unwrap();
    client.execute(SendMessage::new(chat_id, "OK")).await?;
    Ok(())
}
