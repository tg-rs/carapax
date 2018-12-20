use crate::types::primitive::Float;

/// Point on the map
#[derive(Clone, Debug, Deserialize)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: Float,
    /// Latitude as defined by sender
    pub latitude: Float,
}
