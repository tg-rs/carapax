use crate::{
    methods::Method,
    request::{Form, RequestBuilder},
    types::{ChatId, Integer, MediaGroup, Message},
};
use failure::Error;

/// Send a group of photos or videos as an album
#[derive(Debug)]
pub struct SendMediaGroup {
    form: Form,
}

impl SendMediaGroup {
    /// Creates a new SendMediaGroup with empty optional parameters
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * media - Photos and videos to be sent, must include 2–10 items
    pub fn new<C: Into<ChatId>>(chat_id: C, media: MediaGroup) -> Result<Self, Error> {
        let mut form = Form::new();
        form.set_field("chat_id", chat_id.into());
        for (k, v) in media.into_form()? {
            form.set_field(k, v);
        }
        Ok(SendMediaGroup { form })
    }

    /// Sends the messages silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.form.set_field("disable_notification", value);
        self
    }

    /// If the messages are a reply, ID of the original message
    pub fn reply_to_message_id(mut self, value: Integer) -> Self {
        self.form.set_field("reply_to_message_id", value);
        self
    }
}

impl Method for SendMediaGroup {
    type Response = Vec<Message>;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::form("sendMediaGroup", self.form)
    }
}
