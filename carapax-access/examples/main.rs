use carapax::prelude::*;
use carapax_access::{AccessHandler, AccessRule, InMemoryAccessPolicy};
use dotenv::dotenv;
use env_logger;
use std::env;

fn handle_message(_context: &mut Context, message: &Message) -> HandlerFuture {
    log::info!("Got a new message: {:?}", message);
    HandlerResult::Continue.into()
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let proxy = env::var("CARAPAX_PROXY").ok();
    let username = env::var("CARAPAX_DENY_USERNAME").expect("CARAPAX_DENY_USERNAME");

    let api = Api::new(token, proxy).unwrap();

    // Deny from all except for @username (specify without @)
    let rule = AccessRule::allow_user(username);
    let policy = InMemoryAccessPolicy::default().push_rule(rule);

    tokio::run(
        App::new()
            .add_handler(Handler::update(AccessHandler::new(policy)))
            .add_handler(Handler::message(handle_message))
            .run(api.clone(), UpdateMethod::poll(UpdatesStream::new(api))),
    )
}
