//! An i18n utilities for carapax
#![warn(missing_docs)]

use carapax::{
    types::{CallbackQuery, ChosenInlineResult, InlineQuery, Message, PreCheckoutQuery, ShippingQuery, Update, User},
    Command,
};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    error::Error,
    fmt,
    sync::Arc,
};

pub use gettext::Catalog;

/// A store for translators
#[derive(Debug, Clone)]
pub struct TranslatorStore {
    default_translator: Translator,
    translators: HashMap<String, Translator>,
}

impl TranslatorStore {
    /// Creates a new store
    ///
    /// # Arguments
    ///
    /// * default_translator - Default translator used when translator not found for locale or locale is missing
    pub fn new(default_translator: Translator) -> Self {
        Self {
            default_translator,
            translators: HashMap::new(),
        }
    }

    /// Adds a translator
    pub fn add_translator(mut self, translator: Translator) -> Self {
        self.translators.insert(translator.locale.clone(), translator);
        self
    }

    /// Returns a translator for given locale
    ///
    /// If translator not found for locale,
    /// default translator will be returned.
    pub fn get_translator<L>(&self, locale: L) -> Translator
    where
        L: TryInto<Locale>,
    {
        match locale.try_into() {
            Ok(locale) => self
                .translators
                .get(&locale.0)
                .cloned()
                .unwrap_or_else(|| self.default_translator.clone()),
            Err(_) => self.default_translator.clone(),
        }
    }
}

/// An i18n translator uses `gettext` crate
#[derive(Debug, Clone)]
pub struct Translator {
    locale: String,
    catalog: Arc<Catalog>,
}

impl Translator {
    /// Creates a new translator
    ///
    /// # Arguments
    ///
    /// * locale - A locale string
    /// * catalog - A gettext message catalog
    pub fn new<S: Into<String>>(locale: S, catalog: Catalog) -> Self {
        Self {
            locale: locale.into(),
            catalog: Arc::new(catalog),
        }
    }

    /// Translates a given string
    pub fn translate<K: Into<TranslationKey>>(&self, key: K) -> String {
        let key = key.into();
        let singular = &key.id;
        match (&key.kind, &key.context) {
            (TranslationKind::Singular, None) => self.catalog.gettext(singular),
            (TranslationKind::Plural { id, n }, None) => self.catalog.ngettext(singular, id, *n),
            (TranslationKind::Singular, Some(context)) => self.catalog.pgettext(context, singular),
            (TranslationKind::Plural { id, n }, Some(context)) => self.catalog.npgettext(context, singular, id, *n),
        }
        .to_string()
    }

    /// Returns a locale
    pub fn get_locale(&self) -> &str {
        &self.locale
    }
}

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

/// A key for translation
#[derive(Debug, Clone)]
pub struct TranslationKey {
    id: String,
    kind: TranslationKind,
    context: Option<String>,
}

impl TranslationKey {
    /// Creates a new key for a singular
    pub fn singular<S: Into<String>>(id: S) -> Self {
        Self {
            id: id.into(),
            kind: TranslationKind::Singular,
            context: None,
        }
    }

    /// Creates a new key for a plural
    pub fn plural<S, P>(id: S, plural_id: P, n: u64) -> Self
    where
        S: Into<String>,
        P: Into<String>,
    {
        Self {
            id: id.into(),
            kind: TranslationKind::Plural {
                id: plural_id.into(),
                n,
            },
            context: None,
        }
    }

    /// Sets `gettext` context
    pub fn context<S: Into<String>>(mut self, context: S) -> Self {
        self.context = Some(context.into());
        self
    }
}

impl<S> From<S> for TranslationKey
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        TranslationKey::singular(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TranslationKind {
    Singular,
    Plural { id: String, n: u64 },
}

#[cfg(test)]
mod tests {
    use super::*;

    const EN: &[u8] = include_bytes!("../data/en.mo");
    const RU: &[u8] = include_bytes!("../data/ru.mo");

    #[test]
    fn store() {
        let en = Catalog::parse(EN).unwrap();
        let en = Translator::new("en", en);

        let ru = Catalog::parse(RU).unwrap();
        let ru = Translator::new("ru", ru);

        let store = TranslatorStore::new(en).add_translator(ru);

        let en_update: serde_json::Value = serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 0,
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                    "username": "username",
                },
                "chat": {
                    "id": 1,
                    "type": "private",
                    "first_name": "test",
                    "username": "username"
                },
                "text": "test i18n"
            }
        });
        let mut ru_update = en_update.clone();
        ru_update
            .get_mut("message")
            .unwrap()
            .get_mut("from")
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("language_code".to_string(), serde_json::Value::from("ru"));
        let en_update: Update = serde_json::from_value(en_update).unwrap();
        let ru_update: Update = serde_json::from_value(ru_update).unwrap();

        for (key, value, update) in vec![
            (TranslationKey::singular("Apple"), "Apple", en_update.clone()),
            (TranslationKey::singular("Apple"), "Яблоко", ru_update.clone()),
            (
                TranslationKey::plural("{} apple", "{} apples", 2),
                "{} apples",
                en_update.clone(),
            ),
            (
                TranslationKey::plural("{} apple", "{} apples", 2),
                "{} яблока",
                ru_update.clone(),
            ),
            (
                TranslationKey::singular("This is context").context("context"),
                "This is context",
                en_update,
            ),
            (
                TranslationKey::singular("This is context").context("context"),
                "Это контекст",
                ru_update,
            ),
        ] {
            let translator = store.get_translator(&update);
            assert_eq!(translator.translate(key), value);
        }
    }
}
