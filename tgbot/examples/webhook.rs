use dotenv::dotenv;
use env_logger;
use log;
use tgbot::{handle_updates, types::Update, UpdateHandler, UpdateMethod};

struct Handler;

impl UpdateHandler for Handler {
    fn handle(&mut self, update: Update) {
        log::info!("got an update: {:?}\n", update);
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();
    tokio::run(handle_updates(
        UpdateMethod::webhook(([127, 0, 0, 1], 8080), "/"),
        Handler,
    ));
}
