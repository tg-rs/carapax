//! A Telegram Bot API client library
#![warn(missing_docs)]

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate failure;

mod api;
mod executor;
mod poll;

/// Methods available in the Bot API
pub mod methods;

/// Types available in the Bot API
pub mod types;

/// Webhook support
pub mod webhook;

pub use self::{
    api::{Api, ApiFuture},
    poll::UpdatesStream,
};
