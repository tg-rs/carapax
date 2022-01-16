use crate::error::AppError;
use carapax::{
    methods::SendMessage,
    session::{backend::fs::FilesystemBackend, Session},
    types::{ChatId, Command},
    Api, CommandExt, DispatcherBuilder, Ref,
};

const KEY: &str = "example-session-key";

pub fn setup(builder: &mut DispatcherBuilder) {
    builder
        .add_handler(get.command("/sget"))
        .add_handler(set.command("/sset"))
        .add_handler(expire.command("/sexpire"))
        .add_handler(reset.command("/sdel"));
}

async fn get(api: Ref<Api>, mut session: Session<FilesystemBackend>, chat_id: ChatId) -> Result<(), AppError> {
    log::info!("/sget");
    let val: Option<String> = session.get(KEY).await?;
    api.execute(SendMessage::new(
        chat_id,
        format!("Value: {}", val.unwrap_or_else(|| String::from("None"))),
    ))
    .await?;
    Ok(())
}

async fn set(
    api: Ref<Api>,
    mut session: Session<FilesystemBackend>,
    chat_id: ChatId,
    command: Command,
) -> Result<(), AppError> {
    let args = command.get_args();
    if args.is_empty() {
        api.execute(SendMessage::new(chat_id, "You need to provide a value"))
            .await?;
        return Ok(());
    }
    let val = &args[0];
    log::info!("/sset {}", val);
    session.set(KEY, &val).await?;
    api.execute(SendMessage::new(chat_id, "OK")).await?;
    Ok(())
}

async fn expire(
    api: Ref<Api>,
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
                api.execute(SendMessage::new(
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
    api.execute(SendMessage::new(chat_id, "OK")).await?;
    Ok(())
}

async fn reset(api: Ref<Api>, mut session: Session<FilesystemBackend>, chat_id: ChatId) -> Result<(), AppError> {
    log::info!("/sdel");
    session.remove(KEY).await.unwrap();
    api.execute(SendMessage::new(chat_id, "OK")).await?;
    Ok(())
}
