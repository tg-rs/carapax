use futures_util::future::{ready, Ready};

use crate::{core::handler::Handler, types::Command};

#[cfg(test)]
mod tests;

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
    type Output = bool;
    type Future = Ready<Self::Output>;

    fn handle(&self, input: Command) -> Self::Future {
        ready(input.get_name() == self.name)
    }
}
