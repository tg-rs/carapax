use carapax::prelude::*;
use carapax_i18n::{Catalog, I18nHandler, Translator, UserLocaleResolver};
use dotenv::dotenv;
use env_logger;
use futures::future::Future;
use std::env;

const RU: &[u8] = include_bytes!("../data/ru.mo");

fn handle_message(context: &mut Context, message: Message) -> HandlerFuture {
    let api: &Api = context.get();
    let translator: &Translator = context.get();

    HandlerFuture::new(
        api.execute(SendMessage::new(
            message.get_chat_id(),
            translator.translate("Hello, stranger!"),
        ))
        .map(|_| HandlerResult::Continue),
    )
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();

    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy);
    }

    let api = Api::new(config).unwrap();

    let ru = Translator::new("ru", Catalog::parse(RU).unwrap());

    tokio::run(
        App::new()
            .add_handler(I18nHandler::new(UserLocaleResolver, ru))
            .add_handler(FnHandler::from(handle_message))
            .run(api.clone(), UpdateMethod::poll(UpdatesStream::new(api))),
    )
}
