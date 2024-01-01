use std::{convert::Infallible, hash::Hash};

use crate::{
    core::{HandlerInput, TryFromInput},
    types::{ChatPeerId, UserPeerId},
};

/// Represents a key for a keyed rate limiter.
pub trait Key: Clone + Eq + Hash + TryFromInput {}

/// Represents a rate limit key for a chat.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct KeyChat(ChatPeerId);

impl<T> From<T> for KeyChat
where
    T: Into<ChatPeerId>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl TryFromInput for KeyChat {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.get_chat_id().map(Self))
    }
}

impl Key for KeyChat {}

/// Represents a rate limit key for a user.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct KeyUser(UserPeerId);

impl<T> From<T> for KeyUser
where
    T: Into<UserPeerId>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl TryFromInput for KeyUser {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(input.update.get_user().map(|user| Self(user.id)))
    }
}

impl Key for KeyUser {}

/// Represents a rate limit key for a user in a chat.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct KeyChatUser(ChatPeerId, UserPeerId);

impl<A, B> From<(A, B)> for KeyChatUser
where
    A: Into<ChatPeerId>,
    B: Into<UserPeerId>,
{
    fn from((chat_id, user_id): (A, B)) -> Self {
        Self(chat_id.into(), user_id.into())
    }
}

impl TryFromInput for KeyChatUser {
    type Error = Infallible;

    async fn try_from_input(input: HandlerInput) -> Result<Option<Self>, Self::Error> {
        Ok(if let Some(chat_id) = input.update.get_chat_id() {
            input.update.get_user().map(|user| Self(chat_id, user.id))
        } else {
            None
        })
    }
}

impl Key for KeyChatUser {}
