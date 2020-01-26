use serde::Serialize;

use crate::{requests::Request, responses::UserProfilePhotos};

/// Use this struct to send text messages. On success, the sent `Message` is returned.
#[derive(Serialize, Debug, Clone)]
pub struct GetUserProfilePhotos {
    /// Unique identifier of the target user
    pub user_id: i64,

    /// Sequential number of the first photo to be returned. By default, all photos are returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Limits the number of photos to be retrieved. Values between 1—100 are accepted. Defaults to 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl Request for GetUserProfilePhotos {
    type ResponseType = UserProfilePhotos;

    fn method(&self) -> &'static str {
        "getUserProfilePhotos"
    }
}

impl GetUserProfilePhotos {
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            offset: None,
            limit: None,
        }
    }
}
