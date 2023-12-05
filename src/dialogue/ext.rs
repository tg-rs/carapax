use crate::{
    core::{Handler, HandlerInput, Predicate, TryFromInput},
    dialogue::{decorator::DialogueDecorator, predicate::DialoguePredicate},
};

/// Provides a shortcut for wrapping a [`Handler`] by a [`DialogueDecorator`].
pub trait DialogueExt<P, PI, HI, HS>: Sized {
    /// Shortcut to wrap a [`Handler`] with a [`DialogueDecorator`].
    ///
    /// Example: `handler.dialogue(predicate)`.
    ///
    /// # Arguments
    ///
    /// * `predicate` - A predicate to be execute before starting the dialogue.
    ///
    /// If you don't need to start the dialogue conditionally,
    /// you can use [`DialogueDecorator::new`] directly.
    #[allow(clippy::type_complexity)]
    fn with_dialogue<B>(
        self,
        predicate: P,
    ) -> Predicate<DialoguePredicate<B, P, PI, HS>, HandlerInput, DialogueDecorator<B, Self, HI, HS>, HandlerInput>
    {
        Predicate::new(DialoguePredicate::new(predicate), DialogueDecorator::new(self))
    }
}

impl<P, PI, H, HI, HS> DialogueExt<P, PI, HI, HS> for H
where
    P: Handler<PI>,
    PI: TryFromInput,
    H: Handler<HI>,
    HI: TryFromInput,
{
}
