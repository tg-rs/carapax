use serde::ser::Serialize;
use serde_json::Error as JsonError;

mod form;

pub(crate) use self::form::*;

#[doc(hidden)]
#[derive(Debug)]
pub struct Request {
    path: String,
    method: RequestMethod,
    body: RequestBody,
}

impl Request {
    pub(crate) fn form<P: Into<String>>(path: P, form: Form) -> Self {
        Self {
            path: path.into(),
            method: RequestMethod::Post,
            body: RequestBody::Form(form),
        }
    }

    pub(crate) fn json<P: Into<String>>(path: P, data: impl Serialize) -> Self {
        Self {
            path: path.into(),
            method: RequestMethod::Post,
            body: RequestBody::Json(serde_json::to_string(&data)),
        }
    }

    pub(crate) fn empty<P: Into<String>>(path: P) -> Self {
        Self {
            path: path.into(),
            method: RequestMethod::Get,
            body: RequestBody::Empty,
        }
    }

    pub(crate) fn build_url(&self, base_url: &str, token: &str) -> String {
        format!("{}/bot{}/{}", base_url, token, self.path)
    }

    pub(crate) fn get_method(&self) -> RequestMethod {
        self.method
    }

    pub(crate) fn into_body(self) -> RequestBody {
        self.body
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub(crate) enum RequestMethod {
    Get,
    Post,
}

#[derive(Debug)]
pub(crate) enum RequestBody {
    Form(Form),
    Json(Result<String, JsonError>),
    Empty,
}
