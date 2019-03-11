/// Context for handlers
pub struct Context<S> {
    pub(crate) inner: S,
}

impl Default for Context<()> {
    fn default() -> Self {
        Self { inner: () }
    }
}

impl<S> From<S> for Context<S> {
    fn from(inner: S) -> Self {
        Self { inner }
    }
}

impl<S> Context<S> {
    /// Return inner state
    pub fn get(&self) -> &S {
        &self.inner
    }

    /// Return inner state as mutable
    pub fn get_mut(&mut self) -> &mut S {
        &mut self.inner
    }
}
