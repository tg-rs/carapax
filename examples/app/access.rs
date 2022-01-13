use carapax::{
    access::{AccessExt, AccessRule, InMemoryAccessPolicy},
    types::Update,
    DispatcherBuilder, HandlerResult,
};

pub fn setup(builder: &mut DispatcherBuilder, username: &str) {
    let policy = InMemoryAccessPolicy::from(vec![AccessRule::allow_user(username)]);
    builder.add_handler(log_protected.access(policy));
}

async fn log_protected(update: Update) -> HandlerResult {
    log::info!("Got a new update in protected handler: {:?}", update);
    HandlerResult::Continue
}
