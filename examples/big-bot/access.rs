use crate::{get_env, util::Module};
use carapax::{
    access::{AccessExt, AccessRule, InMemoryAccessPolicy},
    types::Update,
    Dispatcher,
};

pub struct AccessModule;

impl Module for AccessModule {
    fn add_handlers(&self, dispatcher: &mut Dispatcher) {
        let username = get_env("CARAPAX_ACCESS_USERNAME");
        let policy = InMemoryAccessPolicy::from(vec![AccessRule::allow_user(username)]);

        dispatcher.add_handler(update_handler.access(policy));
    }
}

async fn update_handler(update: Update) {
    log::info!("Got a new update: {:?}", update);
}
