use crate::{
    core::{FromUpdate, Handler, HandlerResult},
    session::SessionManager,
};
use async_trait::async_trait;
use seance::backend::SessionBackend;
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, fmt::Display, marker::PhantomData};
use tgbot::types::Update;

/// Mark an async function as dialogue handler
///
/// # Example
///
/// ```
/// use carapax::{dialogue::{DialogueResult, State, dialogue}, types::Message};
/// use serde::{Serialize, Deserialize};
/// use std::convert::Infallible;
///
/// #[derive(Serialize, Deserialize)]
/// enum ExampleState {
///     Start,
///     Step1,
///     Step2,
/// }
///
/// impl State for ExampleState {
///     fn new() -> Self {
///         ExampleState::Start
///     }
/// }
///
/// #[dialogue]
/// async fn handler(
///    state: ExampleState,
///    context: &(),
///    input: Message,
/// ) -> Result<DialogueResult<ExampleState>, Infallible> {
///     unimplemented!()
/// }
/// ```
pub use carapax_codegen::dialogue;

const SESSION_KEY_PREFIX: &str = "__carapax_dialogue";

/// Adapts dialogue handlers for [Dispatcher](../struct.Dispatcher.html)
pub struct Dialogue<C, B, H, S>
where
    H: DialogueHandler<C, S>,
    S: State,
{
    session_manager: SessionManager<B>,
    session_key: String,
    handler: H,
    _marker: PhantomData<(C, S)>,
}

impl<C, B, H, S> Dialogue<C, B, H, S>
where
    H: DialogueHandler<C, S>,
    S: State,
{
    /// Creates a new adapter
    ///
    /// # Arguments
    ///
    /// * session_manager - Session manager to store dialogue state
    /// * name - Key used to store dialogue
    /// * handler - Dialogue handler
    pub fn new<N>(session_manager: SessionManager<B>, name: N, handler: H) -> Self
    where
        N: Display,
    {
        Self {
            session_manager,
            session_key: format!("{}:{}", SESSION_KEY_PREFIX, name),
            handler,
            _marker: PhantomData,
        }
    }
}

/// Dialogue state
pub trait State: Serialize + DeserializeOwned {
    /// Returns initial state
    fn new() -> Self;
}

/// Dialogue handler
#[async_trait]
pub trait DialogueHandler<C, S> {
    /// An object to handle (Update, Message, Command, etc...)
    type Input: FromUpdate + Send + Sync;

    /// An error occurred in handler
    type Error: Error + Send + Sync;

    /// Handles an update
    ///
    /// # Arguments
    ///
    /// * state - State of dialogue
    /// * context - A context provided to dispatcher (same as in [Handler](../trait.Handler.html) trait)
    /// * input - An object to handle (same as in [Handler](../trait.Handler.html) trait)
    async fn handle(&mut self, state: S, context: &C, input: Self::Input) -> Result<DialogueResult<S>, Self::Error>;
}

/// Result of dialogue handler
#[derive(Debug)]
pub enum DialogueResult<S> {
    /// Next state
    Next(S),
    /// Exit from dialogue
    Exit,
}

#[async_trait]
impl<C, B, H, S> Handler<C> for Dialogue<C, B, H, S>
where
    C: Send + Sync,
    B: SessionBackend + Send,
    H: DialogueHandler<C, S> + Send,
    S: State + Send + Sync,
    <H as DialogueHandler<C, S>>::Error: 'static,
    <<H as DialogueHandler<C, S>>::Input as FromUpdate>::Error: 'static,
{
    type Input = Update;
    type Output = HandlerResult;

    async fn handle(&mut self, context: &C, input: Self::Input) -> Self::Output {
        let mut session = match self.session_manager.get_session(&input) {
            Ok(session) => session,
            Err(err) => return HandlerResult::error(err),
        };
        let input = match FromUpdate::from_update(input) {
            Ok(Some(input)) => input,
            Ok(None) => return HandlerResult::Continue,
            Err(err) => return HandlerResult::error(err),
        };

        let state: S = {
            match session.get(&self.session_key).await {
                Ok(Some(state)) => state,
                Ok(None) => State::new(),
                Err(err) => return HandlerResult::error(err),
            }
        };
        match self.handler.handle(state, context, input).await {
            Ok(DialogueResult::Next(state)) => {
                if let Err(err) = session.set(&self.session_key, &state).await {
                    return HandlerResult::error(err);
                }
            }
            Ok(DialogueResult::Exit) => {
                if let Err(err) = session.remove(&self.session_key).await {
                    return HandlerResult::error(err);
                };
            }
            Err(err) => {
                return HandlerResult::error(err);
            }
        }
        HandlerResult::Continue
    }
}

#[cfg(test)]
#[cfg(feature = "session-fs")]
mod tests {
    use super::*;
    use crate::session::backend::fs::FilesystemBackend;
    use serde::{Deserialize, Serialize};
    use std::convert::Infallible;
    use tempfile::tempdir;
    use tgbot::types::Message;

    struct MockDialogueHandler;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum MockState {
        Start,
        Stop,
    }

    impl State for MockState {
        fn new() -> Self {
            Self::Start
        }
    }

    struct Context;

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

    #[async_trait]
    impl DialogueHandler<Context, MockState> for MockDialogueHandler {
        type Input = Message;
        type Error = Infallible;

        async fn handle(
            &mut self,
            state: MockState,
            _context: &Context,
            _input: Self::Input,
        ) -> Result<DialogueResult<MockState>, Self::Error> {
            use self::DialogueResult::*;
            Ok(match state {
                MockState::Start => Next(MockState::Stop),
                MockState::Stop => Exit,
            })
        }
    }

    #[tokio::test]
    async fn dialogue() {
        let tmpdir = tempdir().expect("Failed to create temp directory");
        let session_backend = FilesystemBackend::new(tmpdir.path());
        let session_manager = SessionManager::new(session_backend);
        let name = "test";
        let mut dialogue = Dialogue::new(session_manager.clone(), name, MockDialogueHandler);
        let context = Context;
        let update = create_update();
        match dialogue.handle(&context, update.clone()).await {
            HandlerResult::Continue => {
                let mut session = session_manager.get_session(&update).unwrap();
                let key = format!("{}:{}", SESSION_KEY_PREFIX, name);
                let state: MockState = session
                    .get(key)
                    .await
                    .expect("Failed to get dialogue state")
                    .expect("Dialogue state is None");
                assert_eq!(state, MockState::Stop);
            }
            HandlerResult::Stop => panic!("Unexpected handler result"),
            HandlerResult::Error(err) => panic!("Dialogue error: {:?}", err),
        }
        match dialogue.handle(&context, update.clone()).await {
            HandlerResult::Continue => {
                let mut session = session_manager.get_session(&update).unwrap();
                let key = format!("{}:{}", SESSION_KEY_PREFIX, name);
                let state: Option<MockState> = session.get(key).await.expect("Failed to get dialogue state");
                assert!(state.is_none());
            }
            HandlerResult::Stop => panic!("Unexpected handler result"),
            HandlerResult::Error(err) => panic!("Dialogue error: {:?}", err),
        }
    }
}
