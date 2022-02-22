use crate::{
    core::{Handler, HandlerInput, Predicate, TryFromInput},
    dialogue::{decorator::DialogueDecorator, predicate::DialoguePredicate},
};

/// Dialogue shortcuts
pub trait DialogueExt<P, PI, HI, HS>: Sized {
    /// Shortcut to create a new dialogue decorator (`handler.dialogue(predicate)`)
    ///
    /// # Arguments
    ///
    /// * predicate - A predicate for dialogue
    #[allow(clippy::type_complexity)]
    fn dialogue<B>(
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
