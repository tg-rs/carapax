use crate::core::Command;
use std::{convert::TryFrom, error::Error, fmt};
use tgbot::types::{
    CallbackQuery, ChosenInlineResult, InlineQuery, Message, PreCheckoutQuery, ShippingQuery, Update, User,
};

/// A locale
pub struct Locale(String);

impl Locale {
    /// Creates a new locale
    ///
    /// # Arguments
    ///
    /// * locale - Locale string
    pub fn new<L>(locale: L) -> Self
    where
        L: Into<String>,
    {
        Self(locale.into())
    }

    /// Returns a locale name
    pub fn get_name(&self) -> &str {
        &self.0
    }
}

/// User not found when converting an object to locale
#[derive(Debug)]
pub struct LocaleNotFound;

impl Error for LocaleNotFound {}

impl fmt::Display for LocaleNotFound {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "unable to get locale")
    }
}

impl TryFrom<&User> for Locale {
    type Error = LocaleNotFound;

    fn try_from(user: &User) -> Result<Self, Self::Error> {
        match user.language_code {
            Some(ref language_code) => Ok(Self::new(language_code.clone())),
            None => Err(LocaleNotFound),
        }
    }
}

impl TryFrom<&Update> for Locale {
    type Error = LocaleNotFound;

    fn try_from(update: &Update) -> Result<Self, Self::Error> {
        match update.get_user() {
            Some(user) => Self::try_from(user),
            None => Err(LocaleNotFound),
        }
    }
}

impl TryFrom<&CallbackQuery> for Locale {
    type Error = LocaleNotFound;

    fn try_from(callback_query: &CallbackQuery) -> Result<Self, Self::Error> {
        Self::try_from(&callback_query.from)
    }
}

impl TryFrom<&ChosenInlineResult> for Locale {
    type Error = LocaleNotFound;

    fn try_from(chosen_inline_result: &ChosenInlineResult) -> Result<Self, Self::Error> {
        Self::try_from(&chosen_inline_result.from)
    }
}

impl TryFrom<&Command> for Locale {
    type Error = LocaleNotFound;

    fn try_from(command: &Command) -> Result<Self, Self::Error> {
        match command.get_message().get_user() {
            Some(user) => Self::try_from(user),
            None => Err(LocaleNotFound),
        }
    }
}

impl TryFrom<&InlineQuery> for Locale {
    type Error = LocaleNotFound;

    fn try_from(inline_query: &InlineQuery) -> Result<Self, Self::Error> {
        Self::try_from(&inline_query.from)
    }
}

impl TryFrom<&Message> for Locale {
    type Error = LocaleNotFound;

    fn try_from(message: &Message) -> Result<Self, Self::Error> {
        match message.get_user() {
            Some(user) => Self::try_from(user),
            None => Err(LocaleNotFound),
        }
    }
}

impl TryFrom<&PreCheckoutQuery> for Locale {
    type Error = LocaleNotFound;

    fn try_from(pre_checkout_query: &PreCheckoutQuery) -> Result<Self, Self::Error> {
        Self::try_from(&pre_checkout_query.from)
    }
}

impl TryFrom<&ShippingQuery> for Locale {
    type Error = LocaleNotFound;

    fn try_from(shipping_query: &ShippingQuery) -> Result<Self, Self::Error> {
        Self::try_from(&shipping_query.from)
    }
}
