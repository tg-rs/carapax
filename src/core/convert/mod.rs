use std::{any::TypeId, convert::Infallible, error::Error, fmt, future::Future};

use crate::{
    core::{context::Ref, handler::HandlerInput},
    types::{
        CallbackQuery, Chat, ChatJoinRequest, ChatMemberUpdated, ChatPeerId, ChatUsername, ChosenInlineResult, Command,
        CommandError, InlineQuery, Message, Poll, PollAnswer, PreCheckoutQuery, ShippingQuery, Text, Update, User,
        UserPeerId, UserUsername,
    },
};

#[cfg(test)]
mod tests;

/// Allows to create a specific handler input.
pub trait TryFromInput: Send + Sized {
    /// An error when conversion failed.
    type Error: Error + Send;

    /// Performs conversion.
    ///
    /// # Arguments
    ///
    /// * `input` - An input to convert from.
    fn try_from_input(input: HandlerInput) -> impl Future<Output = Result<Option<Self>, Self::Error>> + Send;
}

impl TryFromInput for HandlerInput {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(Some(input))
    }
}

impl TryFromInput for () {
    type Error = Infallible;

    async fn try_from_input(_input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(Some(()))
    }
}

impl<T> TryFromInput for Ref<T>
where
    T: Clone + Send + 'static,
{
    type Error = ConvertInputError;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        input
            .context
            .get::<T>()
            .cloned()
            .map(Ref::new)
            .ok_or_else(ConvertInputError::context::<T>)
            .map(Some)
    }
}

impl TryFromInput for Update {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(Some(input.update))
    }
}

impl TryFromInput for ChatPeerId {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.get_chat_id())
    }
}

impl TryFromInput for ChatUsername {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.get_chat_username().cloned())
    }
}

impl TryFromInput for Chat {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.get_chat().cloned())
    }
}

impl TryFromInput for UserPeerId {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.get_user_id())
    }
}

impl TryFromInput for UserUsername {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.get_user_username().cloned())
    }
}

impl TryFromInput for User {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.get_user().cloned())
    }
}

impl TryFromInput for Text {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(Message::try_from(input.update).ok().and_then(|x| x.get_text().cloned()))
    }
}

impl TryFromInput for Message {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.try_into().ok())
    }
}

impl TryFromInput for Command {
    type Error = CommandError;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Message::try_from(input.update)
            .ok()
            .map(Command::try_from)
            .transpose()
            .or_else(|err| match err {
                CommandError::NotFound => Ok(None),
                err => Err(err),
            })
    }
}

impl TryFromInput for InlineQuery {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.try_into().ok())
    }
}

impl TryFromInput for ChosenInlineResult {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.try_into().ok())
    }
}

impl TryFromInput for CallbackQuery {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.try_into().ok())
    }
}

impl TryFromInput for ShippingQuery {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.try_into().ok())
    }
}

impl TryFromInput for PreCheckoutQuery {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.try_into().ok())
    }
}

impl TryFromInput for Poll {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.try_into().ok())
    }
}

impl TryFromInput for PollAnswer {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.try_into().ok())
    }
}

impl TryFromInput for ChatMemberUpdated {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.try_into().ok())
    }
}

impl TryFromInput for ChatJoinRequest {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.try_into().ok())
    }
}

macro_rules! convert_tuple {
    ($($T:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($T),+> TryFromInput for ($($T,)+)
        where
            $(
                $T: TryFromInput,
                $T::Error: 'static,
            )+
        {
            type Error = ConvertInputError;

            async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
                $(
                    let $T = match <$T>::try_from_input(
                        input.clone()
                    ).await.map_err(ConvertInputError::tuple)? {
                        Some(v) => v,
                        None => return Ok(None)
                    };
                )+
                Ok(Some(($($T,)+)))
            }
        }
    };
}

convert_tuple!(A);
convert_tuple!(A, B);
convert_tuple!(A, B, C);
convert_tuple!(A, B, C, D);
convert_tuple!(A, B, C, D, E);
convert_tuple!(A, B, C, D, E, F);
convert_tuple!(A, B, C, D, E, F, G);
convert_tuple!(A, B, C, D, E, F, G, H);
convert_tuple!(A, B, C, D, E, F, G, H, I);
convert_tuple!(A, B, C, D, E, F, G, H, I, J);

/// An error when converting a [`HandlerInput`].
#[derive(Debug)]
pub enum ConvertInputError {
    /// Object is not found in the [`crate::Context`].
    Context(TypeId),
    /// Unable to convert [`HandlerInput`] into a tuple of specific inputs.
    ///
    /// Contains a first occurred error.
    Tuple(Box<dyn Error + Send>),
}

impl ConvertInputError {
    fn context<T: 'static>() -> Self {
        Self::Context(TypeId::of::<T>())
    }

    fn tuple<E: Error + Send + 'static>(err: E) -> Self {
        Self::Tuple(Box::new(err))
    }
}

impl Error for ConvertInputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::ConvertInputError::*;
        match self {
            Context(_) => None,
            Tuple(err) => err.source(),
        }
    }
}

impl fmt::Display for ConvertInputError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::ConvertInputError::*;
        match self {
            Context(type_id) => write!(out, "Object of type {:?} not found in context", type_id),
            Tuple(err) => write!(out, "Unable to convert HandlerInput into tuple: {}", err),
        }
    }
}
