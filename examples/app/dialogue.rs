use serde::{Deserialize, Serialize};

use carapax::{
    api::Client,
    dialogue::{DialogueExt, DialogueInput, DialogueResult, DialogueState},
    session::backend::fs::FilesystemBackend,
    types::{ChatPeerId, SendMessage, Text},
    Chain, CommandPredicate, Ref,
};

use crate::error::AppError;

pub fn setup(chain: Chain) -> Chain {
    chain.with(example_dialogue.dialogue::<FilesystemBackend>(CommandPredicate::new("/dialogue")))
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
    client: Ref<Client>,
    chat_id: ChatPeerId,
    input: ExampleDialogueInput,
    text: Text,
) -> Result<DialogueResult<ExampleDialogueState>, AppError> {
    let state = match input.state {
        ExampleDialogueState::Start => {
            client
                .execute(SendMessage::new(chat_id, "What is your first name?"))
                .await?;
            ExampleDialogueState::FirstName
        }
        ExampleDialogueState::FirstName => {
            let first_name = text.data.clone();
            client
                .execute(SendMessage::new(chat_id, "What is your last name?"))
                .await?;
            ExampleDialogueState::LastName { first_name }
        }
        ExampleDialogueState::LastName { first_name } => {
            let last_name = &text.data;
            client
                .execute(SendMessage::new(
                    chat_id,
                    format!("Your name is: {} {}", first_name, last_name),
                ))
                .await?;
            return Ok(DialogueResult::Exit);
        }
    };
    Ok(state.into())
}
