use crate::{
    core::{
        convert::{BoxedConvertFuture, ConvertHandler},
        handler::Handler,
        result::{HandlerResult, HandlerResultError},
    },
    Data, FromUpdate, ServiceUpdate,
};
use futures::future::BoxFuture;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    future::Future,
    sync::Arc,
};
use tgbot::{types::Update, Api, UpdateHandler};

pub(crate) type BoxedHandler = Box<dyn Handler<ServiceUpdate, BoxedConvertFuture> + Send>;
type BoxedErrorHandler = Box<dyn ErrorHandler<Future = BoxFuture<'static, ErrorPolicy>> + Send + Sync>;

/// A Telegram Update dispatcher
pub struct Dispatcher {
    api: Api,
    handlers: Vec<BoxedHandler>,
    error_handler: Arc<BoxedErrorHandler>,
    data: Arc<DispatcherData>,
}

impl Dispatcher {
    /// Creates a new Dispatcher
    pub fn new(api: Api) -> Self {
        Self {
            api,
            handlers: Vec::new(),
            error_handler: Arc::new(Box::new(LoggingErrorHandler::default())),
            data: Arc::new(DispatcherData::default()),
        }
    }

    /// Adds a user data wrapping it into [`Arc`]
    ///
    /// If your data is already wrapped into [`Arc`] it will be automatically taken into account
    /// because of [`impl From<Arc<T>> for Data<T>`]
    ///
    /// [`impl From<Arc<T>> for Data<T>`]: Data#impl-From<Arc<T>>
    pub fn data<T: Send + Sync + 'static>(&mut self, value: impl Into<Data<T>>) -> &mut Self {
        let data = Arc::get_mut(&mut self.data).unwrap();
        data.push(value.into());
        self
    }

    /// Adds a handler to dispatcher
    ///
    /// Handlers will be dispatched in the same order as they are added
    pub fn add_handler<H, T, R>(&mut self, handler: H) -> &mut Self
    where
        H: Handler<T, R> + Send + Clone + 'static,
        T: FromUpdate + Send + 'static,
        T::Error: std::error::Error,
        T::Future: Send,
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
    pub fn set_error_handler<H>(&mut self, handler: H) -> &mut Self
    where
        H: ErrorHandler + Send + Sync + 'static,
        H::Future: Send,
    {
        self.error_handler = Arc::new(ConvertErrorHandler::boxed(handler));
        self
    }

    #[allow(missing_docs)]
    pub fn get_api(&self) -> Api {
        self.api.clone()
    }

    fn dispatch(&self, update: Update) -> impl Future<Output = ()> + Send {
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

        let error_handler = self.error_handler.clone();

        async move {
            for (name, fut) in futs {
                log::debug!("Run {} handler", name);

                let result = fut.await;
                match result {
                    HandlerResult::Continue => continue,
                    HandlerResult::Stop => break,
                    HandlerResult::Error(err) => match error_handler.handle(err).await {
                        ErrorPolicy::Continue => continue,
                        ErrorPolicy::Stop => break,
                    },
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

    pub fn push<T: Send + Sync + 'static>(&mut self, value: T) {
        self.inner.insert(TypeId::of::<T>(), Box::new(value));
        // TODO: return old value
        //.and_then(|boxed| boxed.downcast().ok().map(|boxed| *boxed))
    }
}

/// A handler for errors occurred when dispatching update
pub trait ErrorHandler {
    /// A future returned from error handler
    type Future: Future<Output = ErrorPolicy>;

    /// Handles a error
    ///
    /// This method is called on each error returned by a handler
    /// [ErrorPolicy](enum.ErrorPolicy.html) defines
    /// whether next handler should process current update or not.
    fn handle(&self, err: HandlerResultError) -> Self::Future;
}

struct ConvertErrorHandler<H>(H);

impl<H> ConvertErrorHandler<H> {
    fn boxed(handler: H) -> Box<Self> {
        Box::new(Self(handler))
    }
}

impl<H> ErrorHandler for ConvertErrorHandler<H>
where
    H: ErrorHandler,
    H::Future: Send + 'static,
{
    type Future = BoxFuture<'static, ErrorPolicy>;

    fn handle(&self, err: HandlerResultError) -> Self::Future {
        Box::pin(self.0.handle(err))
    }
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

impl ErrorHandler for LoggingErrorHandler {
    type Future = BoxFuture<'static, ErrorPolicy>;

    fn handle(&self, err: HandlerResultError) -> Self::Future {
        let ret = self.0;
        Box::pin(async move {
            log::error!("An error has occurred: {}", err);
            ret
        })
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

    async fn handler_with_continue(updates: Data<Updates>, update: Update) -> HandlerResult {
        updates.lock().await.push(update);
        HandlerResult::Continue
    }

    async fn handler_with_stop(updates: Data<Updates>, update: Update) -> HandlerResult {
        updates.lock().await.push(update);
        HandlerResult::Stop
    }

    async fn handler_with_error(updates: Data<Updates>, update: Update) -> HandlerResult {
        updates.lock().await.push(update);
        HandlerResult::error(ErrorMock)
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
                let api = Api::new("123").unwrap();
                let updates = Mutex::new(Vec::<Update>::new());
                let mut dispatcher = Dispatcher::new(api);
                dispatcher.data(updates);
                $(dispatcher.add_handler($handler);)*
                let update = create_update();
                dispatcher.dispatch(update).await;
                let context = dispatcher.data.get::<Data<Updates>>().unwrap().lock().await;
                assert_eq!(context.len(), $count);
            }};
        }

        assert_dispatch!(2, handler_with_continue, handler_with_stop, handler_with_error);

        assert_dispatch!(1, handler_with_stop, handler_with_continue, handler_with_error);

        assert_dispatch!(1, handler_with_error, handler_with_stop, handler_with_continue);
    }

    struct MockErrorHandler {
        error_policy: ErrorPolicy,
        sender: Arc<Mutex<Option<Sender<HandlerResultError>>>>,
    }

    impl MockErrorHandler {
        fn new(error_policy: ErrorPolicy, sender: Sender<HandlerResultError>) -> Self {
            MockErrorHandler {
                error_policy,
                sender: Arc::new(Mutex::new(Some(sender))),
            }
        }
    }

    impl ErrorHandler for MockErrorHandler {
        type Future = BoxFuture<'static, ErrorPolicy>;

        fn handle(&self, err: HandlerResultError) -> Self::Future {
            let sender = self.sender.clone();
            let error_policy = self.error_policy;
            Box::pin(async move {
                let sender = sender.lock().await.take().unwrap();
                sender.send(err).unwrap();
                error_policy
            })
        }
    }

    #[tokio::test]
    async fn dispatch_custom_error_handler() {
        let update = create_update();
        for (count, error_policy) in &[(1usize, ErrorPolicy::Stop), (2usize, ErrorPolicy::Continue)] {
            let mut dispatcher = Dispatcher::new(Api::new("123").unwrap());
            dispatcher.data(Mutex::new(Vec::<Update>::new()));
            dispatcher.add_handler(handler_with_error);
            dispatcher.add_handler(handler_with_continue);
            let (tx, mut rx) = channel();
            dispatcher.set_error_handler(MockErrorHandler::new(*error_policy, tx));
            dispatcher.dispatch(update.clone()).await;
            rx.close();
            let context = dispatcher.data.get::<Data<Updates>>().unwrap();
            let context = context.lock().await;
            assert_eq!(context.len(), *count);
            assert!(rx.try_recv().is_ok());
        }
    }
}
