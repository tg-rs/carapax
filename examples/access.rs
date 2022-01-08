use carapax::{
    access::{AccessExt, AccessRule, InMemoryAccessPolicy},
    types::Update,
};
use dotenv::dotenv;

async fn update_handler(update: Update) {
    log::info!("Got a new update: {:?}", update);
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let username = helper::get_env("CARAPAX_ACCESS_USERNAME");
    let policy = InMemoryAccessPolicy::from(vec![AccessRule::allow_user(username)]);

    helper::run(update_handler.access(policy)).await;
}
