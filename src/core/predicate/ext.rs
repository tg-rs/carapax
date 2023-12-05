use crate::{
    core::{
        convert::TryFromInput,
        handler::Handler,
        predicate::{base::Predicate, command::CommandPredicate},
    },
    types::Command,
};

/// Provides a shortcut for wrapping a [`Handler`] by a [`Predicate`].
pub trait PredicateExt<P, PI, HI>: Sized {
    /// Shortcut to create a wrap a [`Handler`] with a [`Predicate`].
    ///
    /// Example: `handler.predicate(predicate)`.
    ///
    /// # Arguments
    ///
    /// * `predicate` - A predicate handler.
    fn with_predicate(self, predicate: P) -> Predicate<P, PI, Self, HI> {
        Predicate::new(predicate, self)
    }
}

impl<P, PI, H, HI> PredicateExt<P, PI, HI> for H
where
    H: Handler<HI>,
    HI: TryFromInput,
{
}

/// Provides a shortcut for wrapping a [`Handler`] by a [`CommandPredicate`].
pub trait CommandExt<I>: Sized {
    /// Shortcut to create a command handler.
    ///
    /// Example: `handler.command("/name")`.
    ///
    /// # Arguments
    ///
    /// * `name` - A name of a command with leading `/`.
    fn with_command<S: Into<String>>(self, name: S) -> Predicate<CommandPredicate, Command, Self, I> {
        Predicate::new(CommandPredicate::new(name), self)
    }
}

impl<H, I> CommandExt<I> for H
where
    H: Handler<I>,
    I: TryFromInput,
{
}
