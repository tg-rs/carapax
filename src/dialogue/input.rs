use std::marker::PhantomData;

use seance::{backend::SessionBackend, Session};

use crate::{
    core::{HandlerInput, TryFromInput},
    dialogue::{error::DialogueError, state::DialogueState},
};

/// Represents an input for a dialogue handler.
///
/// The input provides access to the dialogue state.
/// When included in a list of handler arguments,
/// [`TryFromInput`] will automatically handle the extraction of the input.
#[derive(Clone)]
pub struct DialogueInput<S, B>
where
    S: DialogueState,
    B: SessionBackend,
{
    /// Dialogue state
    pub state: S,
    session_backend: PhantomData<B>,
}

impl<S, B> TryFromInput for DialogueInput<S, B>
where
    S: DialogueState + Send,
    B: SessionBackend + Send + 'static,
{
    type Error = DialogueError;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        match <Session<B>>::try_from_input(input.clone()).await? {
            Some(ref mut session) => {
                let session_key = S::session_key();
                let state = session
                    .get(session_key)
                    .await
                    .map_err(DialogueError::LoadState)?
                    .unwrap_or_default();
                Ok(Some(Self {
                    state,
                    session_backend: PhantomData,
                }))
            }
            None => unreachable!("TryFromInput implementation for Session<B> never returns None"),
        }
    }
}
