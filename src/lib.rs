//! A Telegram Bot framework
#![warn(missing_docs)]

#[macro_use]
extern crate failure;

mod app;
mod dispatcher;
mod handler;
mod middleware;

/// A "prelude" for users of the framework
pub mod prelude;

pub use self::{app::*, dispatcher::*, handler::*, middleware::*};
