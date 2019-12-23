use crate::{
    methods::Method,
    request::{RequestBody, RequestBuilder, RequestMethod},
    types::Response,
};
use bytes::Bytes;
use failure::{format_err, Error};
use log::{debug, log_enabled, Level};
use reqwest::{Client, ClientBuilder, Proxy};
use serde::de::DeserializeOwned;
use std::{fmt::Debug, sync::Arc};
use url::Url;

const DEFAULT_HOST: &str = "https://api.telegram.org";

/// An API config
#[derive(Debug, Clone)]
pub struct Config {
    host: String,
    token: String,
    proxy: Option<Proxy>,
}

impl Config {
    /// Creates a new configuration with a given token
    pub fn new<S: Into<String>>(token: S) -> Self {
        Self {
            token: token.into(),
            host: String::from(DEFAULT_HOST),
            proxy: None,
        }
    }

    /// Sets an API host
    ///
    /// https://api.telegram.org is used by default
    pub fn host<S: Into<String>>(mut self, host: S) -> Self {
        self.host = host.into();
        self
    }

    /// Sets a proxy to config
    ///
    /// Proxy format:
    /// * http://\[user:password\]@host:port
    /// * https://\[user:password\]@host:port
    /// * socks5://\[user:password\]@host:port
    pub fn proxy<U: AsRef<str>>(mut self, url: U) -> Result<Self, Error> {
        let raw_url = url.as_ref();
        let url = Url::parse(raw_url)?;
        if url.has_authority() {
            let mut base_url = url.clone();
            base_url
                .set_username("")
                .map_err(|()| format_err!("Failed to remove username"))?;
            base_url
                .set_password(None)
                .map_err(|()| format_err!("Failed to remove password"))?;
            self.proxy = Some(Proxy::all(base_url.as_str())?.basic_auth(url.username(), url.password().unwrap_or("")));
        } else {
            self.proxy = Some(Proxy::all(raw_url)?);
        }
        Ok(self)
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
    client: Arc<Client>,
    host: String,
    token: String,
}

impl Api {
    /// Creates a API instance with a given configuration.
    pub fn new<C: Into<Config>>(config: C) -> Result<Self, Error> {
        let config = config.into();

        let mut builder = ClientBuilder::new();
        builder = if let Some(proxy) = config.proxy {
            builder.proxy(proxy)
        } else {
            builder.no_proxy()
        };
        let client = builder.build()?;

        Ok(Api {
            client: Arc::new(client),
            host: config.host,
            token: config.token,
        })
    }

    /// Downloads a file
    ///
    /// Use getFile method in order to get value for file_path argument
    pub async fn download_file<P: AsRef<str>>(&self, file_path: P) -> Result<Bytes, Error> {
        let req = RequestBuilder::empty(file_path.as_ref())?.build(&format!("{}/file", &self.host), &self.token);
        debug!("Downloading file from {}", req.url);
        let rep = self.client.get(&req.url).send().await?;
        if !rep.status().is_success() {
            Err(format_err!("Failed to download file: {}", rep.text().await?))
        } else {
            Ok(rep.bytes().await?)
        }
    }

    /// Executes a method
    pub async fn execute<M: Method>(&self, method: M) -> Result<M::Response, Error>
    where
        M::Response: DeserializeOwned + Send + 'static,
    {
        let req = method.into_request()?.build(&self.host, &self.token);

        let http_req = match req.method {
            RequestMethod::Get => {
                debug!("Execute GET {}", req.url);
                self.client.get(&req.url)
            }
            RequestMethod::Post => {
                debug!("Execute POST: {}", req.url);
                self.client.post(&req.url)
            }
        };
        let rep = match req.body {
            RequestBody::Form(form) => {
                let form = form.into_multipart().await?;
                debug!("Sending multipart body: {:?}", form);
                http_req.multipart(form)
            }
            RequestBody::Json(data) => {
                if log_enabled!(Level::Debug) {
                    debug!("Sending JSON body: {:?}", String::from_utf8(data.clone()));
                }
                http_req.header("Content-Type", "application/json").body(data)
            }
            RequestBody::Empty => {
                debug!("Sending empty body");
                http_req
            }
        }
        .send()
        .await?;
        let data = rep.json::<Response<M::Response>>().await?;
        match data {
            Response::Success(obj) => Ok(obj),
            Response::Error(err) => Err(err.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config() {
        // TODO: test socks when reqwest feature will be available
        let config = Config::new("token");
        // .proxy("socks5://user:password@127.0.0.1:1234")
        // .unwrap();
        assert_eq!(config.token, "token");
        assert_eq!(config.host, DEFAULT_HOST);
        // assert!(config.proxy.is_some());

        let config = Config::new("token")
            .host("https://example.com")
            .proxy("http://127.0.0.1:1234")
            .unwrap();
        assert_eq!(config.token, "token");
        assert_eq!(config.host, "https://example.com");
        assert!(config.proxy.is_some());

        let config = Config::new("token").proxy("https://127.0.0.1:1234").unwrap();
        assert_eq!(config.token, "token");
        assert_eq!(config.host, DEFAULT_HOST);
        assert!(config.proxy.is_some());

        let config = Config::new("token");
        assert_eq!(config.token, "token");
        assert_eq!(config.host, DEFAULT_HOST);
        assert!(config.proxy.is_none());
    }

    #[test]
    fn api() {
        let config = Config::new("token")
            .host("https://example.com")
            .proxy("http://user:password@127.0.0.1:1234")
            .unwrap();
        let api = Api::new(config).unwrap();
        assert_eq!(api.host, "https://example.com");
        assert_eq!(api.token, "token");

        let config = Config::new("token");
        let api = Api::new(config).unwrap();
        assert_eq!(api.host, DEFAULT_HOST);
        assert_eq!(api.token, "token");
    }
}
