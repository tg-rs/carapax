use carapax::{
    access::{AccessExt, AccessRule, InMemoryAccessPolicy},
    types::Update,
    Chain, HandlerResult,
};

pub fn setup(chain: &mut Chain, username: &str) {
    let policy = InMemoryAccessPolicy::from(vec![AccessRule::allow_user(username)]);
    chain.add_handler(log_protected.access(policy));
}

async fn log_protected(update: Update) -> HandlerResult {
    log::info!("Got a new update in protected handler: {:?}", update);
    HandlerResult::Continue
}
