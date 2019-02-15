use http::{uri::InvalidUri as HttpInvalidUriError, Error as HttpError};
use hyper::Error as HyperError;
use hyper_tls::Error as HyperTlsError;
use std::io::Error as IoError;
use std::net::AddrParseError;
use typed_headers::Error as TypedHeadersError;
use url::ParseError as UrlError;

#[derive(Debug, Fail)]
pub enum ExecutorError {
    #[fail(display = "Failed to parse network address: {}", _0)]
    AddrParse(#[fail(cause)] AddrParseError),
    #[fail(display = "A HTTP error: {}", _0)]
    Http(HttpError),
    #[fail(display = "Failed to parse URI: {}", _0)]
    HttpInvalidUri(#[fail(cause)] HttpInvalidUriError),
    #[fail(display = "Hyper error: {}", _0)]
    Hyper(HyperError),
    #[fail(display = "TLS error occurred: {}", _0)]
    HyperTls(#[fail(cause)] HyperTlsError),
    #[fail(display = "IO error occurred: {}", _0)]
    Io(#[fail(cause)] IoError),
    #[fail(display = "Failed to parse credentials: {}", _0)]
    TypedHeaders(#[fail(cause)] TypedHeadersError),
    #[fail(display = "Got an unexpected proxy string: {}", _0)]
    UnexpectedProxy(String),
    #[fail(display = "Failed to parse url: {}", _0)]
    Url(#[fail(cause)] UrlError),
}

macro_rules! impl_from {
    ($target:ident($from:ty)) => {
        impl From<$from> for ExecutorError {
            fn from(err: $from) -> ExecutorError {
                ExecutorError::$target(err)
            }
        }
    };
}

impl_from!(AddrParse(AddrParseError));
impl_from!(Http(HttpError));
impl_from!(HttpInvalidUri(HttpInvalidUriError));
impl_from!(Hyper(HyperError));
impl_from!(HyperTls(HyperTlsError));
impl_from!(Io(IoError));
impl_from!(TypedHeaders(TypedHeadersError));
impl_from!(Url(UrlError));
