use crate::{
    core::{
        convert::TryFromInput,
        handler::Handler,
        predicate::{base::Predicate, command::CommandPredicate},
    },
    types::Command,
};

/// Predicate shortcuts
pub trait PredicateExt<P, PI, HI>: Sized {
    /// Shortcut to create a new predicate decorator (`handler.predicate(predicate)`)
    ///
    /// # Arguments
    ///
    /// * predicate - A predicate handler
    fn predicate(self, predicate: P) -> Predicate<P, PI, Self, HI> {
        Predicate::new(predicate, self)
    }
}

impl<P, PI, H, HI> PredicateExt<P, PI, HI> for H
where
    H: Handler<HI>,
    HI: TryFromInput,
{
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
