use crate::{methods::Method, request::RequestBuilder};
use failure::Error;
use serde::Serialize;

/// Respond to pre-checkout query
///
/// Once the user has confirmed their payment and shipping details,
/// the Bot API sends the final confirmation in the form of an Update with the field pre_checkout_query
/// Note: The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent
#[derive(Clone, Debug, Serialize)]
pub struct AnswerPreCheckoutQuery {
    pre_checkout_query_id: String,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
}

impl AnswerPreCheckoutQuery {
    /// Success answer
    ///
    /// # Arguments
    ///
    /// * pre_checkout_query_id - Unique identifier for the query to be answered
    pub fn ok<S: Into<String>>(pre_checkout_query_id: S) -> Self {
        AnswerPreCheckoutQuery {
            pre_checkout_query_id: pre_checkout_query_id.into(),
            ok: true,
            error_message: None,
        }
    }

    /// Error answer
    ///
    /// # Arguments
    ///
    /// * pre_checkout_query_id - Unique identifier for the query to be answered
    /// * error_message - Error message in human readable form
    ///                   that explains the reason for failure to proceed with the checkout
    pub fn error<S: Into<String>>(pre_checkout_query_id: S, error_message: S) -> Self {
        AnswerPreCheckoutQuery {
            pre_checkout_query_id: pre_checkout_query_id.into(),
            ok: false,
            error_message: Some(error_message.into()),
        }
    }
}

impl Method for AnswerPreCheckoutQuery {
    type Response = bool;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("answerPreCheckoutQuery", &self)
    }
}
