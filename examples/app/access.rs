//! # Access control
//!
//! Here we create an access policy that determines
//! whether access should be granted or denied based on the [`carapax::HandlerInput`].
//!
//! You can implement your own policy using [`carapax::access::AccessPolicy`] trait.
//!
//! After we wrap the [`log_protected`] handler with the [`carapax::access::AccessPredicate`].
//! This ensures that the handler executes only when the access policy grants permission.
//!
//! Note that you need to enable the `access` feature in your `Cargo.toml`.
use carapax::{
    access::{AccessExt, AccessRule, InMemoryAccessPolicy},
    types::Update,
    Chain,
};

pub fn setup(chain: Chain, username: &str) -> Chain {
    let policy = InMemoryAccessPolicy::from(vec![AccessRule::allow_user(username)]);
    chain.with(log_protected.with_access_policy(policy))
}

async fn log_protected(update: Update) {
    log::info!("Got a new update in the protected handler: {:?}", update);
}
