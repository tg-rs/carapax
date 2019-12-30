use crate::{
    methods::Method,
    request::Request,
    types::{Integer, PassportElementError},
};
use serde::Serialize;

/// Informs a user that some of the Telegram Passport elements they provided contains errors
///
/// The user will not be able to re-submit their Passport to you until the errors are fixed
/// (the contents of the field for which you returned the error must change)
///
/// Use this if the data submitted by the user doesn't satisfy the standards
/// your service requires for any reason
///
/// For example, if a birthday date seems invalid, a submitted document is blurry,
/// a scan shows evidence of tampering, etc.
///
/// Supply some details in the error message to make sure the user knows how to correct the issues
#[derive(Clone, Debug, Serialize)]
pub struct SetPassportDataErrors {
    user_id: Integer,
    errors: Vec<PassportElementError>,
}

impl SetPassportDataErrors {
    /// Creates a new SetPassportDataErrors
    ///
    /// # Arguments
    ///
    /// * user_id - User identifier
    /// * errors - Array describing the errors
    pub fn new(user_id: Integer, errors: Vec<PassportElementError>) -> Self {
        SetPassportDataErrors { user_id, errors }
    }
}

impl Method for SetPassportDataErrors {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("setPassportDataErrors", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn set_passport_data_errors() {
        let request = SetPassportDataErrors::new(1, vec![]).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/setPassportDataErrors"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["user_id"], 1);
            assert!(data["errors"].as_array().unwrap().is_empty());
        } else {
            panic!("Unexpected request body");
        }
    }
}
