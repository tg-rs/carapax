use carapax::{
    dialogue::{
        dialogue, Dialogue,
        DialogueResult::{self, *},
        State,
    },
    longpoll::LongPoll,
    methods::SendMessage,
    session::{backend::fs::FilesystemBackend, SessionManager},
    types::Message,
    Api, Config, Dispatcher,
};
use dotenv::dotenv;
use env_logger;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, env};
use tempfile::tempdir;

struct Context {
    api: Api,
    session_manager: SessionManager<FilesystemBackend>,
}

#[derive(Serialize, Deserialize)]
enum ExampleState {
    Start,
    FirstName,
    LastName,
}

impl State for ExampleState {
    fn new() -> Self {
        ExampleState::Start
    }
}

#[dialogue]
async fn handle(
    state: ExampleState,
    context: &Context,
    input: Message,
) -> Result<DialogueResult<ExampleState>, Infallible> {
    use self::ExampleState::*;
    let chat_id = input.get_chat_id();
    let mut session = context.session_manager.get_session(&input);

    Ok(match state {
        Start => {
            context
                .api
                .execute(SendMessage::new(chat_id, "What is your first name?"))
                .await
                .unwrap();
            Next(FirstName)
        }
        FirstName => {
            let first_name = input.get_text().unwrap();
            session.set("first_name", &first_name.data).await.unwrap();
            context
                .api
                .execute(SendMessage::new(chat_id, "What is your last name?"))
                .await
                .unwrap();
            Next(LastName)
        }
        LastName => {
            let last_name = input.get_text().unwrap();
            let first_name: String = session.get("first_name").await.unwrap().unwrap();
            let text = format!(
                "First name: {first_name}\nLast name: {last_name}",
                first_name = first_name,
                last_name = last_name.data,
            );
            context.api.execute(SendMessage::new(chat_id, text)).await.unwrap();
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
    let dialogue_name = "example"; // unique dialogue name used to store state
    let mut dispatcher = Dispatcher::new(Context {
        api: api.clone(),
        session_manager: session_manager.clone(),
    });
    dispatcher.add_handler(Dialogue::new(session_manager, dialogue_name, handle));
    LongPoll::new(api, dispatcher).run().await
}
