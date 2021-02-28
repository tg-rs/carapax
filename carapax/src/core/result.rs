use std::fmt;

/// An error returned by handler
pub type HandlerResultError = Box<dyn std::error::Error + Send>;

/// Result of a handler
pub enum HandlerResult {
    /// Continue propagation
    ///
    /// Next handler (if exists) will run after current has finished
    Continue,
    /// Stop propagation
    ///
    /// Next handler (if exists) will not run after current has finished
    Stop,
    /// An error has occurred
    ///
    /// This error will be passed to [ErrorHandler](trait.ErrorHandler.html).
    /// If error handler returned [ErrorPolicy::Continue](enum.ErrorPolicy.html),
    /// next handler will run after current has finished
    /// For `ErrorPolicy::Stop` next handler will not run (default behavior).
    Error(HandlerResultError),
}

impl HandlerResult {
    /// Creates an error result
    pub fn error<E>(err: E) -> Self
    where
        E: std::error::Error + Send + 'static,
    {
        HandlerResult::Error(Box::new(err))
    }
}

impl From<()> for HandlerResult {
    fn from(_: ()) -> Self {
        // default behavior is stop because we continue propagation
        // when handler's arguments (types implementing FromUpdate) return None
        HandlerResult::Stop
    }
}

impl<T, E> From<Result<T, E>> for HandlerResult
where
    T: Into<HandlerResult>,
    E: std::error::Error + Send + 'static,
{
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(res) => res.into(),
            Err(err) => HandlerResult::error(err),
        }
    }
}

impl fmt::Debug for HandlerResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HandlerResult::Continue => f.debug_tuple("Continue").finish(),
            HandlerResult::Stop => f.debug_tuple("Stop").finish(),
            HandlerResult::Error(err) => f.debug_tuple("Error").field(&format_args!("{}", err)).finish(),
        }
    }
}
