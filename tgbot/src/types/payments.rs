use crate::types::{primitive::Integer, user::User};
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
    label: String,
    amount: Integer,
}

impl LabeledPrice {
    /// Creates a new LabeledPrice
    ///
    /// # Arguments
    ///
    /// * label - Portion label
    /// * amount - For example, for a price of US$ 1.45 pass amount = 145
    ///            See the exp parameter in currencies.json, it shows the number of digits past the
    ///            decimal point for each currency (2 for the majority of currencies)
    pub fn new<S: Into<String>>(label: S, amount: Integer) -> Self {
        Self {
            label: label.into(),
            amount,
        }
    }

    /// Returns a portion label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns an amount
    pub fn amount(&self) -> Integer {
        self.amount
    }
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
    id: String,
    title: String,
    prices: Vec<LabeledPrice>,
}

impl ShippingOption {
    /// Creates a new ShippingOption
    ///
    /// # Arguments
    ///
    /// * id - Shipping option identifier
    /// * title - Option title
    /// * prices - List of price portions
    pub fn new<I, T>(id: I, title: T, prices: Vec<LabeledPrice>) -> Self
    where
        I: Into<String>,
        T: Into<String>,
    {
        Self {
            id: id.into(),
            title: title.into(),
            prices,
        }
    }

    /// Returns an option id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns an option title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns price portions
    pub fn prices(&self) -> &[LabeledPrice] {
        &self.prices
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_invoice() {
        let data: Invoice = serde_json::from_value(serde_json::json!({
            "title": "invoice title",
            "description": "invoice description",
            "start_parameter": "invoice start parameter",
            "currency": "RUB",
            "total_amount": 100
        }))
        .unwrap();
        assert_eq!(data.title, "invoice title");
        assert_eq!(data.description, "invoice description");
        assert_eq!(data.start_parameter, "invoice start parameter");
        assert_eq!(data.currency, "RUB");
        assert_eq!(data.total_amount, 100);
    }

    #[test]
    fn serialize_labeled_price() {
        let price = LabeledPrice::new("price label", 145);
        let data = serde_json::to_string(&price).unwrap();
        let new_price: serde_json::Value = serde_json::from_str(&data).unwrap();
        assert_eq!(price.label(), new_price.get("label").unwrap().as_str().unwrap());
        assert_eq!(price.amount(), new_price.get("amount").unwrap().as_i64().unwrap());
    }

    #[test]
    fn deserialize_order_info_full() {
        let data: OrderInfo = serde_json::from_value(serde_json::json!({
            "name": "magticom",
            "phone_number": "+995 32 217 00 00",
            "email": "office@magticom.ge",
            "shipping_address": {
                "country_code": "GE",
                "state": "",
                "city": "Tbilisi",
                "street_line1": "7 A. Politkovskaya st.",
                "street_line2": "",
                "post_code": ""
            }
        }))
        .unwrap();
        assert_eq!(data.name.unwrap(), "magticom");
        assert_eq!(data.phone_number.unwrap(), "+995 32 217 00 00");
        assert_eq!(data.email.unwrap(), "office@magticom.ge");
        let addr = data.shipping_address.unwrap();
        assert_eq!(addr.country_code, "GE");
        assert_eq!(addr.state, "");
        assert_eq!(addr.city, "Tbilisi");
        assert_eq!(addr.street_line1, "7 A. Politkovskaya st.");
        assert_eq!(addr.street_line2, "");
        assert_eq!(addr.post_code, "");
    }

    #[test]
    fn deserialize_order_info_partial() {
        let data: OrderInfo = serde_json::from_value(serde_json::json!({})).unwrap();
        assert!(data.name.is_none());
        assert!(data.phone_number.is_none());
        assert!(data.email.is_none());
        assert!(data.shipping_address.is_none());
    }

    #[test]
    fn deserialize_pre_checkout_query_full() {
        let data: PreCheckoutQuery = serde_json::from_value(serde_json::json!({
            "id": "query id",
            "from": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            },
            "currency": "GEL",
            "total_amount": 100,
            "invoice_payload": "invoice payload",
            "shipping_option_id": "option id",
            "order_info": {}
        }))
        .unwrap();
        assert_eq!(data.id, "query id");
        assert_eq!(data.from.id, 1);
        assert_eq!(data.currency, "GEL");
        assert_eq!(data.total_amount, 100);
        assert_eq!(data.invoice_payload, "invoice payload");
        assert_eq!(data.shipping_option_id.unwrap(), "option id");
        assert!(data.order_info.unwrap().name.is_none());
    }

    #[test]
    fn deserialize_pre_checkout_query_partial() {
        let data: PreCheckoutQuery = serde_json::from_value(serde_json::json!({
            "id": "query id",
            "from": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            },
            "currency": "GEL",
            "total_amount": 100,
            "invoice_payload": "invoice payload"
        }))
        .unwrap();
        assert_eq!(data.id, "query id");
        assert_eq!(data.from.id, 1);
        assert_eq!(data.currency, "GEL");
        assert_eq!(data.total_amount, 100);
        assert_eq!(data.invoice_payload, "invoice payload");
        assert!(data.shipping_option_id.is_none());
        assert!(data.order_info.is_none());
    }

    #[test]
    fn deserialize_shipping_address() {
        let data: ShippingAddress = serde_json::from_value(serde_json::json!({
            "country_code": "RU",
            "state": "Chechen Republic",
            "city": "Gudermes",
            "street_line1": "Nuradilov st., 12",
            "street_line2": "",
            "post_code": "366200"
        }))
        .unwrap();
        assert_eq!(data.country_code, "RU");
        assert_eq!(data.state, "Chechen Republic");
        assert_eq!(data.city, "Gudermes");
        assert_eq!(data.street_line1, "Nuradilov st., 12");
        assert_eq!(data.street_line2, "");
        assert_eq!(data.post_code, "366200");
    }

    #[test]
    fn serialize_shipping_option() {
        let option = ShippingOption::new("id", "title", vec![]);
        let data = serde_json::to_string(&option).unwrap();
        let new_option: serde_json::Value = serde_json::from_str(&data).unwrap();
        assert_eq!(new_option.get("id").unwrap().as_str().unwrap(), option.id());
        assert_eq!(new_option.get("title").unwrap().as_str().unwrap(), option.title());
        assert!(new_option.get("prices").unwrap().as_array().unwrap().is_empty());
        assert!(option.prices().is_empty())
    }

    #[test]
    fn deserialize_shipping_query() {
        let data: ShippingQuery = serde_json::from_value(serde_json::json!({
            "id": "query-id",
            "from": {
                "id": 1,
                "first_name": "test",
                "is_bot": false
            },
            "invoice_payload": "payload",
            "shipping_address": {
                "country_code": "RU",
                "state": "Chechen Republic",
                "city": "Gudermes",
                "street_line1": "Nuradilov st., 12",
                "street_line2": "",
                "post_code": "366200"
            }
        }))
        .unwrap();
        assert_eq!(data.id, "query-id");
        assert_eq!(data.from.id, 1);
        assert_eq!(data.invoice_payload, "payload");
        assert_eq!(data.shipping_address.country_code, "RU");
    }

    #[test]
    fn deserialize_successful_payment_full() {
        let data: SuccessfulPayment = serde_json::from_value(serde_json::json!({
            "currency": "RUB",
            "total_amount": 145,
            "invoice_payload": "invoice payload",
            "shipping_option_id": "option id",
            "order_info": {},
            "telegram_payment_charge_id": "tg-charge-id",
            "provider_payment_charge_id": "provider-charge-id"
        }))
        .unwrap();
        assert_eq!(data.currency, "RUB");
        assert_eq!(data.total_amount, 145);
        assert_eq!(data.invoice_payload, "invoice payload");
        assert_eq!(data.shipping_option_id.unwrap(), "option id");
        assert!(data.order_info.unwrap().name.is_none());
        assert_eq!(data.telegram_payment_charge_id, "tg-charge-id");
        assert_eq!(data.provider_payment_charge_id, "provider-charge-id");
    }

    #[test]
    fn deserialize_successful_payment_partial() {
        let data: SuccessfulPayment = serde_json::from_value(serde_json::json!({
            "currency": "RUB",
            "total_amount": 145,
            "invoice_payload": "invoice payload",
            "telegram_payment_charge_id": "tg-charge-id",
            "provider_payment_charge_id": "provider-charge-id"
        }))
        .unwrap();
        assert_eq!(data.currency, "RUB");
        assert_eq!(data.total_amount, 145);
        assert_eq!(data.invoice_payload, "invoice payload");
        assert!(data.shipping_option_id.is_none());
        assert!(data.order_info.is_none());
        assert_eq!(data.telegram_payment_charge_id, "tg-charge-id");
        assert_eq!(data.provider_payment_charge_id, "provider-charge-id");
    }
}
