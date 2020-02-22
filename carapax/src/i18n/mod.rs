mod locale;
mod store;
mod translator;

pub use self::{
    locale::{Locale, LocaleNotFound},
    store::TranslatorStore,
    translator::{TranslationKey, Translator},
};
pub use gettext::Catalog;

#[cfg(test)]
mod tests {
    use super::*;
    use gettext::Catalog;
    use tgbot::types::Update;

    const EN: &[u8] = include_bytes!("../../data/en.mo");
    const RU: &[u8] = include_bytes!("../../data/ru.mo");

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
