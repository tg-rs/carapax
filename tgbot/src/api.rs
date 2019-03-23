use crate::{
    executor::{default_executor, proxy_executor, Executor},
    methods::Method,
    types::Response,
};
use failure::Error;
use futures::{future, Future, Poll};
use serde::de::DeserializeOwned;
use std::{fmt::Debug, sync::Arc};

/// An API config
#[derive(Debug, Clone)]
pub struct Config {
    token: String,
    proxy: Option<String>,
}

impl Config {
    /// Creates a new config with given token
    pub fn new<S: Into<String>>(token: S) -> Self {
        Self {
            token: token.into(),
            proxy: None,
        }
    }

    /// Sets a proxy to config
    ///
    /// Proxy format:
    /// * http://[user:password]host:port
    /// * https://[user:password]@host:port
    /// * socks4://userid@host:port
    /// * socks5://[user:password]@host:port
    pub fn proxy<S: Into<String>>(mut self, proxy: S) -> Self {
        self.proxy = Some(proxy.into());
        self
    }
}

impl<S> From<S> for Config
where
    S: Into<String>,
{
    fn from(token: S) -> Self {
        Config::new(token.into())
    }
}

/// Telegram Bot API client
#[derive(Clone)]
pub struct Api {
    executor: Arc<Box<Executor>>,
    token: String,
}

impl Api {
    /// Creates a new client
    pub fn new<C: Into<Config>>(config: C) -> Result<Self, Error> {
        let config = config.into();
        Ok(Api {
            executor: Arc::new(if let Some(ref proxy) = config.proxy {
                proxy_executor(proxy)?
            } else {
                default_executor()?
            }),
            token: config.token,
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

    /// Spawns a future on the default executor.
    pub fn spawn<F, T, E: Debug>(&self, f: F)
    where
        F: Future<Item = T, Error = E> + 'static + Send,
    {
        tokio_executor::spawn(f.then(|r| {
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
