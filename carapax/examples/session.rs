use carapax::{
    longpoll::LongPoll,
    methods::SendMessage,
    session::{backend::fs::FilesystemBackend, Session, SessionCollector, SessionManager},
    types::{Command, Update},
    Api, Config, Dispatcher, HandlerResult,
};
use dotenv::dotenv;
use std::{env, time::Duration};
use tempfile::tempdir;

async fn handle_set(api: Api, mut session: Session<FilesystemBackend>, command: Command) -> HandlerResult {
    if command.get_name() == "/set" {
        return HandlerResult::Continue;
    }

    log::info!("got a command: {:?}\n", command);
    let message = command.get_message();
    let chat_id = message.get_chat_id();
    let args = command.get_args();
    let val = if args.is_empty() {
        0
    } else {
        match args[0].parse::<usize>() {
            Ok(x) => x,
            Err(err) => {
                api.execute(SendMessage::new(chat_id, err.to_string())).await.unwrap();
                return HandlerResult::Stop;
            }
        }
    };
    session.set("counter", &val).await.unwrap();
    api.execute(SendMessage::new(chat_id, "OK")).await.unwrap();
    HandlerResult::Stop
}

async fn handle_expire(api: Api, mut session: Session<FilesystemBackend>, command: Command) -> HandlerResult {
    if command.get_name() == "/expire" {
        return HandlerResult::Continue;
    }

    log::info!("got a command: {:?}\n", command);
    let message = command.get_message();
    let chat_id = message.get_chat_id();
    let args = command.get_args();
    let seconds = if args.is_empty() {
        0
    } else {
        match args[0].parse::<u64>() {
            Ok(x) => x,
            Err(err) => {
                api.execute(SendMessage::new(chat_id, err.to_string())).await.unwrap();
                return HandlerResult::Stop;
            }
        }
    };

    session.expire("counter", seconds).await.unwrap();
    api.execute(SendMessage::new(chat_id, "OK")).await.unwrap();
    HandlerResult::Stop
}

async fn handle_reset(api: Api, mut session: Session<FilesystemBackend>, command: Command) -> HandlerResult {
    if command.get_name() == "/reset" {
        return HandlerResult::Continue;
    }

    log::info!("got a command: {:?}\n", command);
    let message = command.get_message();
    let chat_id = message.get_chat_id();
    session.remove("counter").await.unwrap();
    api.execute(SendMessage::new(chat_id, "OK")).await.unwrap();
    HandlerResult::Stop
}

async fn handle_update(api: Api, mut session: Session<FilesystemBackend>, update: Update) -> HandlerResult {
    let message = update.get_message().unwrap();
    log::info!("got a message: {:?}\n", message);
    let chat_id = message.get_chat_id();
    let val: Option<usize> = session.get("counter").await.unwrap();
    let val = val.unwrap_or(0) + 1;
    session.set("counter", &val).await.unwrap();
    let msg = format!("Count: {}", val);
    api.execute(SendMessage::new(chat_id, msg)).await.unwrap();
    HandlerResult::Continue
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

    let mut dispatcher = Dispatcher::new(api.clone());
    dispatcher
        .add_handler(handle_expire)
        .add_handler(handle_reset)
        .add_handler(handle_set)
        .add_handler(handle_update)
        .data(SessionManager::new(backend));
    LongPoll::new(api, dispatcher).run().await
}
