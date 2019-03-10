use carapax::{
    access::{AccessMiddleware, AccessRule, InMemoryAccessPolicy},
    prelude::*,
};
use dotenv::dotenv;
use env_logger;
use futures::Future;
use log;
use std::env;

fn handle_message(context: &Context, message: &Message) -> HandlerFuture {
    log::info!("got a message: {:?}\n", message);
    if let Some(text) = message.get_text() {
        let chat_id = message.get_chat_id();
        let method = SendMessage::new(chat_id, text.data.clone());
        let api: &Api = context.get();
        return HandlerFuture::new(api.execute(&method).then(|x| {
            log::info!("sendMessage result: {:?}\n", x);
            Ok(())
        }));
    }
    ().into()
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let proxy = env::var("CARAPAX_PROXY").ok();
    let allowed_username = env::var("CARAPAX_ALLOWED_USERNAME").expect("CARAPAX_ALLOWED_USERNAME is not set");

    let mut app = App::new(token);

    if let Some(proxy) = proxy {
        app = app.proxy(proxy);
    }

    // Deny from all except for allowed_username
    let rule = AccessRule::allow_user(allowed_username);
    let policy = InMemoryAccessPolicy::default().push_rule(rule);

    app.add_middleware(AccessMiddleware::new(policy))
        .add_handler(Handler::message(handle_message))
        .run(RunMethod::poll())
        .expect("Failed to start app");
}
