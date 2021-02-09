use crate::longpoll::LongPoll;
use crate::types::Update;
use crate::webhook::HyperError;
use crate::{Api, Config, Dispatcher, UpdateHandler};
use std::env;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::Mutex;

pub struct App {
    dispatcher: Dispatcher,
}

impl App {
    pub fn new(dispatcher: Dispatcher) -> Self {
        Self { dispatcher }
    }

    pub fn from_env() -> Self {
        let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
        let mut config = Config::new(token);

        let proxy = env::var("CARAPAX_PROXY").ok();
        if let Some(proxy) = proxy {
            config = config.proxy(proxy).expect("Failed to set proxy");
        }

        let host = env::var("CARAPAX_HOST").ok();
        if let Some(host) = host {
            config = config.host(host);
        }

        let api = Api::new(config).expect("Failed to create API");

        Self {
            dispatcher: Dispatcher::new(api),
        }
    }

    pub fn with_dispatcher<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Dispatcher),
    {
        f(&mut self.dispatcher);
        self
    }

    pub fn long_poll(self) -> LongPoll<Dispatcher> {
        LongPoll::new(self.dispatcher.get_api(), self.dispatcher)
    }

    pub async fn webhook<A, P>(self, address: A, path: P) -> Result<(), HyperError>
    where
        A: Into<SocketAddr>,
        P: Into<String>,
    {
        struct SyncedUpdateHandler {
            dispatcher: Arc<Mutex<Dispatcher>>,
        }

        impl UpdateHandler for SyncedUpdateHandler {
            type Future = SyncedUpdateHandlerFuture;

            fn handle(&self, update: Update) -> Self::Future {
                SyncedUpdateHandlerFuture {
                    dispatcher: self.dispatcher.clone(),
                    update: Some(update),
                }
            }
        }

        struct SyncedUpdateHandlerFuture {
            dispatcher: Arc<Mutex<Dispatcher>>,
            update: Option<Update>,
        }

        impl Future for SyncedUpdateHandlerFuture {
            type Output = ();

            fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
                let this = self.get_mut();

                let fut = this.dispatcher.lock();
                tokio::pin!(fut);
                let guard = futures::ready!(fut.poll(cx));
                let update = this.update.take().unwrap();
                let fut = guard.handle(update);
                tokio::pin!(fut);
                fut.poll(cx)
            }
        }

        tgbot::webhook::run_server(
            address,
            path,
            SyncedUpdateHandler {
                dispatcher: Arc::new(Mutex::new(self.dispatcher)),
            },
        )
        .await
    }
}
