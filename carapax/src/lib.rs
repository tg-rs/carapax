//! A Telegram Bot framework
#![warn(missing_docs)]

#[macro_use]
extern crate failure;

mod app;
mod dispatcher;
mod handler;

/// A convenience "prelude" for users of the framework
pub mod prelude;

pub use self::{app::*, dispatcher::*, handler::*};

pub use tgbot as core;

/// Context for handlers
pub mod context;
