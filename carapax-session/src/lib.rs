#![warn(missing_docs)]
//! A session handler for carapax

mod gc;
mod handler;
mod session;

pub use self::{
    gc::{spawn_gc, GarbageCollector},
    handler::SessionHandler,
    session::{Session, SessionKey, SessionLifetime},
};

/// Contains session store implementations
pub mod store;
