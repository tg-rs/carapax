use crate::{
    executor::{default_executor, proxy_executor, Executor},
    methods::Method,
    types::Response,
};
use failure::Error;
use futures::{future, Future, Poll};
use serde::de::DeserializeOwned;
use std::{fmt::Debug, sync::Arc};

/// Telegram Bot API client
#[derive(Clone)]
pub struct Api {
    executor: Arc<Box<Executor>>,
    token: String,
}

impl Api {
    /// Creates a client
    ///
    /// # Arguments
    ///
    /// * token - Bot API token
    /// * proxy - Optional proxy
    ///
    /// Proxy format:
    /// * http://[user:password]host:port
    /// * https://[user:password]@host:port
    /// * socks4://userid@host:port
    /// * socks5://[user:password]@host:port
    pub fn new<T, P>(token: T, proxy: Option<P>) -> Result<Self, Error>
    where
        T: Into<String>,
        P: AsRef<str>,
    {
        Ok(Api {
            executor: Arc::new(if let Some(proxy) = proxy {
                proxy_executor(proxy.as_ref())?
            } else {
                default_executor()?
            }),
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
