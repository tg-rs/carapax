use crate::{
    context::Context,
    handler::{Handler, HandlerFuture, HandlerResult, HandlerWrapper},
};
use std::{collections::HashMap, string::FromUtf16Error};
use tgbot::types::Message;

/// A simple commands handler
///
/// Takes the first command from a message and ignores others.
///
/// Assumes that all text after the command is its arguments.
///
/// Use quotes in order to include spaces in argument: `'hello word'`
#[derive(Default)]
pub struct CommandsHandler {
    handlers: HashMap<String, BoxedCommandHandler>,
    not_found_handler: Option<BoxedCommandHandler>,
}

type BoxedCommandHandler = Box<CommandHandler<Output = HandlerFuture> + Send + Sync>;

impl CommandsHandler {
    /// Adds a command handler
    ///
    /// # Arguments
    ///
    /// - name - Command name (starts with `/`)
    /// - handler - Command handler
    pub fn add_handler<S, H, O>(mut self, name: S, handler: H) -> Self
    where
        S: Into<String>,
        H: CommandHandler<Output = O> + Send + Sync + 'static,
        O: Into<HandlerFuture>,
    {
        self.handlers.insert(name.into(), HandlerWrapper::boxed(handler));
        self
    }

    /// Adds a handler to be executed when the command is not found
    pub fn not_found_handler<H, O>(mut self, handler: H) -> Self
    where
        H: CommandHandler<Output = O> + Send + Sync + 'static,
        O: Into<HandlerFuture>,
    {
        self.not_found_handler = Some(HandlerWrapper::boxed(handler));
        self
    }
}

/// An error occurred when parsing command arguments
#[derive(Debug, Fail)]
pub enum CommandError {
    /// Can not decode command arguments
    #[fail(display = "Can not decode command arguments: {:?}", _0)]
    FromUtf16(#[cause] FromUtf16Error),
    /// Can not split arguments: quotes mismatched
    #[fail(display = "Can not split command arguments: quotes mismatched")]
    MismatchedQuotes,
}

impl From<FromUtf16Error> for CommandError {
    fn from(err: FromUtf16Error) -> Self {
        CommandError::FromUtf16(err)
    }
}

impl From<shellwords::MismatchedQuotes> for CommandError {
    fn from(_: shellwords::MismatchedQuotes) -> Self {
        CommandError::MismatchedQuotes
    }
}

impl Handler for CommandsHandler {
    type Input = Message;
    type Output = HandlerFuture;

    fn handle(&self, context: &mut Context, message: Self::Input) -> Self::Output {
        match (&message.commands, message.get_text()) {
            (Some(commands), Some(text)) => {
                // tgbot guarantees that commands will never be empty, but we must be sure
                assert!(!commands.is_empty());
                // just take first command and ignore others
                let command = &commands[0];
                // assume that all text after command is arguments
                let pos = command.data.offset + command.data.length;
                // pos is UTF-16 offset
                let input: Vec<u16> = text.data.encode_utf16().skip(pos).collect();
                let command = command.command.clone();
                let res = || -> Result<HandlerFuture, CommandError> {
                    let input = String::from_utf16(&input)?;
                    let args = shellwords::split(&input)?;
                    Ok(self
                        .handlers
                        .get(&command)
                        .or_else(|| self.not_found_handler.as_ref())
                        .map(|handler| handler.handle(context, message, args))
                        .unwrap_or_else(|| HandlerResult::Continue.into()))
                };
                match res() {
                    Ok(fut) => fut,
                    Err(err) => Err(err).into(),
                }
            }
            _ => HandlerResult::Continue.into(),
        }
    }
}

/// A command handler trait
pub trait CommandHandler {
    /// A handler's output.
    ///
    /// See [HandlerFuture](struct.HandlerFuture.html) for more information
    type Output: Into<HandlerFuture>;

    /// Handles the command
    ///
    /// # Arguments
    ///
    /// * context - A handler context
    /// * message - A message that triggered a command
    /// * args - List of arguments
    ///
    /// The context is valid for all handlers that accepts given update
    ///
    /// When processing a subsequent update, the old context will be destroyed and replaced with a new one
    fn handle(&self, context: &mut Context, message: Message, args: Vec<String>) -> Self::Output;
}

impl<H, O> CommandHandler for HandlerWrapper<H>
where
    H: CommandHandler<Output = O>,
    O: Into<HandlerFuture>,
{
    type Output = HandlerFuture;

    fn handle(&self, context: &mut Context, message: Message, args: Vec<String>) -> Self::Output {
        self.handler.handle(context, message, args).into()
    }
}

impl<F, O> CommandHandler for F
where
    F: Fn(&mut Context, Message, Vec<String>) -> O,
    O: Into<HandlerFuture>,
{
    type Output = O;

    fn handle(&self, context: &mut Context, message: Message, args: Vec<String>) -> Self::Output {
        (self)(context, message, args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler::FromUpdate;
    use futures::Future;

    struct Args {
        items: Vec<String>,
    }

    impl Args {
        fn new() -> Self {
            Self { items: vec![] }
        }

        fn extend(&mut self, items: Vec<String>) {
            self.items.extend(items);
        }
    }

    fn create_context() -> Context {
        let mut context = Context::default();
        context.set(Args::new());
        context
    }

    fn command_handler(context: &mut Context, _message: Message, args: Vec<String>) {
        context.get_mut::<Args>().extend(args);
    }

    #[test]
    fn commands_handler() {
        let message = Message::from_update(
            serde_json::from_value(serde_json::json!(
                {
                    "update_id": 1,
                    "message": {
                        "message_id": 1111,
                        "date": 0,
                        "from": {"id": 1, "is_bot": false, "first_name": "test"},
                        "chat": {"id": 1, "type": "private", "first_name": "test"},
                        "text": "/testcommand 'arg1 v' arg2",
                        "entities": [
                            {"type": "bot_command", "offset": 0, "length": 12}
                        ]
                    }
                }
            ))
            .unwrap(),
        )
        .unwrap();
        let handler = CommandsHandler::default().add_handler("/testcommand", command_handler);
        let mut context = create_context();
        assert_eq!(
            handler.handle(&mut context, message).wait().unwrap(),
            HandlerResult::Continue
        );
        let args = context.get::<Args>();
        assert_eq!(args.items, vec![String::from("arg1 v"), String::from("arg2")]);
    }
}
