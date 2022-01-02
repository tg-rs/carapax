use carapax::{
    dialogue::{DialogueDecorator, DialogueInput, DialogueState},
    methods::SendMessage,
    session::{backend::fs::FilesystemBackend, SessionCollector, SessionManager},
    types::ChatId,
    Api, Config, Context, Dispatcher, ExecuteError, Ref,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{env, time::Duration};
use tempfile::tempdir;
use tgbot::{longpoll::LongPoll, types::Text};

type ExampleDialogueInput = DialogueInput<ExampleDialogueState, FilesystemBackend>;

#[derive(Clone, Deserialize, Serialize)]
enum ExampleDialogueState {
    Start,
    FirstName,
    LastName { first_name: String },
}

impl Default for ExampleDialogueState {
    fn default() -> Self {
        Self::Start
    }
}

impl DialogueState for ExampleDialogueState {
    fn dialogue_name() -> &'static str {
        "example"
    }
}

async fn handle_dialogue(
    api: Ref<Api>,
    chat_id: ChatId,
    input: ExampleDialogueInput,
    text: Text,
) -> Result<ExampleDialogueState, ExecuteError> {
    match input.state {
        ExampleDialogueState::Start => {
            api.execute(SendMessage::new(chat_id, "What is your first name?"))
                .await?;
            Ok(ExampleDialogueState::FirstName)
        }
        ExampleDialogueState::FirstName => {
            let first_name = text.data.clone();
            api.execute(SendMessage::new(chat_id, "What is your last name?"))
                .await?;
            Ok(ExampleDialogueState::LastName { first_name })
        }
        ExampleDialogueState::LastName { first_name } => {
            let last_name = &text.data;
            api.execute(SendMessage::new(
                chat_id,
                format!("Your name is: {} {}", first_name, last_name),
            ))
            .await?;
            Ok(ExampleDialogueState::Start)
        }
    }
}

fn getenv(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("{} is not set", name))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = getenv("CARAPAX_TOKEN");
    let proxy = env::var("CARAPAX_PROXY").ok();
    let gc_period = getenv("CARAPAX_SESSION_GC_PERIOD");
    let gc_period = Duration::from_secs(
        gc_period
            .parse::<u64>()
            .expect("CARAPAX_SESSION_GC_PERIOD must be integer"),
    ); // period between GC calls
    let session_lifetime = getenv("CARAPAX_SESSION_LIFETIME");
    let session_lifetime = Duration::from_secs(
        session_lifetime
            .parse::<u64>()
            .expect("CARAPAX_SESSION_LIFETIME must be integer"),
    ); // how long session lives

    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }

    let api = Api::new(config).expect("Failed to create API");
    let tmpdir = tempdir().expect("Failed to create temp directory");
    log::info!("Session directory: {}", tmpdir.path().display());

    let backend = FilesystemBackend::new(tmpdir.path());

    // spawn GC to remove old sessions
    let mut collector = SessionCollector::new(backend.clone(), gc_period, session_lifetime);
    tokio::spawn(async move { collector.run().await });

    let mut context = Context::default();
    context.insert(api.clone());
    context.insert(SessionManager::new(backend));

    let mut dispatcher = Dispatcher::new(context);
    dispatcher.add_handler(<DialogueDecorator<FilesystemBackend, _, _>>::new(handle_dialogue));
    LongPoll::new(api, dispatcher).run().await
}
