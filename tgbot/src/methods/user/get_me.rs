use crate::{methods::Method, request::RequestBuilder, types::User};
use failure::Error;

/// Returns basic information about the bot in form of a User object
#[derive(Clone, Copy, Debug)]
pub struct GetMe;

impl Method for GetMe {
    type Response = User;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::empty("getMe")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};

    #[test]
    fn get_me() {
        let request = GetMe.into_request().unwrap().build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Get);
        assert_eq!(request.url, "base-url/bottoken/getMe");
        if let RequestBody::Empty = request.body {
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
