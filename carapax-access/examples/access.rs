use carapax::prelude::*;
use carapax_access::{AccessHandler, AccessRule, InMemoryAccessPolicy};
use dotenv::dotenv;
use env_logger;
use std::env;

fn handle_message(_context: &mut Context, message: Message) {
    log::info!("Got a new message: {:?}", message);
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();
    let username = env::var("TGRS_ACCESS_USERNAME").expect("TGRS_ACCESS_USERNAME");

    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy);
    }

    let api = Api::new(config).unwrap();

    // Deny from all except for @username (specify without @)
    let rule = AccessRule::allow_user(username);
    let policy = InMemoryAccessPolicy::default().push_rule(rule);

    tokio::run(
        App::new()
            .add_handler(AccessHandler::new(policy))
            .add_handler(FnHandler::from(handle_message))
            .run(api.clone(), UpdateMethod::poll(UpdatesStream::new(api))),
    )
}
