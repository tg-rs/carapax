use crate::types::primitive::Float;
use serde::Deserialize;

/// Point on the map
#[derive(Clone, Debug, Deserialize)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: Float,
    /// Latitude as defined by sender
    pub latitude: Float,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]
    use super::*;

    #[test]
    fn deserialize() {
        let data: Location = serde_json::from_value(serde_json::json!({
            "longitude": 2.5,
            "latitude": 2.6
        }))
        .unwrap();
        assert_eq!(data.longitude, 2.5);
        assert_eq!(data.latitude, 2.6);
    }
}
