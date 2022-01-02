use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::Deref,
};

/// Allows to share values of any type between handlers
#[derive(Debug, Default)]
pub struct Context {
    items: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Context {
    /// Returns an immutable reference to the value of type `T`
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.items
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }

    /// Inserts a value of type `T` to the context
    ///
    /// Returns a previously inserted object if exists
    pub fn insert<T: Send + Sync + 'static>(&mut self, value: T) -> Option<T> {
        self.items
            .insert(TypeId::of::<T>(), Box::new(value))
            .and_then(|boxed| <Box<dyn Any + 'static>>::downcast(boxed).ok().map(|boxed| *boxed))
    }
}

/// A reference for a value in [Context](struct.Context.html)
///
/// Thanks to [TryFromInput](trait.TryFromInput.html) trait
/// you can use `Ref<T>` as type of an argument in your handlers.
#[derive(Clone)]
pub struct Ref<T: Clone>(T);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct X;

    #[test]
    fn context() {
        let mut context = Context::default();
        let x = X;
        assert!(context.insert(x).is_none());
        assert!(context.get::<X>().is_some());
        assert!(context.insert(x).is_some());
    }

    #[test]
    fn reference() {
        let x = X;
        let ref_x = Ref::new(x);
        assert_eq!(x, *ref_x);
    }
}
