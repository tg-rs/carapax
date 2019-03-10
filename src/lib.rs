//! A Telegram Bot framework
#![warn(missing_docs)]

mod app;

/// Access rules and policies
pub mod access;

/// Rate limit middleware
pub mod ratelimit;

/// A "prelude" for users of the framework
pub mod prelude;
