use std::{future::Future, marker::PhantomData, sync::Arc};

use futures_util::future::BoxFuture;

use crate::{
    core::{
        context::Context,
        convert::TryFromInput,
        handler::{Handler, HandlerInput, IntoHandlerResult},
    },
    handler::UpdateHandler,
    types::Update,
};

#[cfg(test)]
mod tests;

/// The main entry point
///
/// Implements [`UpdateHandler`] trait, so you can use it
/// in [`LongPoll`] or [tgbot::handler::WebhookServer].
#[derive(Clone)]
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
