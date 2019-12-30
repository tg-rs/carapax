use carapax::{handler, longpoll::LongPoll, methods::SendMessage, types::Update, Api, Config, Dispatcher};
use carapax_i18n::{Catalog, Translator, TranslatorStore, UserLocaleResolver};
use dotenv::dotenv;
use env_logger;
use std::env;

const RU: &[u8] = include_bytes!("../data/ru.mo");
const EN: &[u8] = include_bytes!("../data/en.mo");

struct Context {
    api: Api,
    translators: TranslatorStore<UserLocaleResolver>,
}

#[handler]
async fn update_handler(context: &mut Context, update: Update) {
    let translator = context.translators.get_translator(&update);
    println!("GOT UPDATE: {:?}; LOCALE: {:?}", update, translator.get_locale());
    context
        .api
        .execute(SendMessage::new(
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

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();

    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }

    let api = Api::new(config).expect("Failed to create API");
    let en = Translator::new("en", Catalog::parse(EN).unwrap());
    let ru = Translator::new("ru", Catalog::parse(RU).unwrap());
    let translators = TranslatorStore::new(UserLocaleResolver, en).add_translator(ru);
    let mut dispatcher = Dispatcher::new(Context {
        api: api.clone(),
        translators,
    });
    dispatcher.add_handler(update_handler);

    LongPoll::new(api, dispatcher).run().await
}
