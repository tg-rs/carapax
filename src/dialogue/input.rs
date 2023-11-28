use std::marker::PhantomData;

use futures_util::future::BoxFuture;
use seance::{backend::SessionBackend, Session};

use crate::{
    core::{HandlerInput, TryFromInput},
    dialogue::{error::DialogueError, state::DialogueState},
};

/// Input for dialogue handler
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
    type Future = BoxFuture<'static, Result<Option<Self>, Self::Error>>;
    type Error = DialogueError;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        Box::pin(async move {
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
        })
    }
}
