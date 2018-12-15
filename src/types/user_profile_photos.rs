use crate::types::photo_size::PhotoSize;
use crate::types::primitive::Integer;

/// This object represent a user's profile pictures.
#[derive(Debug)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: Integer,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<Vec<PhotoSize>>,
}
