use crate::executor::{default_executor, proxy_executor, Executor, ExecutorError};
use crate::methods::{Method, RequestError};
use crate::poll::UpdatesStream;
use crate::types::{Response, ResponseError};
use futures::{future, Future, Poll};
use serde::de::DeserializeOwned;
use serde_json::Error as JsonError;
use std::rc::Rc;
use tokio_timer::Error as TimerError;

/// Telegram Bot API client
#[derive(Clone)]
pub struct Api {
    executor: Rc<Box<Executor>>,
    token: String,
}

impl Api {
    /// Creates a client
    pub fn create<S: Into<String>>(token: S) -> Result<Self, ApiError> {
        Ok(Api {
            executor: Rc::new(default_executor()?),
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
    pub fn with_proxy<S: Into<String>>(token: S, url: &str) -> Result<Self, ApiError> {
        Ok(Api {
            executor: Rc::new(proxy_executor(url)?),
            token: token.into(),
        })
    }

    /// Executes a method
    pub fn execute<M: Method>(&self, method: &M) -> ApiFuture<M::Response>
    where
        M::Response: DeserializeOwned,
    {
        let executor = self.executor.clone();
        ApiFuture {
            inner: Box::new(
                future::result(
                    method
                        .get_request()
                        .map(|builder| builder.build(&self.token)),
                )
                .from_err()
                .and_then(move |req| executor.execute(req).from_err())
                .and_then(|data| {
                    future::result(serde_json::from_slice::<Response<M::Response>>(&data))
                        .from_err()
                })
                .and_then(|rep| {
                    future::result(match rep {
                        Response::Success(obj) => Ok(obj),
                        Response::Error(err) => Err(err.into()),
                    })
                }),
            ),
        }
    }

    /// Returns an updates stream used for long polling
    pub fn get_updates(&self) -> UpdatesStream {
        UpdatesStream::new(self.clone())
    }
}

/// An API future
#[must_use = "futures do nothing unless polled"]
pub struct ApiFuture<T> {
    inner: Box<Future<Item = T, Error = ApiError>>,
}

impl<T> Future for ApiFuture<T> {
    type Item = T;
    type Error = ApiError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

/// Represents an API error
#[derive(Debug, Fail)]
pub enum ApiError {
    /// Failed to create client or execute a method
    #[fail(display = "Executor error: {}", _0)]
    Executor(#[fail(cause)] ExecutorError),
    /// Can not parse JSON response
    #[fail(display = "Failed to parse json: {}", _0)]
    Json(#[fail(cause)] JsonError),
    /// Failed to build a request
    #[fail(display = "Can not build request: {}", _0)]
    Request(#[fail(cause)] RequestError),
    /// A telegram server respond with error
    #[fail(display = "Telegram error: {:?}", _0)]
    Telegram(ResponseError),
    /// An error within tokio_timer
    #[fail(display = "Timer error: {:?}", _0)]
    Timer(#[fail(cause)] TimerError),
}

macro_rules! impl_from_api_error {
    ($to:ident($from:ty)) => {
        impl From<$from> for ApiError {
            fn from(err: $from) -> ApiError {
                ApiError::$to(err)
            }
        }
    };
}

impl_from_api_error!(Executor(ExecutorError));
impl_from_api_error!(Json(JsonError));
impl_from_api_error!(Request(RequestError));
impl_from_api_error!(Telegram(ResponseError));
impl_from_api_error!(Timer(TimerError));
