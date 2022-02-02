use carapax::{longpoll::LongPoll, Api, App, Chain, Context, ErrorExt, HandlerError};
use dotenv::dotenv;
use seance::{backend::fs::FilesystemBackend, SessionCollector, SessionManager};
use std::{env, time::Duration};
use tempfile::tempdir;

mod access;
mod command;
mod dialogue;
mod error;
mod predicate;
mod ratelimit;
mod session;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let api = Api::new(get_env("CARAPAX_TOKEN")).expect("Failed to create API");

    let mut context = Context::default();
    context.insert(api.clone());

    let session_backend = create_session_backend();
    spawn_session_collector(session_backend.clone());

    let session_manager = SessionManager::new(session_backend);
    context.insert(session_manager);

    let mut chain = Chain::default();
    if let Ok(username) = env::var("CARAPAX_ACCESS_USERNAME") {
        chain = access::setup(chain, &username);
    }
    chain = command::setup(chain);
    chain = dialogue::setup(chain);
    chain = predicate::setup(chain);
    chain = session::setup(chain);

    if let Ok(ratelimit_strategy) = env::var("CARAPAX_RATE_LIMIT_STRATEGY") {
        chain = ratelimit::setup(chain, &ratelimit_strategy);
    }

    let handler = chain.on_error(error_handler);

    let app = App::new(context, handler);
    LongPoll::new(api, app).run().await
}

fn get_env(s: &str) -> String {
    env::var(s).unwrap_or_else(|_| panic!("{} is not set", s))
}

fn create_session_backend() -> FilesystemBackend {
    let tmpdir = tempdir().expect("Failed to create temp directory");
    log::info!("Session directory: {}", tmpdir.path().display());
    let backend = FilesystemBackend::new(tmpdir.path());
    backend
}

fn spawn_session_collector(backend: FilesystemBackend) {
    let gc_period = get_env("CARAPAX_SESSION_GC_PERIOD");
    let gc_period = Duration::from_secs(
        gc_period
            .parse::<u64>()
            .expect("CARAPAX_SESSION_GC_PERIOD must be integer"),
    ); // period between GC calls

    let session_lifetime = get_env("CARAPAX_SESSION_LIFETIME");
    let session_lifetime = Duration::from_secs(
        session_lifetime
            .parse::<u64>()
            .expect("CARAPAX_SESSION_LIFETIME must be integer"),
    ); // how long session lives

    // spawn GC to remove old sessions
    let mut collector = SessionCollector::new(backend, gc_period, session_lifetime);
    tokio::spawn(async move { collector.run().await });
}

async fn error_handler(err: HandlerError) -> HandlerError {
    log::error!("Got an error in custom error handler: {}", err);
    err
}
