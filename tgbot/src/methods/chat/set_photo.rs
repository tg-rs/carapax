use crate::{
    methods::Method,
    request::{Form, RequestBuilder},
    types::{ChatId, InputFile},
};
use failure::Error;

/// Set a new profile photo for the chat
///
/// Photos can't be changed for private chats
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights
///
/// Note: In regular groups (non-supergroups), this method will only work
/// if the ‘All Members Are Admins’ setting is off in the target group
#[derive(Debug)]
pub struct SetChatPhoto {
    form: Form,
}

impl SetChatPhoto {
    /// Creates a new SetChatPhoto
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * photo - New chat photo, uploaded using multipart/form-data (url and file_id are not supported)
    pub fn new<C: Into<ChatId>>(chat_id: C, photo: InputFile) -> Self {
        let mut form = Form::new();
        form.set_field("chat_id", chat_id.into());
        form.set_field("photo", photo);
        SetChatPhoto { form }
    }
}

impl Method for SetChatPhoto {
    type Response = bool;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::form("setChatPhoto", self.form)
    }
}
