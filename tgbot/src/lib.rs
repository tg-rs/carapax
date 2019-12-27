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

/// Types available in the Bot API
pub mod types;

/// Services to receive updates via webhook
pub mod webhook;

pub use self::{
    api::{Api, ApiError, Config, DownloadFileError, ExecuteError, ParseProxyError},
    handler::UpdateHandler,
};

pub use async_trait::async_trait;
pub use mime;
