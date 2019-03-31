//! An i18n handler for carapax

#![warn(missing_docs)]

use carapax::{core::types::Update, Context, HandlerFuture, HandlerResult, UpdateHandler};
use gettext::Catalog;
use std::{collections::HashMap, sync::Arc};

/// An i18n handler for carapax
///
/// How it works with [resolver](LocaleResolver):
/// If resolver return language code, it will return translator from inner list by language code or
/// If translator is not found in inner list, it will return default translator
/// If resolver do not return language code, it will return default translator
#[derive(Debug, Clone)]
pub struct I18nHandler<R> {
    resolver: R,
    default_translator: Translator,
    translators: HashMap<String, Translator>,
}

impl<R> I18nHandler<R> {
    /// Creates a new I18nHandler
    pub fn new(resolver: R, default_translator: Translator) -> Self {
        Self {
            resolver,
            default_translator,
            translators: HashMap::new(),
        }
    }

    /// Adds a translator to inner list
    pub fn add_translator(mut self, translator: Translator) -> Self {
        self.translators.insert(translator.locale.clone(), translator);
        self
    }
}

impl<R> UpdateHandler for I18nHandler<R>
where
    R: LocaleResolver,
{
    fn handle(&self, context: &mut Context, update: &Update) -> HandlerFuture {
        let lang = self.resolver.resolve(update);
        let translator = lang
            .as_ref()
            .and_then(|lang| self.translators.get(lang).cloned())
            .unwrap_or_else(|| self.default_translator.clone());
        context.set(translator);
        HandlerResult::Continue.into()
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
    pub fn new<S: Into<String>>(locale: S, catalog: Catalog) -> Self {
        Self {
            locale: locale.into(),
            catalog: Arc::new(catalog),
        }
    }

    /// Translates a string with given [translation key](TranslationKey)
    pub fn translate<M: Into<TranslationKey>>(&self, key: M) -> String {
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
        self.locale.as_str()
    }
}

/// A locale resolver trait
pub trait LocaleResolver {
    /// Resolves locale and returns locale string
    fn resolve(&self, update: &Update) -> Option<String>;
}

/// Resolves a translator from user's language code
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
    /// Creates a new one as singular
    pub fn singular<S: Into<String>>(id: S) -> Self {
        Self {
            id: id.into(),
            kind: TranslationKind::Singular,
            context: None,
        }
    }

    /// Creates a new one as plural
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

/// The kind of translation
#[derive(Debug, Clone, PartialEq, Eq)]
enum TranslationKind {
    Singular,
    Plural { id: String, n: u64 },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{I18nHandler, UserLocaleResolver};
    use carapax::{Context, HandlerResult, UpdateHandler};
    use futures::future::Future;

    const EN: &[u8] = include_bytes!("../tests/en.mo");
    const RU: &[u8] = include_bytes!("../tests/ru.mo");

    struct Unit {
        key: TranslationKey,
        value: String,
        update: Update,
    }

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
                    "username": "username1",
                },
                "chat": {
                    "id": 1,
                    "type": "private",
                    "first_name": "test",
                    "username": "username1"
                },
                "text": "test middleware"
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

        let units = vec![
            Unit {
                key: TranslationKey::singular("Apple"),
                value: "Apple".to_string(),
                update: en_update.clone(),
            },
            Unit {
                key: TranslationKey::singular("Apple"),
                value: "Яблоко".to_string(),
                update: ru_update.clone(),
            },
            Unit {
                key: TranslationKey::plural("{} apple", "{} apples", 2),
                value: "{} apples".to_string(),
                update: en_update.clone(),
            },
            Unit {
                key: TranslationKey::plural("{} apple", "{} apples", 2),
                value: "{} яблока".to_string(),
                update: ru_update.clone(),
            },
            Unit {
                key: TranslationKey::singular("This is context").context("context"),
                value: "This is context".to_string(),
                update: en_update.clone(),
            },
            Unit {
                key: TranslationKey::singular("This is context").context("context"),
                value: "Это контекст".to_string(),
                update: ru_update.clone(),
            },
        ];

        for Unit { key, value, update } in units {
            let res = handler.handle(&mut context, &update).wait().unwrap();
            assert_eq!(res, HandlerResult::Continue);
            let translator = context.get::<Translator>();
            assert_eq!(translator.translate(key), value);
        }
    }
}
