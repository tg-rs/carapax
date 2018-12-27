use crate::methods::method::*;
use crate::types::{InlineKeyboardMarkup, Integer, LabeledPrice, Message};
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
    pub fn new<S: Into<String>>(
        chat_id: Integer,
        title: S,
        description: S,
        payload: S,
        provider_token: S,
        start_parameter: S,
        currency: S,
        prices: Vec<LabeledPrice>,
    ) -> Self {
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
    pub fn provider_data<S: Into<String>>(&mut self, provider_data: S) -> &mut Self {
        self.provider_data = Some(provider_data.into());
        self
    }

    /// URL of the product photo for the invoice
    ///
    /// Can be a photo of the goods or a marketing image for a service
    /// People like it better when they see what they are paying for
    pub fn photo_url<S: Into<String>>(&mut self, photo_url: S) -> &mut Self {
        self.photo_url = Some(photo_url.into());
        self
    }

    /// Photo size
    pub fn photo_size(&mut self, photo_size: Integer) -> &mut Self {
        self.photo_size = Some(photo_size);
        self
    }

    /// Photo width
    pub fn photo_width(&mut self, photo_width: Integer) -> &mut Self {
        self.photo_width = Some(photo_width);
        self
    }

    /// Photo height
    pub fn photo_height(&mut self, photo_height: Integer) -> &mut Self {
        self.photo_height = Some(photo_height);
        self
    }

    /// Pass True, if you require the user's full name to complete the order
    pub fn need_name(&mut self, need_name: bool) -> &mut Self {
        self.need_name = Some(need_name);
        self
    }

    /// Pass True, if you require the user's phone number to complete the order
    pub fn need_phone_number(&mut self, need_phone_number: bool) -> &mut Self {
        self.need_phone_number = Some(need_phone_number);
        self
    }

    /// Pass True, if you require the user's email address to complete the order
    pub fn need_email(&mut self, need_email: bool) -> &mut Self {
        self.need_email = Some(need_email);
        self
    }

    /// Pass True, if you require the user's shipping address to complete the order
    pub fn need_shipping_address(&mut self, need_shipping_address: bool) -> &mut Self {
        self.need_shipping_address = Some(need_shipping_address);
        self
    }

    /// Pass True, if user's phone number should be sent to provider
    pub fn send_phone_number_to_provider(
        &mut self,
        send_phone_number_to_provider: bool,
    ) -> &mut Self {
        self.send_phone_number_to_provider = Some(send_phone_number_to_provider);
        self
    }

    /// Pass True, if user's email address should be sent to provider
    pub fn send_email_to_provider(&mut self, send_email_to_provider: bool) -> &mut Self {
        self.send_email_to_provider = Some(send_email_to_provider);
        self
    }

    /// Pass True, if the final price depends on the shipping method
    pub fn flexible(&mut self, is_flexible: bool) -> &mut Self {
        self.is_flexible = Some(is_flexible);
        self
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(&mut self, reply_to_message_id: Integer) -> &mut Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Inline keyboard
    ///
    /// If empty, one 'Pay total price' button will be shown
    /// If not empty, the first button must be a Pay button
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(&mut self, reply_markup: I) -> &mut Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendInvoice {
    type Response = Message;

    fn get_request(&self) -> Result<RequestBuilder, RequestError> {
        RequestBuilder::json("sendInvoice", &self)
    }
}
