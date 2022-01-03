use carapax::{
    methods::SendMessage,
    session::{backend::fs::FilesystemBackend, Session, SessionCollector, SessionError, SessionManager},
    types::{ChatId, Command},
    Api, CommandExt, Config, Context, Dispatcher, ExecuteError, HandlerResult, Ref,
};
use dotenv::dotenv;
use std::{env, error::Error, fmt, time::Duration};
use tempfile::tempdir;
use tgbot::longpoll::LongPoll;

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

fn getenv(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("{} is not set", name))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = getenv("CARAPAX_TOKEN");
    let proxy = env::var("CARAPAX_PROXY").ok();
    let gc_period = getenv("CARAPAX_SESSION_GC_PERIOD");
    let gc_period = Duration::from_secs(
        gc_period
            .parse::<u64>()
            .expect("CARAPAX_SESSION_GC_PERIOD must be integer"),
    ); // period between GC calls
    let session_lifetime = getenv("CARAPAX_SESSION_LIFETIME");
    let session_lifetime = Duration::from_secs(
        session_lifetime
            .parse::<u64>()
            .expect("CARAPAX_SESSION_LIFETIME must be integer"),
    ); // how long session lives

    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }

    let api = Api::new(config).expect("Failed to create API");
    let tmpdir = tempdir().expect("Failed to create temp directory");
    log::info!("Session directory: {}", tmpdir.path().display());

    let backend = FilesystemBackend::new(tmpdir.path());

    // spawn GC to remove old sessions
    let mut collector = SessionCollector::new(backend.clone(), gc_period, session_lifetime);
    tokio::spawn(async move { collector.run().await });

    let mut context = Context::default();
    context.insert(api.clone());
    context.insert(SessionManager::new(backend));

    let mut dispatcher = Dispatcher::new(context);
    dispatcher
        .add_handler(handle_expire.command("/expire"))
        .add_handler(handle_reset.command("/reset"))
        .add_handler(handle_set.command("/set"))
        .add_handler(handle_update);
    LongPoll::new(api, dispatcher).run().await
}
