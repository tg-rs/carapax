use failure::Error;
use futures::{future, Future};

/// Result of a handler
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerResult {
    /// Continue propagation
    ///
    /// Next handler (if exists) will run after current has finished
    Continue,
    /// Stop propagation
    ///
    /// Next handler (if exists) will not run after current has finished
    Stop,
}

/// A future that drives a handler's execution.
///
/// # Example
///
/// ```
/// use futures::future;
/// use failure::Fail;
/// use carapax::prelude::*;
///
/// #[derive(Debug, Fail)]
/// #[fail(display = "Example error")]
/// struct Error;
///
/// fn handler() {
///    // You can create a future in the following ways:
///    let a: HandlerFuture = HandlerResult::Continue.into();
///    let b = HandlerFuture::new(future::ok(HandlerResult::Continue));
///    let c: HandlerFuture = ().into(); // Item is HandlerResult::Continue
///    let b: HandlerFuture = Ok::<_, Error>(HandlerResult::Continue).into();
///    let c: HandlerFuture = Err(Error).into();
/// }
#[must_use = "futures do nothing unless polled"]
pub struct HandlerFuture {
    inner: Box<Future<Item = HandlerResult, Error = Error> + Send>,
}

impl HandlerFuture {
    /// Creates a new handler future from another future
    pub fn new<F>(f: F) -> HandlerFuture
    where
        F: Future<Item = HandlerResult, Error = Error> + Send + 'static,
    {
        HandlerFuture { inner: Box::new(f) }
    }
}

impl From<HandlerResult> for HandlerFuture {
    fn from(result: HandlerResult) -> HandlerFuture {
        HandlerFuture::new(future::ok(result))
    }
}

impl From<()> for HandlerFuture {
    fn from(_: ()) -> Self {
        Self::from(HandlerResult::Continue)
    }
}

impl<E> From<Result<HandlerResult, E>> for HandlerFuture
where
    E: Into<Error>,
{
    fn from(result: Result<HandlerResult, E>) -> Self {
        HandlerFuture::new(future::result(result.map_err(Into::into)))
    }
}

impl Future for HandlerFuture {
    type Item = HandlerResult;
    type Error = Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Fail)]
    #[fail(display = "test handler future")]
    struct Error;

    #[test]
    fn convert_to_future() {
        assert_eq!(
            HandlerFuture::from(HandlerResult::Continue).wait().unwrap(),
            HandlerResult::Continue
        );
        assert_eq!(
            HandlerFuture::from(HandlerResult::Stop).wait().unwrap(),
            HandlerResult::Stop
        );
        assert_eq!(HandlerFuture::from(()).wait().unwrap(), HandlerResult::Continue);
        assert_eq!(
            HandlerFuture::from(Ok::<_, Error>(HandlerResult::Continue))
                .wait()
                .unwrap(),
            HandlerResult::Continue
        );
        assert_eq!(
            HandlerFuture::from(Err(Error)).wait().unwrap_err().to_string(),
            "test handler future"
        );
    }
}
