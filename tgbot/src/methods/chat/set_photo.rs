use crate::{
    methods::Method,
    request::{Form, Request},
    types::{ChatId, InputFile},
};

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
    pub fn new<C, P>(chat_id: C, photo: P) -> Self
    where
        C: Into<ChatId>,
        P: Into<InputFile>,
    {
        let mut form = Form::new();
        form.insert_field("chat_id", chat_id.into());
        form.insert_field("photo", photo.into());
        SetChatPhoto { form }
    }
}

impl Method for SetChatPhoto {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::form("setChatPhoto", self.form)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};

    #[test]
    fn set_chat_photo() {
        let request = SetChatPhoto::new(1, InputFile::file_id("sticker-id")).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/setChatPhoto");
        if let RequestBody::Form(form) = request.into_body() {
            assert_eq!(form.fields["chat_id"].get_text().unwrap(), "1");
            assert!(form.fields["photo"].get_file().is_some());
        } else {
            panic!("Unexpected request body");
        }
    }
}
