use std::{any::TypeId, convert::Infallible, error::Error, fmt, future::Future};

use futures_util::future::{ok, ready, BoxFuture, Ready};

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

/// Allows to create a specific handler input
pub trait TryFromInput: Send + Sized {
    /// A future returned by `try_from_input` method
    type Future: Future<Output = Result<Option<Self>, Self::Error>> + Send;

    /// An error when conversion failed
    type Error: Error + Send;

    /// Performs conversion
    ///
    /// # Arguments
    ///
    /// * input - A value to convert from
    fn try_from_input(input: HandlerInput) -> Self::Future;
}

impl TryFromInput for HandlerInput {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(Some(input))
    }
}

impl TryFromInput for () {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(_input: HandlerInput) -> Self::Future {
        ok(Some(()))
    }
}

impl<T> TryFromInput for Ref<T>
where
    T: Clone + Send + 'static,
{
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = ConvertInputError;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ready(
            input
                .context
                .get::<T>()
                .cloned()
                .map(Ref::new)
                .ok_or_else(ConvertInputError::context::<T>)
                .map(Some),
        )
    }
}

impl TryFromInput for Update {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(Some(input.update))
    }
}

impl TryFromInput for ChatPeerId {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.get_chat_id())
    }
}

impl TryFromInput for ChatUsername {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.get_chat_username().cloned())
    }
}

impl TryFromInput for Chat {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.get_chat().cloned())
    }
}

impl TryFromInput for UserPeerId {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.get_user_id())
    }
}

impl TryFromInput for UserUsername {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.get_user_username().cloned())
    }
}

impl TryFromInput for User {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.get_user().cloned())
    }
}

impl TryFromInput for Text {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(Message::try_from(input.update).ok().and_then(|x| x.get_text().cloned()))
    }
}

impl TryFromInput for Message {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.try_into().ok())
    }
}

impl TryFromInput for Command {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = CommandError;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ready(
            Message::try_from(input.update)
                .ok()
                .map(Command::try_from)
                .transpose()
                .or_else(|err| match err {
                    CommandError::NotFound => Ok(None),
                    err => Err(err),
                }),
        )
    }
}

impl TryFromInput for InlineQuery {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.try_into().ok())
    }
}

impl TryFromInput for ChosenInlineResult {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.try_into().ok())
    }
}

impl TryFromInput for CallbackQuery {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.try_into().ok())
    }
}

impl TryFromInput for ShippingQuery {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.try_into().ok())
    }
}

impl TryFromInput for PreCheckoutQuery {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.try_into().ok())
    }
}

impl TryFromInput for Poll {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.try_into().ok())
    }
}

impl TryFromInput for PollAnswer {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.try_into().ok())
    }
}

impl TryFromInput for ChatMemberUpdated {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.try_into().ok())
    }
}

impl TryFromInput for ChatJoinRequest {
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.try_into().ok())
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
            type Future = BoxFuture<'static, Result<Option<Self>, Self::Error>>;
            type Error = ConvertInputError;

            fn try_from_input(input: HandlerInput) -> Self::Future {
                Box::pin(async move {
                    $(
                        let $T = match <$T>::try_from_input(
                            input.clone()
                        ).await.map_err(ConvertInputError::tuple)? {
                            Some(v) => v,
                            None => return Ok(None)
                        };
                    )+
                    Ok(Some(($($T,)+)))
                })
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

/// An error when converting a [HandlerInput](strut.HandlerInput.html)
#[derive(Debug)]
pub enum ConvertInputError {
    /// Object not found in context
    Context(TypeId),
    /// Could not create a tuple
    ///
    /// Contains a first occurred error
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
