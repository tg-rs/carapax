use crate::{
    core::{Handler, HandlerInput, HandlerResult, TryFromInput},
    session::CreateSessionError,
};
use futures_util::future::BoxFuture;
use seance::{backend::SessionBackend, Session, SessionError};
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, fmt, marker::PhantomData};

const SESSION_KEY_PREFIX: &str = "__carapax_dialogue";

/// A decorator for dialogue handlers
#[derive(Clone)]
pub struct DialogueDecorator<B, H, HI> {
    session_backend: PhantomData<B>,
    handler: H,
    handler_input: PhantomData<HI>,
}

impl<B, H, HI> DialogueDecorator<B, H, HI> {
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
        }
    }
}

async fn handle_dialogue<B, H, HI, HO, HE>(handler: H, input: HandlerInput) -> Result<(), DialogueError>
where
    B: SessionBackend + Send + 'static,
    H: Handler<HI, Output = Result<HO, HE>>,
    HI: TryFromInput,
    HI::Error: 'static,
    HO: DialogueState,
    HE: Error + Send + 'static,
{
    let handler_input = match HI::try_from_input(input.clone())
        .await
        .map_err(|err| DialogueError::ConvertInput(Box::new(err)))?
    {
        Some(input) => input,
        None => return Ok(()),
    };
    let handler_future = handler.handle(handler_input);
    let state = handler_future
        .await
        .map_err(|err| DialogueError::Handle(Box::new(err)))?;
    let mut session = <Session<B>>::try_from_input(input)
        .await?
        .expect("TryFromInput implementation for Session<B> never returns None");
    let session_key = HO::session_key();
    session
        .set(session_key, &state)
        .await
        .map_err(DialogueError::SaveState)?;
    Ok(())
}

impl<B, H, HI, HO, HE> Handler<HandlerInput> for DialogueDecorator<B, H, HI>
where
    H: Handler<HI, Output = Result<HO, HE>> + Clone + 'static,
    HI: TryFromInput + Clone,
    HI::Error: 'static,
    HO: DialogueState + Send + Sync,
    HE: Error + Send + 'static,
    B: SessionBackend + Clone + Send + 'static,
{
    type Output = HandlerResult;
    type Future = BoxFuture<'static, Self::Output>;

    fn handle(&self, input: HandlerInput) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move {
            let future = handle_dialogue::<B, H, HI, HO, HE>(handler, input);
            match future.await {
                Ok(()) => HandlerResult::Continue,
                Err(err) => HandlerResult::Error(Box::new(err)),
            }
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
    ConvertInput(Box<dyn Error + Send>),
    /// Failed to create session
    CreateSession(CreateSessionError),
    /// Contains an error returned from dialogue handler
    Handle(Box<dyn Error + Send>),
    /// Failed to load dialogue state
    LoadState(SessionError),
    /// Failed to save dialogue state
    SaveState(SessionError),
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
            ConvertInput(err) => write!(out, "Failed to convert input for dialogue handler: {}", err),
            CreateSession(err) => write!(out, "{}", err),
            Handle(err) => write!(out, "Dialgoue handler: {}", err),
            LoadState(err) => write!(out, "Failed to load dialogue state: {}", err),
            SaveState(err) => write!(out, "Failed to save dialogue state: {}", err),
        }
    }
}

impl Error for DialogueError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::DialogueError::*;
        match self {
            CreateSession(err) => Some(err),
            LoadState(err) => Some(err),
            SaveState(err) => Some(err),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::Context,
        session::{backend::fs::FilesystemBackend, SessionManager},
        types::Update,
    };
    use serde::Deserialize;
    use std::{convert::Infallible, sync::Arc};
    use tempfile::tempdir;

    #[derive(Clone, Copy, Deserialize, Serialize)]
    enum StateMock {
        Start,
        Stop,
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

    async fn dialogue_handler(input: InputMock) -> Result<StateMock, Infallible> {
        Ok(match input.state {
            StateMock::Start => StateMock::Stop,
            StateMock::Stop => StateMock::Start,
        })
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
    async fn dialogue() {
        let tmpdir = tempdir().expect("Failed to create temp directory");
        let backend = FilesystemBackend::new(tmpdir.path());
        let mut context = Context::default();
        let session_manager = SessionManager::new(backend);
        context.insert(session_manager.clone());
        let handler = <DialogueDecorator<FilesystemBackend, _, _>>::new(dialogue_handler);
        let input = HandlerInput {
            context: Arc::new(context),
            update: create_update(),
        };
        assert!(matches!(handler.handle(input.clone()).await, HandlerResult::Continue));

        let mut session = <Session<FilesystemBackend>>::try_from_input(input.clone())
            .await
            .expect("Failed to get session")
            .expect("Session is None");
        let state: StateMock = session
            .get(StateMock::session_key())
            .await
            .expect("Failed to get state")
            .expect("State is None");
        assert!(matches!(state, StateMock::Stop));

        assert!(matches!(handler.handle(input.clone()).await, HandlerResult::Continue));

        let state: StateMock = session
            .get(StateMock::session_key())
            .await
            .expect("Failed to get state")
            .expect("State is None");
        assert!(matches!(state, StateMock::Start));
    }
}
