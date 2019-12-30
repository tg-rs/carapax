#![warn(missing_docs)]
//! A session utilities for carapax

mod factory;
mod gc;
mod session;

pub use self::{
    factory::SessionFactory,
    gc::{run_gc, GarbageCollector},
    session::{Session, SessionKey, SessionLifetime},
};

/// Contains session store implementations
pub mod store;
