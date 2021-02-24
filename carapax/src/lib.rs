//! A Telegram Bot framework
#![cfg_attr(nightly, feature(doc_cfg, external_doc))]
#![warn(missing_docs)]

mod core;

pub use self::core::*;
pub use tgbot::{
    longpoll, methods, mime, types, webhook, Api, ApiError, Config, DownloadFileError, ExecuteError, ParseProxyError,
    UpdateHandler,
};

/// Access handler
#[cfg(feature = "access")]
#[cfg_attr(nightly, doc(cfg(feature = "access")))]
pub mod access;

/// Dialogue adapter
#[cfg(feature = "dialogue")]
#[cfg_attr(nightly, doc(cfg(feature = "dialogue")))]
pub mod dialogue;

/// i18n utilities
#[cfg(feature = "i18n")]
#[cfg_attr(nightly, doc(cfg(feature = "i18n")))]
pub mod i18n;

/// Ratelimit handler
#[cfg(feature = "ratelimit")]
#[cfg_attr(nightly, doc(cfg(feature = "ratelimit")))]
pub mod ratelimit;

/// Session manager
#[cfg(feature = "session")]
#[cfg_attr(nightly, doc(cfg(feature = "session")))]
pub mod session;
