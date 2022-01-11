mod access;
mod command;
mod dialogue;
mod pingpong;
mod ratelimit;
mod session;
mod util;

use crate::{
    access::AccessModule,
    command::CommandModule,
    dialogue::DialogueModule,
    pingpong::PingPongModule,
    ratelimit::RateLimitModule,
    session::SessionModule,
    util::{get_env, Module},
};
use carapax::{longpoll::LongPoll, Api, Config, Context, Dispatcher};
use dotenv::dotenv;
use std::env;

const MODULES: &[&dyn Module] = &[
    &AccessModule,
    &CommandModule,
    &PingPongModule,
    &RateLimitModule,
    &SessionModule,
    &DialogueModule,
];

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let config = config_from_env();
    let api = Api::new(config).expect("Failed to create API");

    let mut context = Context::default();
    context.insert(api.clone());
    util::insert_fs_backend(&mut context);

    let mut dispatcher = Dispatcher::new(context);
    for &module in MODULES {
        module.add_handlers(&mut dispatcher);
    }

    LongPoll::new(api, dispatcher).run().await
}

fn config_from_env() -> Config {
    let token = get_env("CARAPAX_TOKEN");
    let proxy = env::var("CARAPAX_PROXY").ok();

    let mut config = Config::new(token);

    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }

    config
}
