use gettext::Catalog;
use std::sync::Arc;

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
