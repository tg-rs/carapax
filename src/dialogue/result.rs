use crate::dialogue::state::DialogueState;

/// Represents a result of a dialogue handler.
#[derive(Debug)]
pub enum DialogueResult<S> {
    /// Indicates the next step of the dialogue containing the current value of the state.
    Next(S),
    /// Indicates an exit from the dialogue.
    Exit,
}

impl<S> From<S> for DialogueResult<S>
where
    S: DialogueState,
{
    fn from(state: S) -> Self {
        DialogueResult::Next(state)
    }
}
