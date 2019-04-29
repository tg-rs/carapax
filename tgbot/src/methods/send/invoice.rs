use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{InlineKeyboardMarkup, Integer, LabeledPrice, Message},
};
use failure::Error;
use serde::Serialize;

/// Send invoice
#[derive(Clone, Debug, Serialize)]
pub struct SendInvoice {
    chat_id: Integer,
    title: String,
    description: String,
    payload: String,
    provider_token: String,
    start_parameter: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_size: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_flexible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl SendInvoice {
    /// Creates a new SendInvoice with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target private chat
    /// * title - Product name, 1-32 characters
    /// * description - Product description, 1-255 characters
    /// * payload - Bot-defined invoice payload, 1-128 bytes
    ///             This will not be displayed to the user, use for your internal processes
    /// * provider_token - Payments provider token, obtained via Botfather
    /// * start_parameter - Unique deep-linking parameter that can be used
    ///                     to generate this invoice when used as a start parameter
    /// * currency - Three-letter ISO 4217 currency code, see more on currencies
    /// * prices - Price breakdown, a list of components
    ///            (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    #[allow(clippy::too_many_arguments)]
    pub fn new<A, B, C, D, E, F>(
        chat_id: Integer,
        title: A,
        description: B,
        payload: C,
        provider_token: D,
        start_parameter: E,
        currency: F,
        prices: Vec<LabeledPrice>,
    ) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
        D: Into<String>,
        E: Into<String>,
        F: Into<String>,
    {
        SendInvoice {
            chat_id,
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            provider_token: provider_token.into(),
            start_parameter: start_parameter.into(),
            currency: currency.into(),
            prices,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// JSON-encoded data about the invoice, which will be shared with the payment provider
    ///
    /// A detailed description of required fields should be provided by the payment provider
    pub fn provider_data<S: Into<String>>(mut self, provider_data: S) -> Self {
        self.provider_data = Some(provider_data.into());
        self
    }

    /// URL of the product photo for the invoice
    ///
    /// Can be a photo of the goods or a marketing image for a service
    /// People like it better when they see what they are paying for
    pub fn photo_url<S: Into<String>>(mut self, photo_url: S) -> Self {
        self.photo_url = Some(photo_url.into());
        self
    }

    /// Photo size
    pub fn photo_size(mut self, photo_size: Integer) -> Self {
        self.photo_size = Some(photo_size);
        self
    }

    /// Photo width
    pub fn photo_width(mut self, photo_width: Integer) -> Self {
        self.photo_width = Some(photo_width);
        self
    }

    /// Photo height
    pub fn photo_height(mut self, photo_height: Integer) -> Self {
        self.photo_height = Some(photo_height);
        self
    }

    /// Pass True, if you require the user's full name to complete the order
    pub fn need_name(mut self, need_name: bool) -> Self {
        self.need_name = Some(need_name);
        self
    }

    /// Pass True, if you require the user's phone number to complete the order
    pub fn need_phone_number(mut self, need_phone_number: bool) -> Self {
        self.need_phone_number = Some(need_phone_number);
        self
    }

    /// Pass True, if you require the user's email address to complete the order
    pub fn need_email(mut self, need_email: bool) -> Self {
        self.need_email = Some(need_email);
        self
    }

    /// Pass True, if you require the user's shipping address to complete the order
    pub fn need_shipping_address(mut self, need_shipping_address: bool) -> Self {
        self.need_shipping_address = Some(need_shipping_address);
        self
    }

    /// Pass True, if user's phone number should be sent to provider
    pub fn send_phone_number_to_provider(mut self, send_phone_number_to_provider: bool) -> Self {
        self.send_phone_number_to_provider = Some(send_phone_number_to_provider);
        self
    }

    /// Pass True, if user's email address should be sent to provider
    pub fn send_email_to_provider(mut self, send_email_to_provider: bool) -> Self {
        self.send_email_to_provider = Some(send_email_to_provider);
        self
    }

    /// Pass True, if the final price depends on the shipping method
    pub fn flexible(mut self, is_flexible: bool) -> Self {
        self.is_flexible = Some(is_flexible);
        self
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Inline keyboard
    ///
    /// If empty, one 'Pay total price' button will be shown
    /// If not empty, the first button must be a Pay button
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendInvoice {
    type Response = Message;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("sendInvoice", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::InlineKeyboardButton,
    };
    use serde_json::Value;

    #[test]
    fn send_invoice() {
        let request = SendInvoice::new(1, "title", "description", "payload", "token", "param", "RUB", vec![])
            .provider_data("data")
            .photo_url("url")
            .photo_size(100)
            .photo_width(200)
            .photo_height(300)
            .need_name(true)
            .need_phone_number(true)
            .need_email(true)
            .need_shipping_address(true)
            .send_phone_number_to_provider(true)
            .send_email_to_provider(true)
            .flexible(true)
            .disable_notification(true)
            .reply_to_message_id(1)
            .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/sendInvoice");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(
                data,
                serde_json::json!({
                    "chat_id": 1,
                    "title": "title",
                    "description": "description",
                    "payload": "payload",
                    "provider_token": "token",
                    "start_parameter": "param",
                    "currency": "RUB",
                    "prices": [],
                    "provider_data": "data",
                    "photo_url": "url",
                    "photo_size": 100,
                    "photo_width": 200,
                    "photo_height": 300,
                    "need_name": true,
                    "need_phone_number": true,
                    "need_email": true,
                    "need_shipping_address": true,
                    "send_phone_number_to_provider": true,
                    "send_email_to_provider": true,
                    "is_flexible": true,
                    "disable_notification": true,
                    "reply_to_message_id": 1,
                    "reply_markup": {
                        "inline_keyboard": [[
                            {"text": "text", "url": "url"}
                        ]]
                    }
                })
            );
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
