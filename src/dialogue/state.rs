use serde::{de::DeserializeOwned, Serialize};

const SESSION_KEY_PREFIX: &str = "__carapax_dialogue";

/// Represents a state of dialogue.
pub trait DialogueState: Default + DeserializeOwned + Serialize {
    /// Returns a unique name for the dialogue.
    fn dialogue_name() -> &'static str;

    /// Returns a key for the dialogue state in a session.
    fn session_key() -> String {
        format!("{}:{}", SESSION_KEY_PREFIX, Self::dialogue_name())
    }
}
