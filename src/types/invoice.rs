/// This object contains basic information about an invoice.
#[derive(Debug)]
pub struct Invoice {
    /// Product name
    pub title: String,
    /// Product description
    pub description: String,
    /// Unique bot deep-linking parameter that can be used to generate this invoice
    pub start_parameter: String,
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double).
    /// For example, for a price of US$ 1.45 pass amount = 145.
    /// See the exp parameter in currencies.json,
    /// it shows the number of digits past
    /// the decimal point for each currency
    /// (2 for the majority of currencies).
    pub total_amount: i64,
}
