use failure::Error;
use serde::ser::Serialize;

mod form;

pub(crate) use self::form::*;

/// A request builder
#[derive(Debug)]
pub struct RequestBuilder {
    method: RequestMethod,
    path: String,
    body: RequestBody,
}

impl RequestBuilder {
    pub(crate) fn form<S: Into<String>>(path: S, form: Form) -> Result<RequestBuilder, Error> {
        Ok(RequestBuilder {
            method: RequestMethod::Post,
            body: RequestBody::Form(form),
            path: path.into(),
        })
    }

    pub(crate) fn json<S: Into<String>>(path: S, s: &impl Serialize) -> Result<RequestBuilder, Error> {
        Ok(RequestBuilder {
            method: RequestMethod::Post,
            body: RequestBody::Json(serde_json::to_vec(s)?),
            path: path.into(),
        })
    }

    pub(crate) fn empty<S: Into<String>>(path: S) -> Result<RequestBuilder, Error> {
        Ok(RequestBuilder {
            method: RequestMethod::Get,
            body: RequestBody::Empty,
            path: path.into(),
        })
    }

    pub(crate) fn build(self, base_url: &str, token: &str) -> Request {
        Request {
            method: self.method,
            url: format!("{}/bot{}/{}", base_url, token, self.path),
            body: self.body,
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) enum RequestBody {
    Form(Form),
    Json(Vec<u8>),
    Empty,
}
