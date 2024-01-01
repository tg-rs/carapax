use std::marker::PhantomData;

use seance::{backend::SessionBackend, Session};

use crate::{
    core::{Handler, HandlerError, HandlerInput, PredicateResult, TryFromInput},
    dialogue::state::DialogueState,
};

/// A predicate for dialogue
///
/// Allows to decide whether a dialogue should start or not.
/// The dialogue handler runs only when his state exists in a session
/// or when the inner predicate returns `true`.
pub struct DialoguePredicate<B, P, PI, HS> {
    session_backend: PhantomData<B>,
    predicate: P,
    predicate_input: PhantomData<PI>,
    handler_state: PhantomData<HS>,
}

impl<B, P, PI, HS> DialoguePredicate<B, P, PI, HS> {
    /// Creates a new `DialoguePredicate`.
    ///
    /// # Arguments
    ///
    /// * `predicate` - The inner predicate (e.g. command).
    pub fn new(predicate: P) -> Self {
        Self {
            session_backend: PhantomData,
            predicate,
            predicate_input: PhantomData,
            handler_state: PhantomData,
        }
    }
}

impl<B, P, PI, HS> Clone for DialoguePredicate<B, P, PI, HS>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        Self {
            session_backend: self.session_backend,
            predicate: self.predicate.clone(),
            predicate_input: self.predicate_input,
            handler_state: self.handler_state,
        }
    }
}

impl<B, P, PI, PO, HS> Handler<HandlerInput> for DialoguePredicate<B, P, PI, HS>
where
    B: SessionBackend + Send + Sync + 'static,
    P: Handler<PI, Output = PO> + Sync + 'static,
    PI: TryFromInput + Sync,
    PI::Error: 'static,
    PO: Into<PredicateResult>,
    HS: DialogueState + Send + Sync,
{
    type Output = PredicateResult;

    async fn handle(&self, input: HandlerInput) -> Self::Output {
        let mut session = match <Session<B>>::try_from_input(input.clone()).await {
            Ok(Some(session)) => session,
            Ok(None) => unreachable!("TryFromInput implementation for Session<B> never returns None"),
            Err(err) => return PredicateResult::Err(HandlerError::new(err)),
        };
        let session_key = HS::session_key();
        match session.get::<&str, HS>(&session_key).await {
            Ok(Some(_)) => {
                // We have dialogue state in session, so we must run dialog handler
                PredicateResult::True
            }
            Ok(None) => {
                // Dialogue state not found in session, let's check predicate
                match PI::try_from_input(input.clone()).await {
                    Ok(Some(predicate_input)) => self.predicate.handle(predicate_input).await.into(),
                    Ok(None) => PredicateResult::False,
                    Err(err) => PredicateResult::Err(HandlerError::new(err)),
                }
            }
            Err(err) => PredicateResult::Err(HandlerError::new(err)),
        }
    }
}
