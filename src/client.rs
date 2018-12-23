use crate::methods::{Method, Request, RequestBody, RequestError, RequestMethod};
use crate::poll::UpdatesIter;
use crate::types::{Response, ResponseError};
use curl::{
    easy::{Easy, List},
    Error as CurlError,
};
use log::{debug, log_enabled, Level::Debug};
use serde::de::DeserializeOwned;
use serde_json::Error as JsonError;
use std::cell::RefCell;

/// Telegram Bot API client
#[derive(Debug)]
pub struct Client {
    curl: RefCell<Easy>,
    token: String,
}

impl Client {
    /// Creates a new Client
    ///
    /// # Arguments
    ///
    /// * token - Bot token
    pub fn new<S: Into<String>>(token: S) -> Self {
        Client {
            curl: RefCell::new(Easy::new()),
            token: token.into(),
        }
    }

    /// Provide the URL of a proxy to use
    ///
    /// By default this option is not set and corresponds to CURLOPT_PROXY
    pub fn proxy(&mut self, url: &str) -> Result<&mut Self, ClientError> {
        debug!("Setting proxy: {}", url);
        self.curl.borrow_mut().proxy(url)?;
        Ok(self)
    }

    /// Executes a method
    pub fn execute<M: Method>(&self, method: &M) -> Result<M::Response, ClientError>
    where
        M::Response: DeserializeOwned,
    {
        let data = self.request(&method.get_request()?)?;
        let rep: Response<M::Response> = serde_json::from_slice(&data)?;
        match rep {
            Response::Success(obj) => Ok(obj),
            Response::Error(err) => Err(err.into()),
        }
    }

    /// Returns an iterator over updates
    pub fn get_updates(&self) -> UpdatesIter {
        UpdatesIter::new(self)
    }

    fn request(&self, request: &Request) -> Result<Vec<u8>, ClientError> {
        let url = request.url.build(&self.token);
        debug!("Sending request: {:?}", request);
        let mut curl = self.curl.borrow_mut();
        curl.url(&url)?;
        match request.method {
            RequestMethod::Get => curl.get(true)?,
            RequestMethod::Post => curl.post(true)?,
        }
        match request.body {
            RequestBody::Json(ref data) => {
                if log_enabled!(Debug) {
                    debug!("Post JSON data: {}", String::from_utf8_lossy(data));
                }
                curl.post_fields_copy(data)?;
                let mut headers = List::new();
                headers.append("Content-Type: application/json")?;
                curl.http_headers(headers)?;
            }
            RequestBody::Empty => {
                // no op
            }
        }
        let mut out = Vec::new();
        {
            let mut transfer = curl.transfer();
            transfer.write_function(|data| {
                out.extend_from_slice(data);
                Ok(data.len())
            })?;
            transfer.perform()?;
        }
        if log_enabled!(Debug) {
            debug!(
                "Got response: {} for request: {:?}",
                String::from_utf8_lossy(&out),
                request
            );
        }
        Ok(out)
    }
}

/// Client error
#[derive(Debug, Fail)]
pub enum ClientError {
    /// Curl error
    #[fail(display = "Curl error: {}", _0)]
    Curl(#[fail(cause)] CurlError),
    /// JSON error
    #[fail(display = "JSON error: {}", _0)]
    Json(#[fail(cause)] JsonError),
    /// Can not create request
    #[fail(display = "Request error: {}", _0)]
    Request(#[fail(cause)] RequestError),
    /// Telegram API respond with a error
    #[fail(display = "Telegram error: {:?}", _0)]
    Telegram(ResponseError),
}

impl From<CurlError> for ClientError {
    fn from(err: CurlError) -> ClientError {
        ClientError::Curl(err)
    }
}

impl From<JsonError> for ClientError {
    fn from(err: JsonError) -> ClientError {
        ClientError::Json(err)
    }
}

impl From<RequestError> for ClientError {
    fn from(err: RequestError) -> ClientError {
        ClientError::Request(err)
    }
}

impl From<ResponseError> for ClientError {
    fn from(err: ResponseError) -> ClientError {
        ClientError::Telegram(err)
    }
}
