use carapax::{
    access::{AccessExt, AccessRule, InMemoryAccessPolicy},
    types::Update,
    Chain,
};

pub fn setup(chain: Chain, username: &str) -> Chain {
    let policy = InMemoryAccessPolicy::from(vec![AccessRule::allow_user(username)]);
    chain.with(log_protected.access(policy))
}

async fn log_protected(update: Update) {
    log::info!("Got a new update in protected handler: {:?}", update);
}
