use failure::Error;
use serde::ser::Serialize;

/// Represents an API method
pub trait Method {
    /// Type of successful result in API response
    type Response;

    /// Returns information about HTTP request
    fn get_request(&self) -> Result<RequestBuilder, Error>;
}

/// A request builder
#[derive(Clone, Debug)]
pub struct RequestBuilder {
    method: RequestMethod,
    url: RequestUrl,
    body: RequestBody,
}

impl RequestBuilder {
    pub(crate) fn json<S: Into<String>>(path: S, s: &impl Serialize) -> Result<RequestBuilder, Error> {
        Ok(RequestBuilder {
            method: RequestMethod::Post,
            body: RequestBody::Json(serde_json::to_vec(s)?),
            url: RequestUrl(path.into()),
        })
    }

    pub(crate) fn empty<S: Into<String>>(path: S) -> Result<RequestBuilder, Error> {
        Ok(RequestBuilder {
            method: RequestMethod::Get,
            body: RequestBody::Empty,
            url: RequestUrl(path.into()),
        })
    }

    pub(crate) fn build(self, base_url: &str, token: &str) -> Request {
        Request {
            method: self.method,
            url: self.url.build(base_url, token),
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
struct RequestUrl(String);

impl RequestUrl {
    fn build(&self, base_url: &str, token: &str) -> String {
        format!("{}/bot{}/{}", base_url, token, self.0)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum RequestBody {
    Json(Vec<u8>),
    Empty,
}
