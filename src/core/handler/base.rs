use crate::{
    core::{context::Context, convert::TryFromInput},
    types::Update,
};
use futures_util::future::BoxFuture;
use std::{error::Error, future::Future, marker::PhantomData, sync::Arc};

/// Allows to handle an update
pub trait Handler<I>: Send
where
    I: TryFromInput,
{
    /// A future output returned by `handle` method
    ///
    /// You should use [HandlerResult](enum.HandlerResult.html)
    /// (or any type, which can be converted into it)
    /// if you want to use that handler in [Dispatcher](struct.Dispatcher.html)
    ///
    /// It is possible to use any other type, e.g. if you want to use it in a decorator.
    /// But finally you need to convert it into `HandlerResult`.
    type Output: Send;

    /// A future returned by `handle` method
    type Future: Future<Output = Self::Output> + Send;

    /// Handles a specific input
    ///
    /// # Arguments
    ///
    /// * input - An input to handle
    ///
    /// See [TryFromInput](trait.TryFromInput.html) trait implementations
    /// for a list of supported types
    fn handle(&self, input: I) -> Self::Future;
}

macro_rules! impl_fn {
    ($($I:ident),+) => {
        #[allow(non_snake_case)]
        impl<X, $($I,)+ R> Handler<($($I,)+)> for X
        where
            X: Fn($($I,)+) -> R + Send,
            ($($I,)+): TryFromInput,
            R: Future + Send,
            R::Output: Send
        {
            type Output = R::Output;
            type Future = R;

            fn handle(&self, ($($I,)+): ($($I,)+)) -> Self::Future {
                (self)($($I,)+)
            }
        }
    };
}

impl_fn!(A);
impl_fn!(A, B);
impl_fn!(A, B, C);
impl_fn!(A, B, C, D);
impl_fn!(A, B, C, D, E);
impl_fn!(A, B, C, D, E, F);
impl_fn!(A, B, C, D, E, F, G);
impl_fn!(A, B, C, D, E, F, G, H);
impl_fn!(A, B, C, D, E, F, G, H, I);
impl_fn!(A, B, C, D, E, F, G, H, I, J);

pub(in crate::core) type BoxedHandler =
    Box<dyn Handler<HandlerInput, Future = BoxedHandlerFuture, Output = HandlerResult>>;

pub(in crate::core) type BoxedHandlerFuture = BoxFuture<'static, HandlerResult>;

pub(in crate::core) struct ConvertHandler<H, I> {
    handler: H,
    input: PhantomData<I>,
}

impl<H, I> ConvertHandler<H, I> {
    pub(in crate::core) fn boxed(handler: H) -> Box<Self> {
        Box::new(Self {
            handler,
            input: PhantomData,
        })
    }
}

impl<H, I> Handler<HandlerInput> for ConvertHandler<H, I>
where
    H: Handler<I> + Clone + Sync + 'static,
    I: TryFromInput,
    I::Error: 'static,
    <H::Future as Future>::Output: Into<HandlerResult>,
{
    type Output = HandlerResult;
    type Future = BoxedHandlerFuture;

    fn handle(&self, input: HandlerInput) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move {
            match I::try_from_input(input).await {
                Ok(Some(input)) => handler.handle(input).await.into(),
                Ok(None) => HandlerResult::Continue,
                Err(err) => HandlerResult::Error(Box::new(err)),
            }
        })
    }
}

/// An input for a handler
#[derive(Clone, Debug)]
pub struct HandlerInput {
    /// An Update received from Telegram API
    pub update: Update,
    /// A context to share data betweeen handlers
    pub context: Arc<Context>,
}

impl From<Update> for HandlerInput {
    fn from(update: Update) -> Self {
        HandlerInput {
            update,
            context: Arc::new(Default::default()),
        }
    }
}

/// A result returned by a handler
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
    /// Next handler will run after current has finished.
    Error(HandlerError),
}

impl From<()> for HandlerResult {
    fn from(_: ()) -> Self {
        HandlerResult::Stop
    }
}

impl From<bool> for HandlerResult {
    fn from(flag: bool) -> Self {
        if flag {
            HandlerResult::Continue
        } else {
            HandlerResult::Stop
        }
    }
}

impl<T, E> From<Result<T, E>> for HandlerResult
where
    T: Into<HandlerResult>,
    E: Error + Send + 'static,
{
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(res) => res.into(),
            Err(err) => HandlerResult::Error(Box::new(err)),
        }
    }
}

/// An error returned by a handler
pub type HandlerError = Box<dyn Error + Send>;

/// Allows to process errors returned by handlers
pub trait ErrorHandler: Send {
    /// A future returned by `handle` method
    type Future: Future<Output = ()> + Send;

    /// Handles a errors
    ///
    /// # Arguments
    ///
    /// * err - An error to handle
    fn handle(&self, err: HandlerError) -> Self::Future;
}

pub(in crate::core) type BoxedErrorHandler = Box<dyn ErrorHandler<Future = BoxFuture<'static, ()>> + Sync>;

pub(in crate::core) struct ConvertErrorHandler<H>(H);

impl<H> ConvertErrorHandler<H> {
    pub(in crate::core) fn boxed(handler: H) -> Box<Self> {
        Box::new(Self(handler))
    }
}

impl<H> ErrorHandler for ConvertErrorHandler<H>
where
    H: ErrorHandler,
    H::Future: 'static,
{
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, err: HandlerError) -> Self::Future {
        Box::pin(self.0.handle(err))
    }
}

/// Writes an error to log
pub struct LoggingErrorHandler;

impl ErrorHandler for LoggingErrorHandler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, err: HandlerError) -> Self::Future {
        Box::pin(async move {
            log::error!("An error has occurred: {}", err);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{core::context::Ref, types::User};
    use std::{error::Error, fmt};

    #[derive(Debug)]
    struct ExampleError;

    impl Error for ExampleError {}

    impl fmt::Display for ExampleError {
        fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(out, "Example error")
        }
    }

    #[derive(Clone)]
    struct ServiceA;

    #[derive(Clone)]
    struct ServiceB;

    #[derive(Clone)]
    struct ServiceC;

    #[derive(Clone)]
    struct ServiceD;

    #[derive(Clone)]
    struct ServiceE;

    #[derive(Clone)]
    struct ServiceF;

    #[derive(Clone)]
    struct ServiceG;

    #[derive(Clone)]
    struct ServiceH;

    #[allow(clippy::too_many_arguments)]
    async fn example_handler(
        update: Update,
        user: User,
        _sa: Ref<ServiceA>,
        _sb: Ref<ServiceB>,
        _sc: Ref<ServiceC>,
        _sd: Ref<ServiceD>,
        _se: Ref<ServiceE>,
        _sf: Ref<ServiceF>,
        _sg: Ref<ServiceG>,
        _sh: Ref<ServiceH>,
    ) {
        assert_eq!(update.id, 1);
        assert_eq!(user.id, 1);
        assert_eq!(update.get_message().map(|message| message.id), Some(1111));
    }

    #[tokio::test]
    async fn handler() {
        let update: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "message": {
                    "message_id": 1111,
                    "date": 0,
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "private", "first_name": "test"},
                    "text": "test",
                }
            }
        ))
        .unwrap();
        let mut context = Context::default();
        context.insert(ServiceA);
        context.insert(ServiceB);
        context.insert(ServiceC);
        context.insert(ServiceD);
        context.insert(ServiceE);
        context.insert(ServiceF);
        context.insert(ServiceG);
        context.insert(ServiceH);
        let input = HandlerInput {
            update,
            context: Arc::new(context),
        };
        let handler: BoxedHandler = ConvertHandler::boxed(example_handler);
        handler.handle(input).await;
    }

    #[test]
    fn convert_input() {
        let update: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "message": {
                    "message_id": 1111,
                    "date": 0,
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "private", "first_name": "test"},
                    "text": "test",
                }
            }
        ))
        .unwrap();
        assert_eq!(HandlerInput::from(update).update.id, 1);
    }

    #[test]
    fn convert_handler_result() {
        assert!(matches!(HandlerResult::from(()), HandlerResult::Stop));
        assert!(matches!(HandlerResult::from(true), HandlerResult::Continue));
        assert!(matches!(HandlerResult::from(false), HandlerResult::Stop));
        assert!(matches!(
            HandlerResult::from(Ok::<(), ExampleError>(())),
            HandlerResult::Stop
        ));
        assert!(matches!(
            HandlerResult::from(Err::<(), ExampleError>(ExampleError)),
            HandlerResult::Error(_)
        ));
    }

    #[tokio::test]
    async fn error_handler() {
        let handler = ConvertErrorHandler::boxed(LoggingErrorHandler);
        handler.handle(Box::new(ExampleError)).await;
    }
}
