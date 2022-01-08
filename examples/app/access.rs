use carapax::{
    access::{AccessExt, AccessRule, InMemoryAccessPolicy},
    types::Update,
    Dispatcher,
};

pub fn setup(dispatcher: &mut Dispatcher, username: &str) {
    let policy = InMemoryAccessPolicy::from(vec![AccessRule::allow_user(username)]);
    dispatcher.add_handler(log_protected.access(policy));
}

async fn log_protected(update: Update) {
    log::info!("Got a new update in protected handler: {:?}", update);
}
