use anymap::{
    any::{Any, IntoBox},
    Map,
};

/// Context for handlers
pub struct Context {
    inner: Map<Any + Send + Sync>,
}

impl Default for Context {
    fn default() -> Self {
        Self { inner: Map::new() }
    }
}

impl Context {
    /// Adds a value to context
    pub fn add<T: IntoBox<Any + Send + Sync>>(&mut self, value: T) {
        self.inner.insert(value);
    }

    /// Get a value from context
    ///
    /// # Panics
    ///
    /// Panics if value not found
    pub fn get<T: IntoBox<Any + Send + Sync>>(&self) -> &T {
        self.inner.get().expect("Value not found in context")
    }

    /// Get a value from context
    ///
    /// Returns a reference to the value stored in context for the type T, if it exists
    pub fn get_opt<T: IntoBox<Any + Send + Sync>>(&self) -> Option<&T> {
        self.inner.get()
    }
}
