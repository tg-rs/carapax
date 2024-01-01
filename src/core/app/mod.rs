use std::{future::Future, marker::PhantomData, sync::Arc};

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

/// The main entry point.
///
/// Implements the [`UpdateHandler`] trait, so you can use it
/// in [`crate::handler::LongPoll`] or [crate::handler::WebhookServer].
///
/// Wraps an update into the [`HandlerInput`] struct and passes it to the inner handler.
///
/// Use [`crate::Chain`] struct to configure multiple handlers.
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
    /// Creates a new `App`.
    ///
    /// # Arguments
    ///
    /// * `context` - A context responsible for storing shared state.
    /// * `handler` - A handler responsible for processing updates.
    pub fn new(context: Context, handler: H) -> Self {
        Self {
            context: Arc::new(context),
            handler,
            handler_input: PhantomData,
        }
    }

    fn handle_update(&self, update: Update) -> impl Future<Output = ()> {
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
    H: Handler<HI, Output = HO> + Sync + 'static,
    HI: TryFromInput + Sync + 'static,
    HI::Error: 'static,
    HO: IntoHandlerResult + Send + 'static,
{
    async fn handle(&self, update: Update) {
        self.handle_update(update).await
    }
}
