//! A Telegram Bot framework
#![warn(missing_docs)]

#[macro_use]
extern crate failure;

mod app;
mod context;

/// Updates dispatcher
pub mod dispatcher;

/// Access rules and policies
pub mod access;

/// Rate limit middleware
pub mod ratelimit;

/// A "prelude" for users of the framework
pub mod prelude;

pub use self::{
    app::{App, RunMethod},
    context::Context,
};
