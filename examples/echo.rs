use carapax::{
    prelude::*,
    ratelimit::{nonzero, RateLimitMiddleware},
};
use dotenv::dotenv;
use env_logger;
use futures::Future;
use log;
use std::env;

fn handle_message(api: &mut Api, message: &Message) -> HandlerFuture {
    log::info!("got a message: {:?}\n", message);
    if let Some(text) = message.get_text() {
        let chat_id = message.get_chat_id();
        let method = SendMessage::new(chat_id, text.data.clone());
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

    let api = Api::new(token, proxy).unwrap();
    let app = App::new(api.clone());

    // take 1 update per 5 seconds
    let rate_limit = RateLimitMiddleware::direct(nonzero!(1u32), 5);

    app.add_middleware(rate_limit)
        .add_handler(Handler::message(handle_message))
        .run(UpdateMethod::poll(UpdatesStream::new(api)));
}
