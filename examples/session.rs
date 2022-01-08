use carapax::{
    methods::SendMessage,
    session::{backend::fs::FilesystemBackend, Session, SessionError, SessionManager},
    types::{ChatId, Command},
    Api, CommandExt, ExecuteError, HandlerResult, Ref,
};
use dotenv::dotenv;
use helper::RunnerBuilder;
use seance::SessionCollector;
use std::{error::Error, fmt, time::Duration};
use tempfile::tempdir;

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
    log::info!("/expire {}", seconds);
    session.expire("counter", seconds).await?;
    api.execute(SendMessage::new(chat_id, "OK")).await?;
    Ok(())
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

async fn handle_update(
    api: Ref<Api>,
    mut session: Session<FilesystemBackend>,
    chat_id: ChatId,
) -> Result<(), ExecuteError> {
    let val: Option<usize> = session.get("counter").await.unwrap();
    let val = val.unwrap_or(0) + 1;
    log::info!("got an update, increment counter by {}", val);
    session.set("counter", &val).await.unwrap();
    let msg = format!("Count: {}", val);
    api.execute(SendMessage::new(chat_id, msg)).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let backend = backend_with_tmpdir();
    spawn_collector(backend.clone());

    RunnerBuilder::from_env()
        .insert_data(SessionManager::new(backend))
        .build()
        .add_handler(handle_expire.command("/expire"))
        .add_handler(handle_reset.command("/reset"))
        .add_handler(handle_set.command("/set"))
        .add_handler(handle_update)
        .run()
        .await;
}

fn backend_with_tmpdir() -> FilesystemBackend {
    let tmpdir = tempdir().expect("Failed to create temp directory");
    log::info!("Session directory: {}", tmpdir.path().display());
    let backend = FilesystemBackend::new(tmpdir.path());
    backend
}

fn spawn_collector(backend: FilesystemBackend) {
    let gc_period = helper::get_env("CARAPAX_SESSION_GC_PERIOD");
    let gc_period = Duration::from_secs(
        gc_period
            .parse::<u64>()
            .expect("CARAPAX_SESSION_GC_PERIOD must be integer"),
    ); // period between GC calls

    let session_lifetime = helper::get_env("CARAPAX_SESSION_LIFETIME");
    let session_lifetime = Duration::from_secs(
        session_lifetime
            .parse::<u64>()
            .expect("CARAPAX_SESSION_LIFETIME must be integer"),
    ); // how long session lives

    // spawn GC to remove old sessions
    let mut collector = SessionCollector::new(backend, gc_period, session_lifetime);
    tokio::spawn(async move { collector.run().await });
}
