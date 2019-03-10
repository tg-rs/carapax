use crate::{
    executor::{default_executor, proxy_executor, Executor},
    handlers::{run_server, UpdateMethod},
    methods::Method,
    types::Response,
    UpdateHandler, UpdatesStream,
};
use failure::Error;
use futures::{future, Future, Poll, Stream};
use serde::de::DeserializeOwned;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

/// Telegram Bot API client
#[derive(Clone)]
pub struct Api {
    executor: Arc<Box<Executor>>,
    token: String,
}

impl Api {
    /// Creates a client
    pub fn new<S: Into<String>>(token: S) -> Result<Self, Error> {
        Ok(Api {
            executor: Arc::new(default_executor()?),
            token: token.into(),
        })
    }

    /// Creates a client with specified proxy
    ///
    /// Proxy format:
    /// * http://[user:password]host:port
    /// * https://[user:password]@host:port
    /// * socks4://userid@host:port
    /// * socks5://[user:password]@host:port
    pub fn with_proxy<S: Into<String>>(token: S, url: &str) -> Result<Self, Error> {
        Ok(Api {
            executor: Arc::new(proxy_executor(url)?),
            token: token.into(),
        })
    }

    /// Executes a method
    pub fn execute<M: Method>(&self, method: &M) -> ApiFuture<M::Response>
    where
        M::Response: DeserializeOwned + Send + 'static,
    {
        let executor = self.executor.clone();
        ApiFuture {
            inner: Box::new(
                future::result(method.get_request().map(|builder| builder.build(&self.token)))
                    .and_then(move |req| executor.execute(req).from_err())
                    .and_then(|data| future::result(serde_json::from_slice::<Response<M::Response>>(&data)).from_err())
                    .and_then(|rep| {
                        future::result(match rep {
                            Response::Success(obj) => Ok(obj),
                            Response::Error(err) => Err(err.into()),
                        })
                    }),
            ),
        }
    }

    /// Run getting updates using update method and handler
    pub fn get_updates<H>(&self, update_method: UpdateMethod, handler: H)
    where
        H: UpdateHandler + Send + Sync + 'static,
    {
        match update_method {
            UpdateMethod::Polling => {
                let handler = Arc::new(Mutex::new(handler));
                let handler_clone = handler.clone();
                tokio::run(
                    UpdatesStream::new(self.clone())
                        .for_each(move |update| {
                            handler_clone.lock().unwrap().handle(update);
                            Ok(())
                        })
                        .then(|_| Ok(())),
                );
            }
            UpdateMethod::Webhook { addr, path } => {
                run_server(addr, path, handler);
            }
        }
    }

    /// Spawns a future on the default executor.
    pub fn spawn<F, T, E: Debug>(&self, f: F)
    where
        F: Future<Item = T, Error = E> + 'static + Send,
    {
        tokio::spawn(f.then(|r| {
            if let Err(e) = r {
                log::error!("An error has occurred: {:?}", e)
            }
            Ok(())
        }));
    }
}

/// An API future
#[must_use = "futures do nothing unless polled"]
pub struct ApiFuture<T> {
    inner: Box<Future<Item = T, Error = Error> + Send>,
}

impl<T> Future for ApiFuture<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}
