use crate::{
    access::predicate::AccessPredicate,
    core::{Handler, HandlerInput, Predicate, TryFromInput},
};

/// Access shortcuts
pub trait AccessExt<P, HI>: Sized {
    /// Shortcut to wrap a handler with access predicate (`handler.access(policy)`)
    ///
    /// # Arguments
    ///
    /// * policy - An access policy
    fn access(self, policy: P) -> Predicate<AccessPredicate<P>, HandlerInput, Self, HI> {
        Predicate::new(AccessPredicate::new(policy), self)
    }
}

impl<P, H, HI> AccessExt<P, HI> for H
where
    H: Handler<HI>,
    HI: TryFromInput,
{
}
