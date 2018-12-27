use serde::ser::Serialize;
use serde_json::Error as JsonError;
use std::fmt::Display;

const BASE_URL: &str = "https://api.telegram.org";

/// Represents an API method
pub trait Method {
    /// Type of successful result in API response
    type Response: 'static;

    /// Returns information about HTTP request
    fn get_request(&self) -> Result<RequestBuilder, RequestError>;
}

/// A request builder
#[derive(Clone, Debug)]
pub struct RequestBuilder {
    method: RequestMethod,
    url: RequestUrl,
    body: RequestBody,
}

impl RequestBuilder {
    pub(crate) fn json(
        path: &'static str,
        s: &impl Serialize,
    ) -> Result<RequestBuilder, RequestError> {
        Ok(RequestBuilder {
            method: RequestMethod::Post,
            body: RequestBody::Json(serde_json::to_vec(s).map_err(RequestError::Json)?),
            url: RequestUrl(path),
        })
    }

    pub(crate) fn empty(path: &'static str) -> Result<RequestBuilder, RequestError> {
        Ok(RequestBuilder {
            method: RequestMethod::Get,
            body: RequestBody::Empty,
            url: RequestUrl(path),
        })
    }

    pub(crate) fn build(self, token: &str) -> Request {
        Request {
            method: self.method,
            url: self.url.build(token),
            body: self.body,
        }
    }
}

/// Information about HTTP request
#[derive(Clone, Debug)]
pub(crate) struct Request {
    pub(crate) method: RequestMethod,
    pub(crate) url: String,
    pub(crate) body: RequestBody,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub(crate) enum RequestMethod {
    Get,
    Post,
}

#[derive(Clone, Debug)]
struct RequestUrl(&'static str);

impl RequestUrl {
    fn build(&self, token: impl Display) -> String {
        format!("{}/bot{}/{}", BASE_URL, token, self.0)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RequestBody {
    Json(Vec<u8>),
    Empty,
}

/// Request error
#[derive(Debug, Fail)]
pub enum RequestError {
    /// Can not serialize JSON
    #[fail(display = "Can not serialize request to JSON: {}", _0)]
    Json(#[cause] JsonError),
}
