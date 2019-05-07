use crate::{
    executor::{default_executor, proxy_executor, Executor},
    methods::Method,
    request::RequestBuilder,
    types::Response,
};
use failure::Error;
use futures::{future, Future, Poll};
use serde::de::DeserializeOwned;
use std::{fmt::Debug, sync::Arc};

const DEFAULT_HOST: &str = "https://api.telegram.org";

/// An API config
#[derive(Debug, Clone)]
pub struct Config {
    host: Option<String>,
    token: String,
    proxy: Option<String>,
}

impl Config {
    /// Creates a new configuration with a given token
    pub fn new<S: Into<String>>(token: S) -> Self {
        Self {
            token: token.into(),
            host: None,
            proxy: None,
        }
    }

    /// Sets an API host
    ///
    /// https://api.telegram.org is used by default
    pub fn host<S: Into<String>>(mut self, host: S) -> Self {
        self.host = Some(host.into());
        self
    }

    /// Sets a proxy to config
    ///
    /// Proxy format:
    /// * http://\[user:password\]@host:port
    /// * https://\[user:password\]@host:port
    /// * socks4://userid@host:port
    /// * socks5://\[user:password\]@host:port
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
    executor: Arc<Box<dyn Executor>>,
    host: String,
    token: String,
}

impl Api {
    /// Creates a API instance with a given configuration.
    pub fn new<C: Into<Config>>(config: C) -> Result<Self, Error> {
        let config = config.into();
        Ok(Api {
            executor: Arc::new(if let Some(ref proxy) = config.proxy {
                proxy_executor(proxy)?
            } else {
                default_executor()?
            }),
            host: config.host.unwrap_or_else(|| String::from(DEFAULT_HOST)),
            token: config.token,
        })
    }

    /// Downloads a file
    ///
    /// Use getFile method in order to get value for file_path argument
    pub fn download_file<P: AsRef<str>>(&self, file_path: P) -> ApiFuture<Vec<u8>> {
        let executor = self.executor.clone();
        ApiFuture {
            inner: Box::new(
                future::result(
                    RequestBuilder::empty(file_path.as_ref())
                        .map(|builder| builder.build(&format!("{}/file", &self.host), &self.token)),
                )
                .and_then(move |req| executor.execute(req)),
            ),
        }
    }

    /// Executes a method
    pub fn execute<M: Method>(&self, method: M) -> ApiFuture<M::Response>
    where
        M::Response: DeserializeOwned + Send + 'static,
    {
        let executor = self.executor.clone();
        ApiFuture {
            inner: Box::new(
                future::result(
                    method
                        .into_request()
                        .map(|builder| builder.build(&self.host, &self.token)),
                )
                .and_then(move |req| executor.execute(req))
                .and_then(|data| serde_json::from_slice::<Response<M::Response>>(&data).map_err(Error::from))
                .and_then(|rep| match rep {
                    Response::Success(obj) => Ok(obj),
                    Response::Error(err) => Err(err.into()),
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
    inner: Box<dyn Future<Item = T, Error = Error> + Send>,
}

impl<T> Future for ApiFuture<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api() {
        let config = Config::new("token")
            .host("https://example.com")
            .proxy("socks5://user:password@127.0.0.1:1234");
        let api = Api::new(config).unwrap();
        assert_eq!(api.host, "https://example.com");
        assert_eq!(api.token, "token");

        let config = Config::new("token");
        let api = Api::new(config).unwrap();
        assert_eq!(api.host, DEFAULT_HOST);
        assert_eq!(api.token, "token");
    }
}
