use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::Deref,
};

#[cfg(test)]
mod tests;

/// A shared state storage for use in [`crate::Handler`] trait implementations.
#[derive(Debug, Default)]
pub struct Context {
    items: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Context {
    /// Returns an immutable reference to the value of type `T`.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.items
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }

    /// Inserts a value of type `T` into the context.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to insert.
    ///
    /// Returns a previously inserted value if it exists.
    pub fn insert<T: Send + Sync + 'static>(&mut self, value: T) -> Option<T> {
        self.items
            .insert(TypeId::of::<T>(), Box::new(value))
            .and_then(|boxed| <Box<dyn Any + 'static>>::downcast(boxed).ok().map(|boxed| *boxed))
    }
}

/// A link to a value of type `T` stored in the [`Context`].
///
/// The link implements [`crate::TryFromInput`] trait,
/// enabling you to use `Ref<T>` as the type of an argument in your
/// [`crate::Handler`] trait implementations.
///
/// Keep in mind that each time a handler is called with `Ref<T>` as an argument,
/// the underlying value is cloned.
#[derive(Clone)]
pub struct Ref<T: Clone>(pub T);

impl<T: Clone> Ref<T> {
    pub(super) fn new(object: T) -> Self {
        Self(object)
    }
}

impl<T: Clone> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
