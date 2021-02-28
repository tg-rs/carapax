use crate::{
    core::FromUpdate,
    session::{backend::SessionBackend, Session, SessionError},
    Handler, HandlerResult, ServiceUpdate,
};
use futures_util::future::BoxFuture;
use serde::{de::DeserializeOwned, Serialize};
use std::{convert::Infallible, error::Error, future::Future, marker::PhantomData};

const SESSION_KEY_PREFIX: &str = "__carapax_dialogue";

/// See [`Handler::dialogue`](crate::Handler::dialogue)
pub struct Dialogue<S, B> {
    /// The user's state itself
    pub state: S,
    _b: PhantomData<B>,
}

impl<S, B> FromUpdate for Dialogue<S, B>
where
    S: State,
    B: SessionBackend + Send + 'static,
{
    type Error = SessionError;

    type Future = BoxFuture<'static, Result<Option<Self>, Self::Error>>;

    fn from_update(service_update: ServiceUpdate) -> Self::Future {
        Box::pin(async move {
            let mut session = Session::<B>::from_update(service_update)
                .await?
                .expect("Session::from_update always returns Some");
            let state: S = session.get(S::session_key()).await?.unwrap_or_default();
            Ok(Some(Dialogue { state, _b: PhantomData }))
        })
    }
}

/// Dialogue state
pub trait State: Default + Serialize + DeserializeOwned {
    /// Name of session to be used to identify dialog session
    fn session_name() -> &'static str;

    /// Key for [session](Session)
    ///
    /// By default, it is "__carapax_dialogue:" + [`session_name()`](State::session_name)
    fn session_key() -> String {
        format!("{}:{}", SESSION_KEY_PREFIX, Self::session_name())
    }
}

/// Result of dialogue handler
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DialogueState<S> {
    /// Next state
    Next(S),
    /// Dialogue session key is removed when this variant is passed
    Exit,
}

/// This utility trait is used to process what user's dialogue handler returned
pub trait DialogueResult<B> {
    #[allow(missing_docs)]
    type Future: Future<Output = HandlerResult>;

    #[allow(missing_docs)]
    fn process_handler(self, session: Session<B>) -> Self::Future;
}

impl<S, B> DialogueResult<B> for DialogueState<S>
where
    S: State + Send + Sync + 'static,
    B: SessionBackend + Send + 'static,
{
    type Future = BoxFuture<'static, HandlerResult>;

    fn process_handler(self, session: Session<B>) -> Self::Future {
        Result::<DialogueState<S>, Infallible>::process_handler(Ok(self), session)
    }
}

impl<S, B, E> DialogueResult<B> for Result<DialogueState<S>, E>
where
    E: Error + Send + 'static,
    S: State + Send + Sync + 'static, // FIXME: get rid of Sync
    B: SessionBackend + Send + 'static,
{
    type Future = BoxFuture<'static, HandlerResult>;

    fn process_handler(self, session: Session<B>) -> Self::Future {
        Box::pin(async move {
            tokio::pin!(session);

            match self {
                Ok(DialogueState::Next(state)) => {
                    if let Err(err) = Session::set(&mut session, S::session_key(), &state).await {
                        HandlerResult::error(err)
                    } else {
                        HandlerResult::Continue
                    }
                }
                Ok(DialogueState::Exit) => {
                    if let Err(err) = session.remove(S::session_key()).await {
                        HandlerResult::error(err)
                    } else {
                        HandlerResult::Continue
                    }
                }
                Err(err) => HandlerResult::error(err),
            }
        })
    }
}

/// A dialogue handler that automatically add and remove state to session
pub struct DialogueHandler<H, B, R> {
    handler: H,
    _b: PhantomData<B>, // B generic is used to be able to annotate type for DialogueResult
    _r: PhantomData<R>,
}

impl<H: Clone, B, R> Clone for DialogueHandler<H, B, R> {
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _b: PhantomData,
            _r: PhantomData,
        }
    }
}

impl<H, B, R> From<H> for DialogueHandler<H, B, R> {
    fn from(handler: H) -> Self {
        Self {
            handler,
            _b: PhantomData,
            _r: PhantomData,
        }
    }
}

impl<H, B, T, R> Handler<(Session<B>, T), BoxFuture<'static, HandlerResult>> for DialogueHandler<H, B, R>
where
    H: Handler<T, R>,
    B: SessionBackend + Send + 'static,
    T: FromUpdate,
    R: Future + Send + 'static,
    R::Output: DialogueResult<B> + Send,
    <R::Output as DialogueResult<B>>::Future: Send,
{
    fn call(&self, (session, param): (Session<B>, T)) -> BoxFuture<'static, HandlerResult> {
        let handler = self.handler.call(param);
        Box::pin(async move {
            let res = handler.await;
            res.process_handler(session).await
        })
    }
}

#[cfg(test)]
#[cfg(feature = "session-fs")]
mod tests {
    use super::*;
    use crate::{
        dispatcher::DispatcherData,
        session::{backend::fs::FilesystemBackend, SessionManager},
        Api, Data,
    };
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;
    use tempfile::tempdir;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    enum MockState {
        Start,
        Stop,
    }

    impl Default for MockState {
        fn default() -> Self {
            Self::Start
        }
    }

    impl State for MockState {
        fn session_name() -> &'static str {
            "mock"
        }
    }

    fn create_update() -> ServiceUpdate {
        let update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test message from private chat"
            }
        }))
        .unwrap();

        ServiceUpdate {
            update,
            api: Api::new("123").unwrap(),
            data: Arc::new(DispatcherData::default()),
        }
    }

    async fn dialogue_handler(
        Dialogue { state, .. }: Dialogue<MockState, FilesystemBackend>,
    ) -> DialogueState<MockState> {
        match state {
            MockState::Start => DialogueState::Next(MockState::Stop),
            MockState::Stop => DialogueState::Exit,
        }
    }

    #[tokio::test]
    async fn dialogue() {
        let tmpdir = tempdir().expect("Failed to create temp directory");
        let session_backend = FilesystemBackend::new(tmpdir.path());
        let session_manager = SessionManager::new(session_backend);

        let mut service_update = create_update();
        let data = Arc::get_mut(&mut service_update.data).unwrap();
        data.push(Data::from(session_manager.clone()));

        let dialogue = dialogue_handler.dialogue::<FilesystemBackend>().boxed();

        for case in [Some(MockState::Stop), None].iter().cloned() {
            match dialogue.call(service_update.clone()).await {
                HandlerResult::Continue => {
                    let mut session = session_manager.get_session(&service_update.update).unwrap();
                    let state = session
                        .get(MockState::session_key())
                        .await
                        .expect("Failed to get dialogue state");
                    assert_eq!(state, case);
                }
                HandlerResult::Stop => panic!("Unexpected handler result"),
                HandlerResult::Error(err) => panic!("Dialogue error: {:?}", err),
            }
        }
    }
}
