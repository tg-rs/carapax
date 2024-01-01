use crate::{core::handler::Handler, types::Command};

#[cfg(test)]
mod tests;

/// Allows to run a handler only for a specific command.
#[derive(Clone)]
pub struct CommandPredicate {
    name: String,
}

impl CommandPredicate {
    /// Creates a new `CommandPredicate`.
    ///
    /// # Arguments
    ///
    /// * `name` - A name of a command with leading `/`.
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }
}

impl Handler<Command> for CommandPredicate {
    type Output = bool;

    async fn handle(&self, input: Command) -> Self::Output {
        input.get_name() == self.name
    }
}
