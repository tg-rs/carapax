use crate::request::Request;

/// Represents an API method
pub trait Method {
    /// Type of successful result in API response
    type Response;

    /// Returns information about HTTP request
    fn into_request(self) -> Request;
}
