use crate::util::Module;
use carapax::{
    dialogue::{DialogueDecorator, DialogueInput, DialogueState},
    methods::SendMessage,
    session::backend::fs::FilesystemBackend,
    types::ChatId,
    Api, Dispatcher, ExecuteError, Ref,
};
use serde::{Deserialize, Serialize};
use tgbot::types::Text;

pub struct DialogueModule;

impl Module for DialogueModule {
    fn add_handlers(&self, dispatcher: &mut Dispatcher) {
        dispatcher.add_handler(<DialogueDecorator<FilesystemBackend, _, _>>::new(handle_dialogue));
    }
}

type ExampleDialogueInput = DialogueInput<ExampleDialogueState, FilesystemBackend>;

#[derive(Debug, Clone, Deserialize, Serialize)]
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
            if text.data == "/dialogue" {
                api.execute(SendMessage::new(chat_id, "What is your first name?"))
                    .await?;
                Ok(ExampleDialogueState::FirstName)
            } else {
                Ok(ExampleDialogueState::Start)
            }
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
