use crate::{
    access::predicate::AccessPredicate,
    core::{Handler, HandlerInput, Predicate, TryFromInput},
};

/// Provides a shortcut for wrapping a [`Handler`] by an [`AccessPredicate`].
pub trait AccessExt<P, HI>: Sized {
    /// Shortcut to wrap a [`Handler`] with an access predicate.
    ///
    /// Example: `let handler = handler.access(policy)`.
    ///
    /// # Arguments
    ///
    /// * `policy` - A [`crate::access::AccessPolicy`].
    fn with_access_policy(self, policy: P) -> Predicate<AccessPredicate<P>, HandlerInput, Self, HI> {
        Predicate::new(AccessPredicate::new(policy), self)
    }
}

impl<P, H, HI> AccessExt<P, HI> for H
where
    H: Handler<HI>,
    HI: TryFromInput,
{
}
