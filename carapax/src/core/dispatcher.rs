use crate::{
    core::{
        handler::{BoxedHandler, Handler},
        result::{HandlerResult, HandlerResultError},
    },
    Data, FromUpdate, ServiceUpdate,
};
use futures_util::future::BoxFuture;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    future::Future,
    sync::Arc,
};
use tgbot::{types::Update, Api, UpdateHandler};

type BoxedErrorHandler = Box<dyn ErrorHandler<Future = BoxFuture<'static, ()>> + Send + Sync>;

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
    /// Use [`data_arc()`](Dispatcher::data_arc) if your data already wrapped in [`Arc`]
    pub fn data<T>(&mut self, value: T) -> &mut Self
    where
        T: Send + Sync + 'static,
    {
        let data = Arc::get_mut(&mut self.data).unwrap();
        data.push(Data::from(value));
        self
    }

    /// Adds a user data that already wrapped in [`Arc`]
    pub fn data_arc<T>(&mut self, value: Arc<T>) -> &mut Self
    where
        T: Send + Sync + 'static,
    {
        let data = Arc::get_mut(&mut self.data).unwrap();
        data.push(Data::from_arc(value));
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
        self.handlers.push(handler.boxed());
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
                    HandlerResult::Error(err) => {
                        error_handler.handle(err).await;
                        break;
                    }
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
    type Future: Future<Output = ()>;

    /// Handles a error
    ///
    /// This method is called on each error returned by a handler
    /// Next handler will not  process current update.
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
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, err: HandlerResultError) -> Self::Future {
        Box::pin(self.0.handle(err))
    }
}

/// A default error handler which logs error
pub struct LoggingErrorHandler(());

impl Default for LoggingErrorHandler {
    fn default() -> Self {
        Self(())
    }
}

impl ErrorHandler for LoggingErrorHandler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, err: HandlerResultError) -> Self::Future {
        Box::pin(async move {
            log::error!("An error has occurred: {}", err);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{error::Error, fmt};
    use tokio::sync::Mutex;

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
}
