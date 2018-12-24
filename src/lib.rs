//! A Telegram Bot API client library
#![warn(missing_docs)]

#[macro_use]
extern crate failure;

mod client;
mod poll;

/// Methods available in the Bot API
pub mod methods;

/// Types available in the Bot API
pub mod types;

pub use self::client::{Client, ClientError};
pub use self::poll::UpdatesIter;
