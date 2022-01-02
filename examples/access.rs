use carapax::{
    access::{AccessExt, AccessRule, InMemoryAccessPolicy},
    longpoll::LongPoll,
    types::Update,
    Api, Config, Context, Dispatcher,
};
use dotenv::dotenv;
use std::env;

async fn update_handler(update: Update) {
    log::info!("Got a new update: {:?}", update);
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let proxy = env::var("CARAPAX_PROXY").ok();
    let username = env::var("CARAPAX_ACCESS_USERNAME").expect("CARAPAX_ACCESS_USERNAME");

    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }
    let api = Api::new(config).expect("Failed to create API");
    let mut context = Context::default();
    context.insert(api.clone());
    let policy = InMemoryAccessPolicy::from(vec![AccessRule::allow_user(username)]);
    let mut dispatcher = Dispatcher::new(context);
    dispatcher.add_handler(update_handler.access(policy));
    LongPoll::new(api, dispatcher).run().await
}
