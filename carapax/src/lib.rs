//! A Telegram Bot framework
#![warn(missing_docs)]

#[macro_use]
extern crate failure;

mod app;
mod dispatcher;
mod handler;

/// A "prelude" for users of the framework
pub mod prelude;

pub use self::{app::*, context::Context, dispatcher::*, handler::*};

pub use tgbot as core;

/// Context related objects
pub mod context;
