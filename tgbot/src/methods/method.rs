use crate::request::RequestBuilder;
use failure::Error;

/// Represents an API method
pub trait Method {
    /// Type of successful result in API response
    type Response;

    /// Returns information about HTTP request
    fn get_request(&self) -> Result<RequestBuilder, Error>;
}
