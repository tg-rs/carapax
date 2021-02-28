use carapax::{
    access::{AccessRule, InMemoryAccessPolicy},
    longpoll::LongPoll,
    types::Message,
    Api, Config, Dispatcher, Handler,
};
use dotenv::dotenv;
use std::env;

async fn handle_message(message: Message) {
    log::info!("Got a new message: {:?}", message);
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

    // Deny from all except for @username (specify without @)
    let rule = AccessRule::allow_user(username);
    let policy = InMemoryAccessPolicy::default().push_rule(rule);

    let mut dispatcher = Dispatcher::new(api.clone());
    dispatcher.add_handler(policy.access());
    dispatcher.add_handler(handle_message);
    LongPoll::new(api, dispatcher).run().await
}
