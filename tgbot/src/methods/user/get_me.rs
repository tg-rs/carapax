use crate::{methods::Method, request::Request, types::User};

/// Returns basic information about the bot in form of a User object
#[derive(Clone, Copy, Debug)]
pub struct GetMe;

impl Method for GetMe {
    type Response = User;

    fn into_request(self) -> Request {
        Request::empty("getMe")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};

    #[test]
    fn get_me() {
        let request = GetMe.into_request();
        assert_eq!(request.get_method(), RequestMethod::Get);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/getMe");
        if let RequestBody::Empty = request.into_body() {
        } else {
            panic!("Unexpected request body");
        }
    }
}
