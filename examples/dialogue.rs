use carapax::{
    dialogue::{DialogueDecorator, DialogueInput, DialogueState},
    methods::SendMessage,
    session::{backend::fs::FilesystemBackend, SessionManager},
    types::ChatId,
    Api, ExecuteError, Ref,
};
use dotenv::dotenv;
use helper::RunnerBuilder;
use serde::{Deserialize, Serialize};
use tgbot::types::Text;

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

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let backend = helper::session::backend_with_tmpdir();
    helper::session::spawn_collector(backend.clone());

    RunnerBuilder::from_env()
        .insert_data(SessionManager::new(backend))
        .build()
        .add_handler(<DialogueDecorator<FilesystemBackend, _, _>>::new(handle_dialogue))
        .run()
        .await;
}
