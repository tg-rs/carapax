use crate::methods::method::*;
use crate::types::{Integer, UserProfilePhotos};
use serde::Serialize;

/// Get a list of profile pictures for a user
#[derive(Clone, Debug, Serialize)]
pub struct GetUserProfilePhotos {
    user_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Integer>,
}

impl GetUserProfilePhotos {
    /// Creates a new GetUserProfilePhotos with empty optional parameters
    ///
    /// # Arguments
    ///
    /// user_id - Unique identifier of the target user
    pub fn new(user_id: Integer) -> Self {
        GetUserProfilePhotos {
            user_id,
            offset: None,
            limit: None,
        }
    }

    /// Sequential number of the first photo to be returned
    ///
    /// By default, all photos are returned
    pub fn offset(&mut self, offset: Integer) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    /// Limits the number of photos to be retrieved
    ///
    /// Values between 1—100 are accepted
    /// Defaults to 100
    pub fn limit(&mut self, limit: Integer) -> &mut Self {
        self.limit = Some(limit);
        self
    }
}

impl Method for GetUserProfilePhotos {
    type Response = UserProfilePhotos;

    fn get_request(&self) -> Result<RequestBuilder, RequestError> {
        RequestBuilder::json("getUserProfilePhotos", &self)
    }
}
