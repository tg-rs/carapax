use crate::{
    core::{Handler, HandlerError, HandlerInput, HandlerResult, PredicateResult, TryFromInput},
    session::CreateSessionError,
};
use futures_util::future::BoxFuture;
use seance::{backend::SessionBackend, Session, SessionError};
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, fmt, marker::PhantomData};

const SESSION_KEY_PREFIX: &str = "__carapax_dialogue";

/// A decorator for dialogue handlers
pub struct DialogueDecorator<B, P, PI, H, HI, HS> {
    session_backend: PhantomData<B>,
    predicate: P,
    predicate_input: PhantomData<PI>,
    handler: H,
    handler_input: PhantomData<HI>,
    handler_state: PhantomData<HS>,
}

impl<B, P, PI, H, HI, HS> Clone for DialogueDecorator<B, P, PI, H, HI, HS>
where
    P: Clone,
    H: Clone,
{
    fn clone(&self) -> Self {
        DialogueDecorator {
            session_backend: self.session_backend,
            predicate: self.predicate.clone(),
            predicate_input: self.predicate_input,
            handler: self.handler.clone(),
            handler_input: self.handler_input,
            handler_state: self.handler_state,
        }
    }
}

impl<B, P, PI, H, HI, HS> DialogueDecorator<B, P, PI, H, HI, HS> {
    /// Creates a new DialogueDecorator
    ///
    /// # Arguments
    ///
    /// * predicate - A predicate which allows to decide, should we start dialogue or not
    /// * handler - A dialogue handler
    pub fn new(predicate: P, handler: H) -> Self {
        Self {
            session_backend: PhantomData,
            predicate,
            predicate_input: PhantomData,
            handler,
            handler_input: PhantomData,
            handler_state: PhantomData,
        }
    }
}

impl<B, P, PI, PO, H, HI, HR, HS, HE> Handler<HandlerInput> for DialogueDecorator<B, P, PI, H, HI, HS>
where
    B: SessionBackend + Send + 'static,
    P: Handler<PI, Output = PO> + 'static,
    PI: TryFromInput,
    PI::Error: 'static,
    PO: Into<PredicateResult>,
    H: Handler<HI, Output = Result<HR, HE>> + 'static,
    HI: TryFromInput,
    HI::Error: 'static,
    HR: Into<DialogueResult<HS>>,
    HS: DialogueState + Send + Sync,
    HE: Error + Send + 'static,
{
    type Output = HandlerResult;
    type Future = BoxFuture<'static, Self::Output>;

    fn handle(&self, input: HandlerInput) -> Self::Future {
        let predicate = self.predicate.clone();
        let handler = self.handler.clone();
        Box::pin(async move {
            let mut session = match <Session<B>>::try_from_input(input.clone()).await {
                Ok(Some(session)) => session,
                Ok(None) => unreachable!("TryFromInput implementation for Session<B> never returns None"),
                Err(err) => return Err(HandlerError::new(err)),
            };
            let session_key = HS::session_key();
            match session.get::<&str, HS>(&session_key).await {
                Ok(Some(_)) => { /* We have dialogue state in session, so we must run dialog handler */ }
                Ok(None) => {
                    // Dialogue state not found in session, let's check predicate
                    let predicate_input = match PI::try_from_input(input.clone()).await {
                        Ok(Some(input)) => input,
                        Ok(None) => return Ok(()),
                        Err(err) => return Err(HandlerError::new(err)),
                    };
                    let predicate_future = predicate.handle(predicate_input);
                    match predicate_future.await.into() {
                        PredicateResult::True => { /* Predicate returned true, so we must run dialog handler */ }
                        PredicateResult::False => return Ok(()),
                        PredicateResult::Err(err) => return Err(err),
                    }
                }
                Err(err) => return Err(HandlerError::new(err)),
            }

            let handler_input = match HI::try_from_input(input.clone()).await {
                Ok(Some(input)) => input,
                Ok(None) => return Err(HandlerError::new(DialogueError::ConvertHandlerInput)),
                Err(err) => return Err(HandlerError::new(err)),
            };
            let handler_future = handler.handle(handler_input);
            let result = match handler_future.await {
                Ok(result) => result.into(),
                Err(err) => return Err(HandlerError::new(err)),
            };

            match result {
                DialogueResult::Next(state) => {
                    if let Err(err) = session.set(session_key, &state).await {
                        return Err(HandlerError::new(err));
                    }
                }
                DialogueResult::Exit => {
                    // Explicitly remove state from session in order to be sure that dialog will not run again
                    if let Err(err) = session.remove(session_key).await {
                        return Err(HandlerError::new(err));
                    }
                }
            }

            Ok(())
        })
    }
}

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
    type Error = DialogueError;
    type Future = BoxFuture<'static, Result<Option<Self>, Self::Error>>;

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

/// Result of dialogue handler
#[derive(Debug)]
pub enum DialogueResult<S> {
    /// Next state
    Next(S),
    /// Exit from dialogue
    Exit,
}

impl<S> From<S> for DialogueResult<S>
where
    S: DialogueState,
{
    fn from(state: S) -> Self {
        DialogueResult::Next(state)
    }
}

/// Represents a state of dialogue
pub trait DialogueState: Default + DeserializeOwned + Serialize {
    /// Unique name of dialogue
    fn dialogue_name() -> &'static str;

    /// A key to store state in session
    fn session_key() -> String {
        format!("{}:{}", SESSION_KEY_PREFIX, Self::dialogue_name())
    }
}

/// An error when processing dialogue
#[derive(Debug)]
pub enum DialogueError {
    /// Could not get input for dialogue handler
    ConvertHandlerInput,
    /// Failed to create session
    CreateSession(CreateSessionError),
    /// Failed to load dialogue state
    LoadState(SessionError),
}

impl From<CreateSessionError> for DialogueError {
    fn from(err: CreateSessionError) -> Self {
        DialogueError::CreateSession(err)
    }
}

impl fmt::Display for DialogueError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::DialogueError::*;
        match self {
            ConvertHandlerInput => write!(out, "Could not obtain input for dialogue handler"),
            CreateSession(err) => write!(out, "{}", err),
            LoadState(err) => write!(out, "Failed to load dialogue state: {}", err),
        }
    }
}

impl Error for DialogueError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::DialogueError::*;
        match self {
            ConvertHandlerInput => None,
            CreateSession(err) => Some(err),
            LoadState(err) => Some(err),
        }
    }
}

/// Dialogue shortcuts
pub trait DialogueExt<P, PI, HI, HS>: Sized {
    /// Shortcut to create a new dialogue decorator (`handler.dialogue(predicate)`)
    ///
    /// # Arguments
    ///
    /// * predicate - A predicate for dialogue
    fn dialogue<B>(self, predicate: P) -> DialogueDecorator<B, P, PI, Self, HI, HS> {
        DialogueDecorator::new(predicate, self)
    }
}

impl<P, PI, H, HI, HS> DialogueExt<P, PI, HI, HS> for H
where
    H: Handler<HI>,
    HI: TryFromInput,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::Context,
        session::{backend::fs::FilesystemBackend, SessionManager},
        types::Text,
    };
    use serde::Deserialize;
    use std::{convert::Infallible, sync::Arc};
    use tempfile::tempdir;

    #[derive(Clone, Copy, Deserialize, Serialize)]
    enum StateMock {
        Start,
        Step,
    }

    impl Default for StateMock {
        fn default() -> Self {
            Self::Start
        }
    }

    impl DialogueState for StateMock {
        fn dialogue_name() -> &'static str {
            "mock"
        }
    }

    type InputMock = DialogueInput<StateMock, FilesystemBackend>;

    async fn dialogue_predicate(text: Text) -> bool {
        text.data == "start"
    }

    async fn dialogue_handler(input: InputMock) -> Result<DialogueResult<StateMock>, Infallible> {
        Ok(match input.state {
            StateMock::Start => StateMock::Step.into(),
            StateMock::Step => DialogueResult::Exit,
        })
    }

    fn create_input(context: Arc<Context>, text: &str) -> HandlerInput {
        let update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": text
            }
        }))
        .unwrap();
        HandlerInput { context, update }
    }

    #[tokio::test]
    async fn dialogue() {
        let tmpdir = tempdir().expect("Failed to create temp directory");
        let backend = FilesystemBackend::new(tmpdir.path());
        let mut context = Context::default();
        let session_manager = SessionManager::new(backend);
        context.insert(session_manager.clone());
        let context = Arc::new(context);
        let handler = dialogue_handler.dialogue::<FilesystemBackend>(dialogue_predicate);

        let input = create_input(context.clone(), "start");

        let mut session = <Session<FilesystemBackend>>::try_from_input(input.clone())
            .await
            .expect("Failed to get session")
            .expect("Session is None");
        let session_key = StateMock::session_key();

        macro_rules! assert_state_matches {
            ($state:pat) => {{
                let state: Option<StateMock> = session.get(&session_key).await.expect("Failed to get state");
                assert!(matches!(state, $state));
            }};
        }

        assert!(matches!(handler.handle(input).await, Ok(())));
        assert_state_matches!(Some(StateMock::Step));

        let input = create_input(context.clone(), "step");
        assert!(matches!(handler.handle(input).await, Ok(())));
        assert_state_matches!(None);
    }
}
