use anymap::{
    any::{Any, IntoBox},
    Map,
};

/// Context for handlers
#[derive(Debug)]
pub struct Context {
    inner: Map<Any + Send + Sync>,
}

impl Default for Context {
    fn default() -> Self {
        Self { inner: Map::new() }
    }
}

impl Context {
    /// Sets a value to context
    pub fn set<T: IntoBox<Any + Send + Sync>>(&mut self, value: T) {
        self.inner.insert(value);
    }

    /// Returns a reference to value from context
    ///
    /// # Panics
    ///
    /// Panics if value not found
    pub fn get<T: IntoBox<Any + Send + Sync>>(&self) -> &T {
        self.inner.get().expect("Value not found in context")
    }

    /// Returns a reference to the value stored in context for the type T, if it exists
    pub fn get_opt<T: IntoBox<Any + Send + Sync>>(&self) -> Option<&T> {
        self.inner.get()
    }

    /// Returns a mutable reference to value from context
    ///
    /// # Panics
    ///
    /// Panics if value not found
    pub fn get_mut<T: IntoBox<Any + Send + Sync>>(&mut self) -> &mut T {
        self.inner.get_mut().expect("Value not found in context")
    }

    /// Returns a mutable reference to the value stored in context for the type T, if it exists
    pub fn get_mut_opt<T: IntoBox<Any + Send + Sync>>(&mut self) -> Option<&mut T> {
        self.inner.get_mut()
    }
}
