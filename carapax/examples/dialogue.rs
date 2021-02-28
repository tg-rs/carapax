use carapax::{
    dialogue::{
        Dialogue,
        DialogueState::{self, *},
        State,
    },
    longpoll::LongPoll,
    methods::SendMessage,
    session::{backend::fs::FilesystemBackend, Session, SessionManager},
    types::Message,
    Api, Config, Dispatcher, ExecuteError, Handler,
};
use dotenv::dotenv;
use seance::SessionError;
use serde::{Deserialize, Serialize};
use std::{env, fmt};
use tempfile::tempdir;

#[derive(Debug)]
enum Error {
    Execute(ExecuteError),
    Session(SessionError),
}

impl From<ExecuteError> for Error {
    fn from(err: ExecuteError) -> Self {
        Self::Execute(err)
    }
}

impl From<SessionError> for Error {
    fn from(err: SessionError) -> Self {
        Self::Session(err)
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Example error")
    }
}

#[derive(Serialize, Deserialize)]
enum ExampleState {
    Start,
    FirstName,
    LastName,
}

impl Default for ExampleState {
    fn default() -> Self {
        Self::Start
    }
}

impl State for ExampleState {
    fn session_name() -> &'static str {
        "example-state"
    }
}

async fn handler(
    api: Api,
    Dialogue { state, .. }: Dialogue<ExampleState, FilesystemBackend>,
    mut session: Session<FilesystemBackend>,
    message: Message,
) -> Result<DialogueState<ExampleState>, Error> {
    use self::ExampleState::*;

    let chat_id = message.get_chat_id();

    Ok(match state {
        Start => {
            api.execute(SendMessage::new(chat_id, "What is your first name?"))
                .await?;
            Next(FirstName)
        }
        FirstName => {
            let first_name = message.get_text().unwrap();
            session.set("first_name", &first_name.data).await?;
            api.execute(SendMessage::new(chat_id, "What is your last name?"))
                .await?;
            Next(LastName)
        }
        LastName => {
            let last_name = message.get_text().unwrap();
            let first_name: String = session.get("first_name").await?.unwrap();
            let text = format!(
                "First name: {first_name}\nLast name: {last_name}",
                first_name = first_name,
                last_name = last_name.data,
            );
            api.execute(SendMessage::new(chat_id, text)).await?;
            Exit
        }
    })
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let proxy = env::var("CARAPAX_PROXY").ok();

    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }

    let tmpdir = tempdir().expect("Failed to create temp directory");
    let session_backend = FilesystemBackend::new(tmpdir.path());
    let session_manager = SessionManager::new(session_backend);

    let api = Api::new(config).expect("Failed to create API");

    let mut dispatcher = Dispatcher::new(api.clone());
    dispatcher.add_handler(handler.dialogue::<FilesystemBackend>());
    dispatcher.data(session_manager);
    LongPoll::new(api, dispatcher).run().await
}
