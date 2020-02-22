#[cfg(not(feature = "session-fs"))]
compile_error!("Enable session-fs feature");

use carapax::{
    handler,
    longpoll::LongPoll,
    methods::SendMessage,
    session::{backend::fs::FilesystemBackend, SessionCollector, SessionManager},
    types::Update,
    Api, Command, Config, Dispatcher, HandlerResult,
};
use dotenv::dotenv;
use std::{env, time::Duration};
use tempfile::tempdir;

struct Context {
    api: Api,
    session_manager: SessionManager<FilesystemBackend>,
}

#[handler(command = "/set")]
async fn handle_set(context: &Context, command: Command) -> HandlerResult {
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
                context
                    .api
                    .execute(SendMessage::new(chat_id, err.to_string()))
                    .await
                    .unwrap();
                return HandlerResult::Stop;
            }
        }
    };
    let mut session = context.session_manager.get_session(&command);
    session.set("counter", &val).await.unwrap();
    context.api.execute(SendMessage::new(chat_id, "OK")).await.unwrap();
    HandlerResult::Stop
}

#[handler(command = "/expire")]
async fn handle_expire(context: &Context, command: Command) -> HandlerResult {
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
                context
                    .api
                    .execute(SendMessage::new(chat_id, err.to_string()))
                    .await
                    .unwrap();
                return HandlerResult::Stop;
            }
        }
    };
    let mut session = context.session_manager.get_session(&command);
    session.expire("counter", seconds).await.unwrap();
    context.api.execute(SendMessage::new(chat_id, "OK")).await.unwrap();
    HandlerResult::Stop
}

#[handler(command = "/reset")]
async fn handle_reset(context: &Context, command: Command) -> HandlerResult {
    log::info!("got a command: {:?}\n", command);
    let message = command.get_message();
    let chat_id = message.get_chat_id();
    let mut session = context.session_manager.get_session(&command);
    session.remove("counter").await.unwrap();
    context.api.execute(SendMessage::new(chat_id, "OK")).await.unwrap();
    HandlerResult::Stop
}

#[handler]
async fn handle_update(context: &Context, update: Update) -> HandlerResult {
    let message = update.get_message().unwrap();
    log::info!("got a message: {:?}\n", message);
    let chat_id = message.get_chat_id();
    let mut session = context.session_manager.get_session(&update);
    let val: Option<usize> = session.get("counter").await.unwrap();
    let val = val.unwrap_or(0) + 1;
    session.set("counter", &val).await.unwrap();
    let msg = format!("Count: {}", val);
    context.api.execute(SendMessage::new(chat_id, msg)).await.unwrap();
    HandlerResult::Continue
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();

    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }

    let api = Api::new(config).expect("Failed to create API");
    let tmpdir = tempdir().expect("Failed to create temp directory");
    log::info!("Session directory: {}", tmpdir.path().display());

    let backend = FilesystemBackend::new(tmpdir.path());

    // spawn GC to remove old sessions
    let gc_period = Duration::from_secs(1); // period between GC calls
    let session_lifetime = Duration::from_secs(1); // how long session lives
    let mut collector = SessionCollector::new(backend.clone(), gc_period, session_lifetime);
    tokio::spawn(async move { collector.run().await });

    let mut dispatcher = Dispatcher::new(Context {
        api: api.clone(),
        session_manager: SessionManager::new(backend),
    });
    dispatcher.add_handler(handle_expire);
    dispatcher.add_handler(handle_reset);
    dispatcher.add_handler(handle_set);
    dispatcher.add_handler(handle_update);
    LongPoll::new(api, dispatcher).run().await
}
