use crate::{methods::method::*, types::ChatId};
use failure::Error;
use serde::Serialize;

/// Set a new profile photo for the chat
///
/// Photos can't be changed for private chats
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights
///
/// Note: In regular groups (non-supergroups), this method will only work
/// if the ‘All Members Are Admins’ setting is off in the target group
#[derive(Clone, Debug, Serialize)]
pub struct SetChatPhoto {
    chat_id: ChatId,
    photo: String,
}

impl SetChatPhoto {
    /// Creates a new SetChatPhoto
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * photo - New chat photo, uploaded using multipart/form-data
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, photo: S) -> Self {
        SetChatPhoto {
            chat_id: chat_id.into(),
            photo: photo.into(),
        }
    }
}

impl Method for SetChatPhoto {
    type Response = bool;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("setChatPhoto", &self)
    }
}
