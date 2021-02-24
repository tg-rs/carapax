use carapax::{
    i18n::{Catalog, Translator, TranslatorStore},
    longpoll::LongPoll,
    methods::SendMessage,
    types::Update,
    Api, Config, Dispatcher,
};
use dotenv::dotenv;
use std::env;

const RU: &[u8] = include_bytes!("../data/ru.mo");
const EN: &[u8] = include_bytes!("../data/en.mo");

async fn update_handler(api: Api, translator: Translator, update: Update) {
    println!("GOT UPDATE: {:?}; LOCALE: {:?}", update, translator.get_locale());
    api.execute(SendMessage::new(
        update.get_chat_id().unwrap(),
        translator.translate("Hello, stranger!"),
    ))
    .await
    .unwrap();
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let proxy = env::var("CARAPAX_PROXY").ok();

    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }

    let api = Api::new(config).expect("Failed to create API");
    let en = Translator::new("en", Catalog::parse(EN).unwrap());
    let ru = Translator::new("ru", Catalog::parse(RU).unwrap());
    let translators = TranslatorStore::new(en).add_translator(ru);
    let mut dispatcher = Dispatcher::new(api.clone());
    dispatcher.add_handler(update_handler).data(translators);

    LongPoll::new(api, dispatcher).run().await
}
