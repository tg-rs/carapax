use std::{error::Error, marker::PhantomData};

use seance::{backend::SessionBackend, Session};

use crate::{
    core::{Handler, HandlerError, HandlerInput, HandlerResult, TryFromInput},
    dialogue::{error::DialogueError, result::DialogueResult, state::DialogueState},
};

/// A decorator for dialogue handlers.
///
/// An inner handler must return a [`DialogueResult`].
/// The decorator will automatically load and save the state of the dialogue returned in the result.
///
/// Note that you need to register
/// a [`crate::session::SessionManager`] instance in the [`crate::Context`].
/// Otherwise the decorator will return an error indicating that
/// the session manager is not registered.
pub struct DialogueDecorator<B, H, HI, HS> {
    session_backend: PhantomData<B>,
    handler: H,
    handler_input: PhantomData<HI>,
    handler_state: PhantomData<HS>,
}

impl<B, H, HI, HS> DialogueDecorator<B, H, HI, HS> {
    /// Creates a new `DialogueDecorator`.
    ///
    /// # Arguments
    ///
    /// * `handler` - A dialogue handler.
    pub fn new(handler: H) -> Self {
        Self {
            session_backend: PhantomData,
            handler,
            handler_input: PhantomData,
            handler_state: PhantomData,
        }
    }
}

impl<B, H, HI, HS> Clone for DialogueDecorator<B, H, HI, HS>
where
    H: Clone,
{
    fn clone(&self) -> Self {
        DialogueDecorator {
            session_backend: PhantomData,
            handler: self.handler.clone(),
            handler_input: self.handler_input,
            handler_state: self.handler_state,
        }
    }
}

impl<B, H, HI, HR, HS, HE> Handler<HandlerInput> for DialogueDecorator<B, H, HI, HS>
where
    B: SessionBackend + Send + Sync + 'static,
    H: Handler<HI, Output = Result<HR, HE>> + Sync + 'static,
    HI: TryFromInput + Sync,
    HI::Error: 'static,
    HR: Into<DialogueResult<HS>>,
    HS: DialogueState + Send + Sync,
    HE: Error + Send + 'static,
{
    type Output = HandlerResult;

    async fn handle(&self, input: HandlerInput) -> Self::Output {
        let handler_input = match HI::try_from_input(input.clone()).await {
            Ok(Some(input)) => input,
            Ok(None) => return Err(HandlerError::new(DialogueError::ConvertHandlerInput)),
            Err(err) => return Err(HandlerError::new(err)),
        };
        let result = match self.handler.handle(handler_input).await {
            Ok(result) => result.into(),
            Err(err) => return Err(HandlerError::new(err)),
        };

        let mut session = match <Session<B>>::try_from_input(input).await {
            Ok(Some(session)) => session,
            Ok(None) => unreachable!("TryFromInput implementation for Session<B> never returns None"),
            Err(err) => return Err(HandlerError::new(err)),
        };
        let session_key = HS::session_key();

        match result {
            DialogueResult::Next(state) => {
                if let Err(err) = session.set(session_key, &state).await {
                    return Err(HandlerError::new(err));
                }
            }
            DialogueResult::Exit => {
                // Explicitly remove the state from the session to make sure that the dialog will not run again.
                if let Err(err) = session.remove(session_key).await {
                    return Err(HandlerError::new(err));
                }
            }
        }

        Ok(())
    }
}
