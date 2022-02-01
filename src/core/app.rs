use crate::{
    core::{
        context::Context,
        convert::TryFromInput,
        handler::{Handler, HandlerInput, IntoHandlerResult},
    },
    types::Update,
};
use futures_util::future::BoxFuture;
use std::{future::Future, marker::PhantomData, sync::Arc};
use tgbot::UpdateHandler;

/// The main entry point
///
/// Implements [UpdateHandler](trait.UpdateHandler.html) trait, so you can use it
/// in [LongPoll](longpoll/struct.LongPoll.html) struct
/// or [webhook::run_server](webhook/fn.run_server.html) function.
pub struct App<H, HI> {
    context: Arc<Context>,
    handler: H,
    handler_input: PhantomData<HI>,
}

impl<H, HI, HO> App<H, HI>
where
    H: Handler<HI, Output = HO>,
    HI: TryFromInput,
    HI::Error: 'static,
    HO: IntoHandlerResult,
{
    /// Creates a new App
    ///
    /// # Arguments
    ///
    /// * context - A context to share data between handlers
    /// * handler - A handler to process updates
    pub fn new(context: Context, handler: H) -> Self {
        Self {
            context: Arc::new(context),
            handler,
            handler_input: PhantomData,
        }
    }

    fn run(&self, update: Update) -> impl Future<Output = ()> {
        let input = HandlerInput {
            update,
            context: self.context.clone(),
        };
        let handler = self.handler.clone();
        async move {
            let input = match HI::try_from_input(input).await {
                Ok(Some(input)) => input,
                Ok(None) => return,
                Err(err) => {
                    log::error!("Failed to convert input: {}", err);
                    return;
                }
            };
            let future = handler.handle(input);
            if let Err(err) = future.await.into_result() {
                log::error!("An error has occurred: {}", err);
            }
        }
    }
}

impl<H, HI, HO> UpdateHandler for App<H, HI>
where
    H: Handler<HI, Output = HO> + 'static,
    HI: TryFromInput + 'static,
    HI::Error: 'static,
    HO: IntoHandlerResult + Send + 'static,
{
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        Box::pin(self.run(update))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{chain::Chain, context::Ref};
    use std::{error::Error, fmt};
    use tokio::sync::Mutex;

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

    #[derive(Clone)]
    struct Counter {
        value: Arc<Mutex<u8>>,
    }

    #[derive(Debug)]
    struct ExampleError;

    impl fmt::Display for ExampleError {
        fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
            write!(out, "Example error")
        }
    }

    impl Error for ExampleError {}

    async fn success_handler(counter: Ref<Counter>) {
        *counter.value.lock().await += 1;
    }

    async fn error_handler(counter: Ref<Counter>) -> Result<(), ExampleError> {
        *counter.value.lock().await += 1;
        Err(ExampleError)
    }

    #[tokio::test]
    async fn handle() {
        let counter = Counter {
            value: Arc::new(Mutex::new(0)),
        };

        let mut context = Context::default();
        context.insert(counter.clone());

        let chain = Chain::default().add(success_handler).add(error_handler);

        let app = App::new(context, chain);

        let update = create_update();
        app.handle(update).await;

        assert_eq!(*counter.value.lock().await, 2);
    }
}
