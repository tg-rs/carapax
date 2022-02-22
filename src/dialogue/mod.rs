use crate::{
    core::{Handler, HandlerError, HandlerInput, HandlerResult, Predicate, PredicateResult, TryFromInput},
    session::CreateSessionError,
};
use futures_util::future::BoxFuture;
use seance::{backend::SessionBackend, Session, SessionError};
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, fmt, marker::PhantomData};

const SESSION_KEY_PREFIX: &str = "__carapax_dialogue";

/// A predicate for dialogue
///
/// Allows to decide, should dialogue start or not.
/// We keep a dialogue state in session.
/// Dialogue handler will run only when state exists, or inner predicate returned true.
pub struct DialoguePredicate<B, P, PI, HS> {
    session_backend: PhantomData<B>,
    predicate: P,
    predicate_input: PhantomData<PI>,
    handler_state: PhantomData<HS>,
}

impl<B, P, PI, HS> DialoguePredicate<B, P, PI, HS> {
    /// Creates a new predicate
    ///
    /// # Arguments
    ///
    /// * predicate - Inner predicate
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
    B: SessionBackend + Send + 'static,
    P: Handler<PI, Output = PO> + 'static,
    PI: TryFromInput,
    PI::Error: 'static,
    PO: Into<PredicateResult>,
    HS: DialogueState + Send + Sync,
{
    type Output = PredicateResult;
    type Future = BoxFuture<'static, Self::Output>;

    fn handle(&self, input: HandlerInput) -> Self::Future {
        let predicate = self.predicate.clone();
        Box::pin(async move {
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
                        Ok(Some(predicate_input)) => {
                            let predicate_future = predicate.handle(predicate_input);
                            predicate_future.await.into()
                        }
                        Ok(None) => PredicateResult::False,
                        Err(err) => PredicateResult::Err(HandlerError::new(err)),
                    }
                }
                Err(err) => PredicateResult::Err(HandlerError::new(err)),
            }
        })
    }
}

/// A decorator for dialogue handlers
pub struct DialogueDecorator<B, H, HI, HS> {
    session_backend: PhantomData<B>,
    handler: H,
    handler_input: PhantomData<HI>,
    handler_state: PhantomData<HS>,
}

impl<B, H, HI, HS> DialogueDecorator<B, H, HI, HS> {
    /// Creates a new DialogueDecorator
    ///
    /// # Arguments
    ///
    /// * handler - A dialogue handler
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
    B: SessionBackend + Send + 'static,
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
        let handler = self.handler.clone();
        Box::pin(async move {
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
    #[allow(clippy::type_complexity)]
    fn dialogue<B>(
        self,
        predicate: P,
    ) -> Predicate<DialoguePredicate<B, P, PI, HS>, HandlerInput, DialogueDecorator<B, Self, HI, HS>, HandlerInput>
    {
        Predicate::new(DialoguePredicate::new(predicate), DialogueDecorator::new(self))
    }
}

impl<P, PI, H, HI, HS> DialogueExt<P, PI, HI, HS> for H
where
    P: Handler<PI>,
    PI: TryFromInput,
    H: Handler<HI>,
    HI: TryFromInput,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{Chain, Context, PredicateOutput},
        session::{backend::fs::FilesystemBackend, SessionManager},
        types::Text,
    };
    use serde::Deserialize;
    use std::{convert::Infallible, sync::Arc};
    use tempfile::tempdir;

    #[derive(Clone, Copy, Deserialize, Serialize)]
    enum State {
        Start,
        Step,
    }

    impl Default for State {
        fn default() -> Self {
            Self::Start
        }
    }

    impl DialogueState for State {
        fn dialogue_name() -> &'static str {
            "test"
        }
    }

    type InputMock = DialogueInput<State, FilesystemBackend>;

    async fn dialogue_predicate(text: Text) -> bool {
        text.data == "start"
    }

    async fn dialogue_handler(input: InputMock) -> Result<DialogueResult<State>, Infallible> {
        Ok(match input.state {
            State::Start => State::Step.into(),
            State::Step => DialogueResult::Exit,
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

    fn create_context() -> Arc<Context> {
        let tmpdir = tempdir().expect("Failed to create temp directory");
        let backend = FilesystemBackend::new(tmpdir.path());
        let mut context = Context::default();
        let session_manager = SessionManager::new(backend);
        context.insert(session_manager);
        Arc::new(context)
    }

    async fn get_session(input: HandlerInput) -> Session<FilesystemBackend> {
        <Session<FilesystemBackend>>::try_from_input(input.clone())
            .await
            .expect("Failed to get session")
            .expect("Session is None")
    }

    #[tokio::test]
    async fn dialogue() {
        let context = create_context();
        let handler = dialogue_handler.dialogue::<FilesystemBackend>(dialogue_predicate);

        let input = create_input(context.clone(), "start");

        let mut session = get_session(input.clone()).await;
        let session_key = State::session_key();

        macro_rules! assert_state_matches {
            ($state:pat) => {{
                let state: Option<State> = session.get(&session_key).await.expect("Failed to get state");
                assert!(matches!(state, $state));
            }};
        }

        assert!(matches!(
            handler.handle((input.clone(), input)).await,
            PredicateOutput::True(Ok(()))
        ));
        assert_state_matches!(Some(State::Step));

        let input = create_input(context.clone(), "step");
        assert!(matches!(
            handler.handle((input.clone(), input)).await,
            PredicateOutput::True(Ok(()))
        ));
        assert_state_matches!(None);
    }

    async fn skip_handler(mut session: Session<FilesystemBackend>) {
        session
            .set("is_skipped", &true)
            .await
            .expect("Failed to set is_skipped key")
    }

    #[tokio::test]
    async fn dialogue_in_chain_skipped() {
        let context = create_context();
        let handler = dialogue_handler.dialogue::<FilesystemBackend>(dialogue_predicate);
        let chain = Chain::once().add(handler).add(skip_handler);
        let input = create_input(context.clone(), "skipped");
        let mut session = get_session(input.clone()).await;
        chain.handle(input).await.expect("Failed to run chain handler");
        let is_skipped: bool = session
            .get("is_skipped")
            .await
            .expect("Failed to get is_skipped key")
            .expect("is_skipped key is not set");
        assert!(is_skipped);
    }
}
