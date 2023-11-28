use crate::{
    core::{HandlerInput, TryFromInput},
    types::Integer,
};
use futures_util::future::{ok, Ready};
use std::{convert::Infallible, hash::Hash};

/// Represents a key for keyed rate-limiter
pub trait Key: Clone + Eq + Hash + TryFromInput {}

/// Represents a ratelimit key for a chat
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct KeyChat(Integer);

impl From<Integer> for KeyChat {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}

impl TryFromInput for KeyChat {
    type Error = Infallible;
    type Future = Ready<Result<Option<Self>, Self::Error>>;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.get_chat_id().map(Self))
    }
}

impl Key for KeyChat {}

/// Represents a ratelimit key for a user
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct KeyUser(Integer);

impl From<Integer> for KeyUser {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}

impl TryFromInput for KeyUser {
    type Error = Infallible;
    type Future = Ready<Result<Option<Self>, Self::Error>>;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        ok(input.update.get_user().map(|user| Self(user.id)))
    }
}

impl Key for KeyUser {}

/// Represents a ratelimit key for a chat user
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct KeyChatUser(Integer, Integer);

impl From<(Integer, Integer)> for KeyChatUser {
    fn from((chat_id, user_id): (Integer, Integer)) -> Self {
        Self(chat_id, user_id)
    }
}

impl TryFromInput for KeyChatUser {
    type Error = Infallible;
    type Future = Ready<Result<Option<Self>, Self::Error>>;

    fn try_from_input(input: HandlerInput) -> Self::Future {
        if let Some(chat_id) = input.update.get_chat_id() {
            ok(input.update.get_user().map(|user| Self(chat_id, user.id)))
        } else {
            ok(None)
        }
    }
}

impl Key for KeyChatUser {}
