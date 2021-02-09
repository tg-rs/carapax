//! A Telegram Bot framework
#![cfg_attr(nightly, feature(doc_cfg, external_doc))]
// TODO: uncomment
//#![warn(missing_docs)]

pub use async_trait::async_trait;
/// Mark an async function as update handler
///
/// If called without arguments it simply will implement [Handler](trait.Handler.html) for given function.
/// Example:
/// ```
/// use carapax::{types::Update, handler};
///
/// #[handler]
/// async fn handle_update(context: &(), update: Update) {
///     println!("Got update: {:?}", update);
/// }
/// ```
///
/// Use `#[handler(command="/name")]` when you need to handle a command.
///
/// Example:
/// ```
/// use carapax::{types::Command, handler};
///
/// #[handler(command = "/start")]
/// async fn handle_start(context: &(), command: Command) {
///     println!("Got command: {:?}", command);
/// }
/// ```
///
/// You also can set a predicate for handler using `#[handler(predicate=path::to::func)]`.
/// `func` should return a boolean value determining whether handler should run or not.
///
/// Example:
/// ```
/// use carapax::{methods::SendMessage, types::Message, handler, Api, ExecuteError};
/// use std::convert::Infallible;
///
/// async fn is_ping(_context: &Api, message: &Message) -> Result<bool, Infallible> {
///     Ok(message.get_text().map(|text| text.data == "ping").unwrap_or(false))
/// }
///
/// // Handler will not run if message text not equals "ping"
/// #[handler(predicate=is_ping)]
/// async fn pingpong_handler(context: &Api, message: Message) -> Result<(), ExecuteError> {
///     let chat_id = message.get_chat_id();
///     let method = SendMessage::new(chat_id, "pong");
///     context.execute(method).await?;
///     Ok(())
/// }
/// ```
pub use carapax_codegen::handler;
pub use tgbot::{
    longpoll, methods, mime, types, webhook, Api, ApiError, Config, DownloadFileError, ExecuteError, ParseProxyError,
    UpdateHandler,
};

pub use self::core::*;

mod core;

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

mod app;
pub use app::App;
