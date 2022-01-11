use crate::util::Module;
use carapax::{
    methods::SendMessage,
    session::{backend::fs::FilesystemBackend, Session, SessionError},
    types::{ChatId, Command},
    Api, CommandExt, Dispatcher, ExecuteError, HandlerResult, Ref,
};
use std::{error::Error, fmt};

pub struct SessionModule;

impl Module for SessionModule {
    fn add_handlers(&self, dispatcher: &mut Dispatcher) {
        dispatcher
            .add_handler(handle_expire.command("/expire"))
            .add_handler(handle_reset.command("/reset"))
            .add_handler(handle_set.command("/set"))
            .add_handler(handle_update);
    }
}

#[derive(Debug)]
enum AppError {
    Execute(ExecuteError),
    Session(SessionError),
}

impl fmt::Display for AppError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Execute(err) => write!(out, "Execute error: {}", err),
            AppError::Session(err) => write!(out, "Session error: {}", err),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Execute(err) => Some(err),
            AppError::Session(err) => Some(err),
        }
    }
}

impl From<ExecuteError> for AppError {
    fn from(err: ExecuteError) -> Self {
        AppError::Execute(err)
    }
}

impl From<SessionError> for AppError {
    fn from(err: SessionError) -> Self {
        AppError::Session(err)
    }
}

async fn handle_expire(
    api: Ref<Api>,
    mut session: Session<FilesystemBackend>,
    chat_id: ChatId,
    command: Command,
) -> Result<HandlerResult, AppError> {
    let args = command.get_args();
    let seconds = if args.is_empty() {
        0
    } else {
        match args[0].parse::<u64>() {
            Ok(x) => x,
            Err(err) => {
                api.execute(SendMessage::new(chat_id, err.to_string())).await?;
                return Ok(HandlerResult::Stop);
            }
        }
    };
    log::info!("/expire {}", seconds);
    session.expire("counter", seconds).await?;
    api.execute(SendMessage::new(chat_id, "OK")).await?;
    Ok(HandlerResult::Stop)
}

async fn handle_set(
    api: Ref<Api>,
    mut session: Session<FilesystemBackend>,
    chat_id: ChatId,
    command: Command,
) -> Result<HandlerResult, AppError> {
    let args = command.get_args();
    let val = if args.is_empty() {
        0
    } else {
        match args[0].parse::<usize>() {
            Ok(x) => x,
            Err(err) => {
                api.execute(SendMessage::new(chat_id, err.to_string())).await?;
                return Ok(HandlerResult::Stop);
            }
        }
    };
    log::info!("/set {}", val);
    session.set("counter", &val).await?;
    api.execute(SendMessage::new(chat_id, "OK")).await?;
    Ok(HandlerResult::Stop)
}

async fn handle_reset(
    api: Ref<Api>,
    mut session: Session<FilesystemBackend>,
    chat_id: ChatId,
) -> Result<HandlerResult, ExecuteError> {
    log::info!("/reset");
    session.remove("counter").await.unwrap();
    api.execute(SendMessage::new(chat_id, "OK")).await?;
    Ok(HandlerResult::Stop)
}

async fn handle_update(mut session: Session<FilesystemBackend>) -> Result<(), ExecuteError> {
    let val: Option<usize> = session.get("counter").await.unwrap();
    let val = val.unwrap_or(0) + 1;
    log::info!("got an update, increment counter by {}", val);
    session.set("counter", &val).await.unwrap();
    Ok(())
}
