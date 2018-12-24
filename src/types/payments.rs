use crate::types::primitive::Integer;
use crate::types::user::User;
use serde::{Deserialize, Serialize};

/// Basic information about an invoice
#[derive(Clone, Debug, Deserialize)]
pub struct Invoice {
    /// Product name
    pub title: String,
    /// Product description
    pub description: String,
    /// Unique bot deep-linking parameter that can be used to generate this invoice
    pub start_parameter: String,
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double)
    ///
    /// For example, for a price of US$ 1.45 pass amount = 145
    /// See the exp parameter in currencies.json, it shows the number of digits past
    /// the decimal point for each currency (2 for the majority of currencies)
    pub total_amount: Integer,
}

/// Portion of the price for goods or services
#[derive(Clone, Debug, Serialize)]
pub struct LabeledPrice {
    /// Portion label
    pub label: String,
    /// Price of the product in the smallest units of the currency (integer, not float/double)
    ///
    /// For example, for a price of US$ 1.45 pass amount = 145
    /// See the exp parameter in currencies.json, it shows the number of digits past the
    /// decimal point for each currency (2 for the majority of currencies)
    pub amount: Integer,
}

/// Information about an order
#[derive(Clone, Debug, Deserialize)]
pub struct OrderInfo {
    /// User name
    pub name: Option<String>,
    /// User's phone number
    pub phone_number: Option<String>,
    /// User email
    pub email: Option<String>,
    /// User shipping address
    pub shipping_address: Option<ShippingAddress>,
}

/// Information about an incoming pre-checkout query
#[derive(Clone, Debug, Deserialize)]
pub struct PreCheckoutQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double)
    ///
    /// For example, for a price of US$ 1.45 pass amount = 145
    /// See the exp parameter in currencies.json, it shows the number of digits past the
    /// decimal point for each currency (2 for the majority of currencies)
    pub total_amount: Integer,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<String>,
    /// Order info provided by the user
    pub order_info: Option<OrderInfo>,
}

/// Shipping address
#[derive(Clone, Debug, Deserialize)]
pub struct ShippingAddress {
    /// ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// State, if applicable
    pub state: String,
    /// City
    pub city: String,
    /// First line for the address
    pub street_line1: String,
    /// Second line for the address
    pub street_line2: String,
    /// Address post code
    pub post_code: String,
}

/// Shipping option
#[derive(Clone, Debug, Serialize)]
pub struct ShippingOption {
    /// Shipping option identifier
    pub id: String,
    /// Option title
    pub title: String,
    /// List of price portions
    pub prices: Vec<LabeledPrice>,
}

/// Information about an incoming shipping query
#[derive(Clone, Debug, Deserialize)]
pub struct ShippingQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// User specified shipping address
    pub shipping_address: ShippingAddress,
}

/// Basic information about a successful payment
#[derive(Clone, Debug, Deserialize)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double)
    ///
    /// For example, for a price of US$ 1.45 pass amount = 145
    /// See the exp parameter in currencies.json, it shows the number of digits past the
    /// decimal point for each currency (2 for the majority of currencies)
    pub total_amount: Integer,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<String>,
    /// Order info provided by the user
    pub order_info: Option<OrderInfo>,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier
    pub provider_payment_charge_id: String,
}
