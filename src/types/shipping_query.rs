use crate::types::shipping_address::ShippingAddress;
use crate::types::user::User;

/// This object contains information about an incoming shipping query.
#[derive(Debug)]
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
