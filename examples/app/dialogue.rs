use crate::error::AppError;
use carapax::{
    dialogue::{DialogueExt, DialogueInput, DialogueResult, DialogueState},
    methods::SendMessage,
    session::backend::fs::FilesystemBackend,
    types::{ChatId, Text},
    Api, Chain, CommandPredicate, Ref,
};
use serde::{Deserialize, Serialize};

pub fn setup(chain: Chain) -> Chain {
    chain.add(example_dialogue.dialogue::<FilesystemBackend>(CommandPredicate::new("/dialogue")))
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
) -> Result<DialogueResult<ExampleDialogueState>, AppError> {
    let state = match input.state {
        ExampleDialogueState::Start => {
            api.execute(SendMessage::new(chat_id, "What is your first name?"))
                .await?;
            ExampleDialogueState::FirstName
        }
        ExampleDialogueState::FirstName => {
            let first_name = text.data.clone();
            api.execute(SendMessage::new(chat_id, "What is your last name?"))
                .await?;
            ExampleDialogueState::LastName { first_name }
        }
        ExampleDialogueState::LastName { first_name } => {
            let last_name = &text.data;
            api.execute(SendMessage::new(
                chat_id,
                format!("Your name is: {} {}", first_name, last_name),
            ))
            .await?;
            return Ok(DialogueResult::Exit);
        }
    };
    Ok(state.into())
}
