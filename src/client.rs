use crate::methods::{Method, Request, RequestBody, RequestError, RequestMethod};
use crate::types::{Response, ResponseError};
use curl::{
    easy::{Easy, List},
    Error as CurlError,
};
use serde::de::DeserializeOwned;
use serde_json::Error as JsonError;

/// Telegram Bot API client
#[derive(Debug)]
pub struct Client {
    curl: Easy,
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
            curl: Easy::new(),
            token: token.into(),
        }
    }

    /// Provide the URL of a proxy to use
    ///
    /// By default this option is not set and corresponds to CURLOPT_PROXY
    pub fn proxy(&mut self, url: &str) -> Result<&mut Self, ClientError> {
        self.curl.proxy(url)?;
        Ok(self)
    }

    /// Executes a method
    pub fn execute<M: Method>(&mut self, method: &M) -> Result<M::Response, ClientError>
    where
        M::Response: DeserializeOwned,
    {
        let data = self.request(method.get_request()?)?;
        let rep: Response<M::Response> = serde_json::from_slice(&data)?;
        match rep {
            Response::Success(obj) => Ok(obj),
            Response::Error(err) => Err(err.into()),
        }
    }

    fn request(&mut self, request: Request) -> Result<Vec<u8>, ClientError> {
        let url = request.url.build(&self.token);
        self.curl.url(&url)?;
        match request.method {
            RequestMethod::Get => self.curl.get(true)?,
            RequestMethod::Post => self.curl.post(true)?,
        }
        match request.body {
            RequestBody::Json(data) => {
                self.curl.post_fields_copy(&data)?;
                let mut headers = List::new();
                headers.append("Content-Type: application/json")?;
                self.curl.http_headers(headers)?;
            }
            RequestBody::Empty => {
                // no op
            }
        }
        let mut out = Vec::new();
        {
            let mut transfer = self.curl.transfer();
            transfer.write_function(|data| {
                out.extend_from_slice(data);
                Ok(data.len())
            })?;
            transfer.perform()?;
        }
        Ok(out)
    }
}

#[derive(Debug, Fail)]
pub enum ClientError {
    #[fail(display = "Curl error: {}", _0)]
    Curl(#[fail(cause)] CurlError),
    #[fail(display = "JSON error: {}", _0)]
    Json(#[fail(cause)] JsonError),
    #[fail(display = "Request error: {}", _0)]
    Request(#[fail(cause)] RequestError),
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
