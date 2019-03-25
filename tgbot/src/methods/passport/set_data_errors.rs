use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{Integer, PassportElementError},
};
use failure::Error;
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

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("setPassportDataErrors", &self)
    }
}
