use crate::{
    core::{
        handler::{Handler, HandlerResult},
        predicate::PredicateResult,
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
