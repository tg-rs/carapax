use crate::methods::method::*;
use crate::types::ShippingOption;
use failure::Error;
use serde::Serialize;

/// Reply to shipping query
///
/// If you sent an invoice requesting a shipping address and the parameter is_flexible was specified,
/// the Bot API will send an Update with a shipping_query field to the bot
#[derive(Clone, Debug, Serialize)]
pub struct AnswerShippingQuery {
    shipping_query_id: String,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<Vec<ShippingOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
}

impl AnswerShippingQuery {
    /// Success answer
    ///
    /// # Arguments
    ///
    /// * shipping_query_id - Unique identifier for the query to be answered
    /// * shipping_options - Array of available shipping options
    pub fn ok<S: Into<String>>(
        shipping_query_id: S,
        shipping_options: Vec<ShippingOption>,
    ) -> Self {
        AnswerShippingQuery {
            shipping_query_id: shipping_query_id.into(),
            ok: true,
            shipping_options: Some(shipping_options),
            error_message: None,
        }
    }

    /// Error answer
    ///
    /// # Arguments
    ///
    /// * shipping_query_id - Unique identifier for the query to be answered
    /// * error_message - Error message in human readable form
    pub fn error<S: Into<String>>(shipping_query_id: S, error_message: S) -> Self {
        AnswerShippingQuery {
            shipping_query_id: shipping_query_id.into(),
            ok: false,
            shipping_options: None,
            error_message: Some(error_message.into()),
        }
    }
}

impl Method for AnswerShippingQuery {
    type Response = bool;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("answerShippingQuery", &self)
    }
}
