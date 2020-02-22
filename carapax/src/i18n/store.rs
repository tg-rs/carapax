use crate::i18n::{locale::Locale, translator::Translator};
use std::{collections::HashMap, convert::TryInto};

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
        self.translators
            .insert(String::from(translator.get_locale()), translator);
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
                .get(locale.get_name())
                .cloned()
                .unwrap_or_else(|| self.default_translator.clone()),
            Err(_) => self.default_translator.clone(),
        }
    }
}
