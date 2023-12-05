//! A Telegram Bot framework
#![cfg_attr(nightly, feature(doc_cfg))]

pub use tgbot::*;

pub use self::core::*;

mod core;

/// Access control
#[cfg(feature = "access")]
#[cfg_attr(nightly, doc(cfg(feature = "access")))]
pub mod access;

/// Dialogue support
#[cfg(feature = "dialogue")]
#[cfg_attr(nightly, doc(cfg(feature = "dialogue")))]
pub mod dialogue;

/// Ratelimit support
#[cfg(feature = "ratelimit")]
#[cfg_attr(nightly, doc(cfg(feature = "ratelimit")))]
pub mod ratelimit;

/// Session support
#[cfg(feature = "session")]
#[cfg_attr(nightly, doc(cfg(feature = "session")))]
pub mod session;
