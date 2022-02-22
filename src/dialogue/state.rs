use serde::{de::DeserializeOwned, Serialize};

const SESSION_KEY_PREFIX: &str = "__carapax_dialogue";

/// Represents a state of dialogue
pub trait DialogueState: Default + DeserializeOwned + Serialize {
    /// Unique name of dialogue
    fn dialogue_name() -> &'static str;

    /// A key to store state in session
    fn session_key() -> String {
        format!("{}:{}", SESSION_KEY_PREFIX, Self::dialogue_name())
    }
}
