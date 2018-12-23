//! A Telegram Bot API client library
#![warn(missing_docs)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

/// Methods available in the Bot API
pub mod methods;

/// Types available in the Bot API
pub mod types;
