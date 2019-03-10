use dotenv::dotenv;
use env_logger;
use log;
use tgbot::{
    types::Update,
    webhook::{self, UpdateHandler},
};

struct Handler;

impl UpdateHandler for Handler {
    fn handle(&mut self, update: Update) {
        log::info!("got an update: {:?}\n", update);
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();
    webhook::run_server(([127, 0, 0, 1], 8080), "/", Handler);
}
