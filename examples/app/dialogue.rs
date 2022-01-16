use crate::error::AppError;
use carapax::{
    dialogue::{DialogueDecorator, DialogueInput, DialogueState},
    methods::SendMessage,
    session::backend::fs::FilesystemBackend,
    types::ChatId,
    Api, Chain, Ref,
};
use serde::{Deserialize, Serialize};
use tgbot::types::Text;

pub fn setup(chain: &mut Chain) {
    chain.add_handler(<DialogueDecorator<FilesystemBackend, _, _>>::new(example_dialogue));
}

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

async fn example_dialogue(
    api: Ref<Api>,
    chat_id: ChatId,
    input: ExampleDialogueInput,
    text: Text,
) -> Result<ExampleDialogueState, AppError> {
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
