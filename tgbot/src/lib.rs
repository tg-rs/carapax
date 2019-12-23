//! A Telegram Bot API client library
#![recursion_limit = "256"]
#![warn(missing_docs)]

mod api;
mod handler;
mod request;

/// Utilities to receive updates using long poll
pub mod longpoll;

/// Methods available in the Bot API
pub mod methods;

/// A "prelude" for users of the library
pub mod prelude;

/// Types available in the Bot API
pub mod types;

/// Services to receive updates via webhook
pub mod webhook;

pub use self::{api::*, handler::*};

pub use mime;
