use crate::{
    core::{
        convert::TryFromInput,
        handler::{
            predicate::{Predicate, PredicateResult},
            Handler, HandlerResult,
        },
    },
    types::Command,
};
use futures_util::future::{ready, Ready};

/// Allows to run a handler for a specific command
#[derive(Clone)]
pub struct CommandPredicate {
    name: String,
}

impl CommandPredicate {
    /// Creates a new CommandPredicate
    ///
    /// # Arguments
    ///
    /// * name - Command name with leading `/`
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }
}

impl Handler<Command> for CommandPredicate {
    type Output = PredicateResult;
    type Future = Ready<Self::Output>;

    fn handle(&self, input: Command) -> Self::Future {
        ready(if input.get_name() == self.name {
            PredicateResult::True
        } else {
            PredicateResult::False(HandlerResult::Continue)
        })
    }
}

/// Command shortcuts
pub trait CommandExt<I>: Sized {
    /// Shortcut to create a command handler (`handler.command("/name")`)
    ///
    /// # Arguments
    ///
    /// * name - Command name with leading `/`
    fn command<S: Into<String>>(self, name: S) -> Predicate<CommandPredicate, Command, Self, I> {
        Predicate::new(CommandPredicate::new(name), self)
    }
}

impl<H, I> CommandExt<I> for H
where
    H: Handler<I>,
    I: TryFromInput,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Message;

    fn create_command(command: &str) -> Command {
        let len = command.len();
        let message: Message = serde_json::from_value(serde_json::json!(
            {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": command,
                "entities": [
                    {"type": "bot_command", "offset": 0, "length": len}
                ]
            }
        ))
        .unwrap();
        Command::try_from(message).unwrap()
    }

    #[tokio::test]
    async fn command_predicate() {
        let handler = CommandPredicate::new("/start");
        assert!(matches!(
            handler.handle(create_command("/start")).await,
            PredicateResult::True
        ));
        assert!(matches!(
            handler.handle(create_command("/unexpected")).await,
            PredicateResult::False(HandlerResult::Continue)
        ));
    }
}
