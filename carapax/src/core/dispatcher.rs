use crate::core::convert::BoxedConvertFuture;
use crate::{
    core::{
        convert::ConvertHandler,
        handler::Handler,
        result::{HandlerResult, HandlerResultError},
    },
    Data, FromUpdate, ServiceUpdate,
};
use async_trait::async_trait;
use futures::future::BoxFuture;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Display;
use std::future::Future;
use std::sync::Arc;
use tgbot::{types::Update, Api, UpdateHandler};

type BoxedHandler = Box<dyn Handler<ServiceUpdate, BoxedConvertFuture> + Send>;
type BoxedErrorHandler = Box<dyn ErrorHandler + Send>;

/// A Telegram Update dispatcher
pub struct Dispatcher {
    api: Api,
    handlers: Vec<BoxedHandler>,
    //error_handler: BoxedErrorHandler,
    data: Arc<DispatcherData>,
}

impl Dispatcher {
    /// Creates a new Dispatcher
    pub fn new(api: Api) -> Self {
        Self {
            api,
            handlers: Vec::new(),
            //error_handler: Box::new(LoggingErrorHandler::default()),
            data: Arc::new(DispatcherData::default()),
        }
    }

    pub fn data<T: Send + Sync + 'static>(&mut self, value: impl Into<Data<T>>) -> &mut Self {
        let data = Arc::get_mut(&mut self.data).unwrap();
        data.insert(value.into());
        self
    }

    /// Adds a handler to dispatcher
    ///
    /// Handlers will be dispatched in the same order as they are added
    pub fn add_handler<H, T, R>(&mut self, handler: H) -> &mut Self
    where
        H: Handler<T, R> + Send + 'static,
        T: FromUpdate + Send + 'static,
        T::Error: Display,
        R: Future + Send + 'static,
        R::Output: Into<HandlerResult>,
    {
        self.handlers.push(ConvertHandler::boxed(handler));
        self
    }

    /// Sets a handler to be executed when an error has occurred
    ///
    /// Error handler will be called if one of update handlers returned
    /// [`HandlerResult::Error`](enum.HandlerResult.html)
    /*pub fn set_error_handler<H>(mut self, handler: H) -> Self
    where
        H: ErrorHandler + Send + 'static,
    {
        self.error_handler = Box::new(handler);
        self
    }*/

    /// Returns [`Api`]
    ///
    /// [`Api`]: tgbot::Api
    pub fn get_api(&self) -> Api {
        self.api.clone()
    }

    fn dispatch(&self, update: Update) -> impl Future<Output = ()> {
        let service_update = ServiceUpdate {
            update,
            api: self.api.clone(),
            data: self.data.clone(),
        };

        let futs: Vec<(&'static str, _)> = self
            .handlers
            .iter()
            .map(|h| (h.name(), h.call(service_update.clone())))
            .collect();

        async move {
            for (name, fut) in futs {
                log::debug!("Run {} handler", name);

                let result = fut.await;
                match result {
                    HandlerResult::Continue => continue,
                    HandlerResult::Stop => break,
                    HandlerResult::Error(err) => log::error!("{}", err), // TODO: error handling
                                                                         /*match self.error_handler.handle(err).await {
                                                                             ErrorPolicy::Continue => continue,
                                                                             ErrorPolicy::Stop => break,
                                                                         }*/
                }
            }
        }
    }
}

impl UpdateHandler for Dispatcher {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        Box::pin(self.dispatch(update))
    }
}

#[derive(Debug, Default)]
pub struct DispatcherData {
    inner: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl DispatcherData {
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.inner
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }

    pub fn insert<T: Send + Sync + 'static>(&mut self, value: T) {
        self.inner.insert(TypeId::of::<T>(), Box::new(value));
        // TODO: return old value
        //.and_then(|boxed| boxed.downcast().ok().map(|boxed| *boxed))
    }
}

/// A handler for errors occurred when dispatching update
#[async_trait]
pub trait ErrorHandler {
    /// Handles a error
    ///
    /// This method is called on each error returned by a handler
    /// [ErrorPolicy](enum.ErrorPolicy.html) defines
    /// whether next handler should process current update or not.
    async fn handle(&mut self, err: HandlerResultError) -> ErrorPolicy;
}

/// A default error handler which logs error
///
/// By default it stops propagation
/// (see [ErrorPolicy](enum.ErrorPolicy.html) for more information)
pub struct LoggingErrorHandler(ErrorPolicy);

impl LoggingErrorHandler {
    /// Creates a new logger error handler with a specified error policy
    pub fn new(error_policy: ErrorPolicy) -> Self {
        Self(error_policy)
    }
}

impl Default for LoggingErrorHandler {
    fn default() -> Self {
        Self::new(ErrorPolicy::Stop)
    }
}

#[async_trait]
impl ErrorHandler for LoggingErrorHandler {
    async fn handle(&mut self, err: HandlerResultError) -> ErrorPolicy {
        log::error!("An error has occurred: {}", err);
        self.0
    }
}

/// A policy for error handler
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ErrorPolicy {
    /// Continue propagation
    ///
    /// Next handler will run
    Continue,
    /// Stop propagation
    ///
    /// Next handler will not run
    Stop,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{error::Error, fmt};
    use tokio::sync::{
        oneshot::{channel, Sender},
        Mutex,
    };

    type Updates = Mutex<Vec<Update>>;

    struct HandlerMock {
        result: Option<HandlerResult>,
    }

    impl HandlerMock {
        fn new(result: HandlerResult) -> Self {
            Self { result: Some(result) }
        }

        fn with_continue() -> Self {
            Self::new(HandlerResult::Continue)
        }

        fn with_stop() -> Self {
            Self::new(HandlerResult::Stop)
        }

        fn with_error() -> Self {
            Self::new(HandlerResult::from(Err::<(), ErrorMock>(ErrorMock)))
        }
    }

    #[async_trait]
    impl Handler<Updates> for HandlerMock {
        type Input = Update;
        type Output = HandlerResult;

        async fn handle(&mut self, context: &Updates, input: Self::Input) -> Self::Output {
            context.lock().await.push(input);
            self.result.take().unwrap()
        }
    }

    #[derive(Debug)]
    struct ErrorMock;

    impl Error for ErrorMock {}

    impl fmt::Display for ErrorMock {
        fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
            write!(out, "Test error")
        }
    }

    fn create_update() -> Update {
        serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test message from private chat"
            }
        }))
        .unwrap()
    }

    #[tokio::test]
    async fn dispatch_default() {
        macro_rules! assert_dispatch {
            ($count:expr, $($handler:expr),*) => {{
                let updates = Mutex::new(Vec::new());
                let mut dispatcher = Dispatcher::new(updates);
                $(dispatcher.add_handler($handler);)*
                let update = create_update();
                dispatcher.dispatch(update).await;
                let context = dispatcher.context.lock().await;
                assert_eq!(context.len(), $count);
            }};
        }

        assert_dispatch!(
            2,
            HandlerMock::with_continue(),
            HandlerMock::with_stop(),
            HandlerMock::with_error()
        );

        assert_dispatch!(
            1,
            HandlerMock::with_stop(),
            HandlerMock::with_continue(),
            HandlerMock::with_error()
        );

        assert_dispatch!(
            1,
            HandlerMock::with_error(),
            HandlerMock::with_stop(),
            HandlerMock::with_continue()
        );
    }

    struct MockErrorHandler {
        error_policy: ErrorPolicy,
        sender: Option<Sender<HandlerResultError>>,
    }

    impl MockErrorHandler {
        fn new(error_policy: ErrorPolicy, sender: Sender<HandlerResultError>) -> Self {
            MockErrorHandler {
                error_policy,
                sender: Some(sender),
            }
        }
    }

    #[async_trait]
    impl ErrorHandler for MockErrorHandler {
        async fn handle(&mut self, err: HandlerResultError) -> ErrorPolicy {
            let sender = self.sender.take().unwrap();
            sender.send(err).unwrap();
            self.error_policy
        }
    }

    #[tokio::test]
    async fn dispatch_custom_error_handler() {
        let update = create_update();
        for (count, error_policy) in &[(1usize, ErrorPolicy::Stop), (2usize, ErrorPolicy::Continue)] {
            let mut dispatcher = Dispatcher::new(Mutex::new(Vec::new()));
            dispatcher.add_handler(HandlerMock::with_error());
            dispatcher.add_handler(HandlerMock::with_continue());
            let (tx, mut rx) = channel();
            dispatcher.set_error_handler(MockErrorHandler::new(*error_policy, tx));
            dispatcher.dispatch(update.clone()).await;
            rx.close();
            let context = dispatcher.context.lock().await;
            assert_eq!(context.len(), *count);
            assert!(rx.try_recv().is_ok());
        }
    }
}
