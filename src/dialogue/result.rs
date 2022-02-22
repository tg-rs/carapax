use crate::dialogue::state::DialogueState;

/// Result of dialogue handler
#[derive(Debug)]
pub enum DialogueResult<S> {
    /// Next state
    Next(S),
    /// Exit from dialogue
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
