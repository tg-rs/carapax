use std::{convert::Infallible, sync::Arc};

use seance::Session;
use serde::{Deserialize, Serialize};
use tempfile::tempdir;

use crate::{
    core::{Chain, Context, Handler, HandlerInput, PredicateOutput, TryFromInput},
    session::{backend::fs::FilesystemBackend, SessionManager},
    types::Text,
};

use super::*;

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
    let handler = dialogue_handler.with_dialogue::<FilesystemBackend>(dialogue_predicate);

    let input = create_input(context.clone(), "start");

    let session = &mut get_session(input.clone()).await;
    let session_key = State::session_key();

    assert!(matches!(
        handler.handle((input.clone(), input)).await,
        PredicateOutput::True(Ok(()))
    ));
    let state: Option<State> = session.get(&session_key).await.expect("Failed to get state");
    assert!(matches!(state, Some(State::Step)));

    let input = create_input(context.clone(), "step");
    assert!(matches!(
        handler.handle((input.clone(), input)).await,
        PredicateOutput::True(Ok(()))
    ));
    let state: Option<State> = session.get(&session_key).await.expect("Failed to get state");
    assert!(state.is_none());
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
    let handler = dialogue_handler.with_dialogue::<FilesystemBackend>(dialogue_predicate);
    let chain = Chain::once().with(handler).with(skip_handler);
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
