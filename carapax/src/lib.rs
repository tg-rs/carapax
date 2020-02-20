//! A Telegram Bot framework
#![warn(missing_docs)]

mod command;
mod convert;
mod dispatcher;
mod handler;

pub use self::{
    command::{Command, CommandDispatcher, CommandError},
    convert::TryFromUpdate,
    dispatcher::{Dispatcher, ErrorHandler, ErrorPolicy, LoggingErrorHandler},
    handler::{Handler, HandlerError, HandlerResult},
};
pub use async_trait::async_trait;
pub use tgbot::{
    longpoll, methods, mime, types, webhook, Api, ApiError, Config, DownloadFileError, ExecuteError, ParseProxyError,
    UpdateHandler,
};

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
/// use carapax::{Command, handler};
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
