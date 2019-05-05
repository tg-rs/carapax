//! An i18n handler for carapax
#![warn(missing_docs)]

use carapax::{context::Context, core::types::Update, Handler};
use std::{collections::HashMap, sync::Arc};

pub use gettext::Catalog;

/// An i18n handler for carapax
#[derive(Debug, Clone)]
pub struct I18nHandler<R> {
    resolver: R,
    default_translator: Translator,
    translators: HashMap<String, Translator>,
}

impl<R> I18nHandler<R> {
    /// Creates a new I18nHandler
    ///
    /// # Arguments
    ///
    /// * resolver - A locale resolver
    /// * default_translator - Default translator used when translator not found for locale or locale is missing
    pub fn new(resolver: R, default_translator: Translator) -> Self {
        Self {
            resolver,
            default_translator,
            translators: HashMap::new(),
        }
    }

    /// Adds a translator
    pub fn add_translator(mut self, translator: Translator) -> Self {
        self.translators.insert(translator.locale.clone(), translator);
        self
    }
}

impl<R> Handler for I18nHandler<R>
where
    R: LocaleResolver,
{
    type Input = Update;
    type Output = ();

    fn handle(&self, context: &mut Context, update: Self::Input) -> Self::Output {
        let locale = self.resolver.resolve(&update);
        let translator = locale
            .as_ref()
            .and_then(|locale| self.translators.get(locale).cloned())
            .unwrap_or_else(|| self.default_translator.clone());
        context.set(translator);
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
    pub fn locale(&self) -> &str {
        &self.locale
    }
}

/// A locale resolver trait
pub trait LocaleResolver {
    /// Returns a locale string from given update
    fn resolve(&self, update: &Update) -> Option<String>;
}

/// Resolves a locale from user's language code
#[derive(Debug, Clone, Copy)]
pub struct UserLocaleResolver;

impl LocaleResolver for UserLocaleResolver {
    fn resolve(&self, update: &Update) -> Option<String> {
        update.get_user().and_then(|user| user.language_code.clone())
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
    fn test_handler() {
        let en = Catalog::parse(EN).unwrap();
        let en = Translator::new("en", en);

        let ru = Catalog::parse(RU).unwrap();
        let ru = Translator::new("ru", ru);

        let handler = I18nHandler::new(UserLocaleResolver, en).add_translator(ru);
        let mut context = Context::default();

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
                en_update.clone(),
            ),
            (
                TranslationKey::singular("This is context").context("context"),
                "Это контекст",
                ru_update.clone(),
            ),
        ] {
            handler.handle(&mut context, update.clone());
            let translator = context.get::<Translator>();
            assert_eq!(translator.translate(key), value);
        }
    }
}
