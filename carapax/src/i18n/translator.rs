use crate::{
    i18n::{Locale, LocaleNotFound, TranslatorStore},
    Data, DataError, FromUpdate, ServiceUpdate,
};
use futures_util::future::BoxFuture;
use gettext::Catalog;
use std::{convert::TryFrom, error::Error, fmt, sync::Arc};

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

impl FromUpdate for Translator {
    type Error = TranslatorError;
    type Future = BoxFuture<'static, Result<Option<Self>, Self::Error>>;

    fn from_update(service_update: ServiceUpdate) -> Self::Future {
        Box::pin(async move {
            let locale = Locale::try_from(&service_update.update)?;
            let store = Data::<TranslatorStore>::from_update(service_update)
                .await
                .map_err(|_: DataError| TranslatorError::NoStoreInData)?
                .expect("Data always returns Some");
            Ok(Some(store.get_translator(locale)))
        })
    }
}

#[derive(Debug)]
pub enum TranslatorError {
    LocaleNotFound(LocaleNotFound),
    NoStoreInData,
}

impl From<LocaleNotFound> for TranslatorError {
    fn from(err: LocaleNotFound) -> Self {
        Self::LocaleNotFound(err)
    }
}

impl fmt::Display for TranslatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TranslatorError::LocaleNotFound(err) => fmt::Display::fmt(err, f),
            TranslatorError::NoStoreInData => f.write_str("TranslatorStore was not added using Dispatcher::data()"),
        }
    }
}

impl Error for TranslatorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TranslatorError::LocaleNotFound(err) => Some(err),
            TranslatorError::NoStoreInData => None,
        }
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
