use std::error::Error;

/// An error returned by handler
pub type HandlerError = Box<dyn Error + Send>;

/// Result of a handler
#[derive(Debug)]
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
    Error(HandlerError),
}

impl HandlerResult {
    /// Creates an error result
    pub fn error<E>(err: E) -> Self
    where
        E: Error + Send + 'static,
    {
        HandlerResult::Error(Box::new(err))
    }
}

impl From<()> for HandlerResult {
    fn from(_: ()) -> Self {
        HandlerResult::Continue
    }
}

impl<T, E> From<Result<T, E>> for HandlerResult
where
    T: Into<HandlerResult>,
    E: Error + Send + Sync + 'static,
{
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(res) => res.into(),
            Err(err) => HandlerResult::error(err),
        }
    }
}
