//! A Telegram Bot framework
#![warn(missing_docs)]

#[macro_use]
extern crate failure;

mod app;
mod context;
mod dispatcher;
mod handler;

/// A "prelude" for users of the framework
pub mod prelude;

pub use self::{app::*, context::*, dispatcher::*, handler::*};

pub use tgbot as core;
