use crate::error::AppError;
use carapax::{
    methods::SendMessage,
    session::{backend::fs::FilesystemBackend, Session},
    types::{ChatId, Command},
    Api, CommandExt, Dispatcher, Ref,
};

pub fn setup(dispatcher: &mut Dispatcher) {
    dispatcher
        .add_handler(update_counter)
        .add_handler(get_counter.command("/cget"))
        .add_handler(set_counter.command("/cset"))
        .add_handler(expire_counter.command("/cexpire"))
        .add_handler(reset_counter.command("/creset"));
}

async fn update_counter(mut session: Session<FilesystemBackend>) -> Result<(), AppError> {
    let val: Option<usize> = session.get("counter").await.unwrap();
    let val = val.unwrap_or(0) + 1;
    log::info!("got an update, increment counter by {}", val);
    session.set("counter", &val).await.unwrap();
    Ok(())
}

async fn get_counter(api: Ref<Api>, mut session: Session<FilesystemBackend>, chat_id: ChatId) -> Result<(), AppError> {
    log::info!("/cget");
    let val: Option<usize> = session.get("counter").await?;
    api.execute(SendMessage::new(
        chat_id,
        format!("Counter value: {}", val.unwrap_or(0)),
    ))
    .await?;
    Ok(())
}

async fn set_counter(
    api: Ref<Api>,
    mut session: Session<FilesystemBackend>,
    chat_id: ChatId,
    command: Command,
) -> Result<(), AppError> {
    let args = command.get_args();
    let val = if args.is_empty() {
        0
    } else {
        match args[0].parse::<usize>() {
            Ok(x) => x,
            Err(err) => {
                api.execute(SendMessage::new(chat_id, err.to_string())).await?;
                return Ok(());
            }
        }
    };
    log::info!("/cset {}", val);
    session.set("counter", &val).await?;
    api.execute(SendMessage::new(chat_id, "OK")).await?;
    Ok(())
}

async fn expire_counter(
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
                api.execute(SendMessage::new(chat_id, err.to_string())).await?;
                return Ok(());
            }
        }
    };
    log::info!("/cexpire {}", seconds);
    session.expire("counter", seconds).await?;
    api.execute(SendMessage::new(chat_id, "OK")).await?;
    Ok(())
}

async fn reset_counter(
    api: Ref<Api>,
    mut session: Session<FilesystemBackend>,
    chat_id: ChatId,
) -> Result<(), AppError> {
    log::info!("/creset");
    session.remove("counter").await.unwrap();
    api.execute(SendMessage::new(chat_id, "OK")).await?;
    Ok(())
}
