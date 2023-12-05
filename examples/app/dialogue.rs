//! # Dialogues
//!
//! A dialogue is a stateful handler that receives the current state and returns a new state.
//!
//! The dialogue handler operates similarly to a regular handler but returns a [`DialogueResult`].
//! To obtain the state from the session, you can use the [`DialogueInput`] struct,
//! which implements the [`carapax::TryFromInput`] trait
//! and can be used as an argument for your handler.
//!
//! The state must implement the [`DialogueState`] trait,
//! and each dialogue must have a unique name,
//! defining a value for the session key to store the state.
//!
//! The state can be converted into the [`DialogueResult`].
//! Thus, you can return `state.into()` instead of `DialogueResult::Next(state)`.
//!
//! You need to wrap the dialogue handler with the [`carapax::dialogue::DialogueDecorator`]
//! and the [`carapax::dialogue::DialoguePredicate`].
//!
//! Predicate decides should the dialogue handler run or not,
//! decorator saves the state and converts a result of the dialogue
//! into the [`carapax::HandlerResult`].
//!
//! Note that you need to enable the `session` and `dialogue` features in `Cargo.toml`.
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
    chain.with(example_dialogue.with_dialogue::<FilesystemBackend>(CommandPredicate::new("/dialogue")))
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
