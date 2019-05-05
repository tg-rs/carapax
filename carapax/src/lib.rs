//! A Telegram Bot framework
#![warn(missing_docs)]

#[macro_use]
extern crate failure;

mod app;
mod dispatcher;
mod handler;

/// A "prelude" for users of the framework
pub mod prelude;

pub use self::{app::*, dispatcher::*, handler::*};

pub use tgbot as core;

/// Context for handler
pub mod context;
