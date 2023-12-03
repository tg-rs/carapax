use std::{convert::Infallible, hash::Hash};

use futures_util::future::{ok, Ready};

use crate::{
    core::{HandlerInput, TryFromInput},
    types::{ChatPeerId, UserPeerId},
};

/// Represents a key for keyed rate-limiter
pub trait Key: Clone + Eq + Hash + TryFromInput {}

/// Represents a ratelimit key for a chat
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
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.get_chat_id().map(Self))
    }
}

impl Key for KeyChat {}

/// Represents a ratelimit key for a user
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
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.get_user().map(|user| Self(user.id)))
    }
}

impl Key for KeyUser {}

/// Represents a ratelimit key for a chat user
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
    type Future = Ready<Result<Option<Self>, Self::Error>>;
    type Error = Infallible;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        if let Some(chat_id) = input.update.get_chat_id() {
            ok(input.update.get_user().map(|user| Self(chat_id, user.id)))
        } else {
            ok(None)
        }
    }
}

impl Key for KeyChatUser {}
