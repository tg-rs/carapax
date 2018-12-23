use serde::ser::Serialize;
use serde_json::Error as JsonError;
use std::fmt::Display;

const BASE_URL: &str = "https://api.telegram.org";

/// Represents an API method
pub trait Method {
    /// Type of successful result in API response
    type Response;

    /// Returns information about HTTP request
    fn get_request(&self) -> Result<Request, RequestError>;
}

/// Information about HTTP request
#[derive(Clone, Debug)]
pub struct Request {
    pub(crate) method: RequestMethod,
    pub(crate) url: RequestUrl,
    pub(crate) body: RequestBody,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub(crate) enum RequestMethod {
    Get,
    Post,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub(crate) struct RequestUrl(&'static str);

impl RequestUrl {
    pub fn new(path: &'static str) -> Self {
        RequestUrl(path)
    }

    pub fn build(&self, token: impl Display) -> String {
        format!("{}/bot{}/{}", BASE_URL, token, self.0)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RequestBody {
    Json(Vec<u8>),
    Empty,
}

impl RequestBody {
    pub fn json(s: &impl Serialize) -> Result<RequestBody, RequestError> {
        Ok(RequestBody::Json(
            serde_json::to_vec(s).map_err(RequestError::Json)?,
        ))
    }
}

/// Request error
#[derive(Debug, Fail)]
pub enum RequestError {
    /// Can not serialize JSON
    #[fail(display = "Can not serialize request to JSON: {}", _0)]
    Json(#[cause] JsonError),
}
