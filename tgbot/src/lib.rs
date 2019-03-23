//! A Telegram Bot API client library
#![warn(missing_docs)]
#![recursion_limit = "128"]

mod api;
mod executor;
mod handler;
mod never;

/// Methods available in the Bot API
pub mod methods;

/// Types available in the Bot API
pub mod types;

/// A "prelude" for users of the library
pub mod prelude;

pub use self::{api::*, handler::*};
use never::Never;
