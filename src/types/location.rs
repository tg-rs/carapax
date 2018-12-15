use crate::types::primitive::Float;

/// This object represents a point on the map.
#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: Float,
    /// Latitude as defined by sender
    pub latitude: Float,
}
