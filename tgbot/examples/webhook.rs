use async_trait::async_trait;
use dotenv::dotenv;
use env_logger;
use log::info;
use std::convert::Infallible;
use tgbot::{types::Update, webhook, UpdateHandler};

struct Handler;

#[async_trait]
impl UpdateHandler for Handler {
    type Error = Infallible;

    async fn handle(&mut self, update: Update) -> Result<(), Self::Error> {
        info!("got an update: {:?}\n", update);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    webhook::run_server(([127, 0, 0, 1], 8080), "/", Handler).await.unwrap();
}
