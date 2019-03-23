use crate::types::location::Location;
use serde::Deserialize;

/// Venue
#[derive(Clone, Debug, Deserialize)]
pub struct Venue {
    /// Venue location
    pub location: Location,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue
    /// For example: “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”
    pub foursquare_type: Option<String>,
}
