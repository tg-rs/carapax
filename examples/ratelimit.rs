use carapax::{
    prelude::*,
    ratelimit::{nonzero, RateLimitMiddleware},
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

    let mut app = App::new(token);

    if let Some(proxy) = proxy {
        app = app.proxy(proxy);
    }

    // take 1 update per 5 seconds
    let middleware = RateLimitMiddleware::direct(nonzero!(1u32), 5);

    app.add_middleware(middleware)
        .add_handler(Handler::message(handle_message))
        .run(RunMethod::poll())
        .expect("Failed to run app");
}
