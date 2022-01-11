use carapax::{Context, Dispatcher};
use seance::{backend::fs::FilesystemBackend, SessionCollector, SessionManager};
use std::{env, time::Duration};
use tempfile::tempdir;

pub trait Module {
    fn add_handlers(&self, _dispatcher: &mut Dispatcher) {}
}

pub fn get_env(s: &str) -> String {
    env::var(s).unwrap_or_else(|_| panic!("{} is not set", s))
}

pub fn insert_fs_backend(context: &mut Context) {
    let backend = backend_with_tmpdir();
    spawn_collector(backend.clone());

    let manager = SessionManager::new(backend);
    context.insert(manager);
}

fn backend_with_tmpdir() -> FilesystemBackend {
    let tmpdir = tempdir().expect("Failed to create temp directory");
    log::info!("Session directory: {}", tmpdir.path().display());
    let backend = FilesystemBackend::new(tmpdir.path());
    backend
}

fn spawn_collector(backend: FilesystemBackend) {
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
