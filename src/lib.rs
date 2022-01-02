//! A Telegram Bot framework
#![cfg_attr(nightly, feature(doc_cfg))]
#![warn(missing_docs)]

mod core;

pub use self::core::*;
pub use tgbot::*;

// /// Access handler
// #[cfg(feature = "access")]
// #[cfg_attr(nightly, doc(cfg(feature = "access")))]
// pub mod access;

// /// Dialogue adapter
// #[cfg(feature = "dialogue")]
// #[cfg_attr(nightly, doc(cfg(feature = "dialogue")))]
// pub mod dialogue;

// /// Ratelimit handler
// #[cfg(feature = "ratelimit")]
// #[cfg_attr(nightly, doc(cfg(feature = "ratelimit")))]
// pub mod ratelimit;

// /// Session manager
// #[cfg(feature = "session")]
// #[cfg_attr(nightly, doc(cfg(feature = "session")))]
// pub mod session;
