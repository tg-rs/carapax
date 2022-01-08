#[cfg(feature = "session")]
pub mod session;

use carapax::{longpoll::LongPoll, Api, Config, Context, Dispatcher, Handler, HandlerResult, TryFromInput};
use std::{env, future::Future};

pub fn get_env(s: &str) -> String {
    env::var(s).unwrap_or_else(|_| panic!("{} is not set", s))
}

pub async fn run<H, I>(handler: H)
where
    H: Handler<I> + Sync + Clone + 'static,
    I: TryFromInput + Sync + 'static,
    <H::Future as Future>::Output: Into<HandlerResult>,
{
    RunnerBuilder::from_env().build().add_handler(handler).run().await;
}

pub struct RunnerBuilder {
    api: Api,
    context: Context,
}

impl RunnerBuilder {
    pub fn from_env() -> Self {
        let config = config_from_env();
        let api = Api::new(config).expect("Failed to create API");

        let mut context = Context::default();
        context.insert(api.clone());

        Self { api, context }
    }

    pub fn insert_data<T>(mut self, data: T) -> Self
    where
        T: Send + Sync + 'static,
    {
        self.context.insert(data);
        self
    }

    pub fn build(self) -> Runner {
        let dispatcher = Dispatcher::new(self.context);

        Runner {
            dispatcher,
            api: self.api,
        }
    }
}

pub struct Runner {
    api: Api,
    dispatcher: Dispatcher,
}

impl Runner {
    pub fn add_handler<H, I>(mut self, handler: H) -> Self
    where
        H: Handler<I> + Sync + Clone + 'static,
        I: TryFromInput + Sync + 'static,
        <H::Future as Future>::Output: Into<HandlerResult>,
    {
        self.dispatcher.add_handler(handler);
        self
    }

    pub async fn run(self) {
        LongPoll::new(self.api, self.dispatcher).run().await
    }
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
