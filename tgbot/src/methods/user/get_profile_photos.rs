use crate::{
    methods::Method,
    request::Request,
    types::{Integer, UserProfilePhotos},
};
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
    pub fn offset(mut self, offset: Integer) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Limits the number of photos to be retrieved
    ///
    /// Values between 1—100 are accepted
    /// Defaults to 100
    pub fn limit(mut self, limit: Integer) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl Method for GetUserProfilePhotos {
    type Response = UserProfilePhotos;

    fn into_request(self) -> Request {
        Request::json("getUserProfilePhotos", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn get_user_profile_photos() {
        let request = GetUserProfilePhotos::new(1).offset(5).limit(10).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/getUserProfilePhotos"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["user_id"], 1);
            assert_eq!(data["offset"], 5);
            assert_eq!(data["limit"], 10);
        } else {
            panic!("Unexpected request body");
        }
    }
}
