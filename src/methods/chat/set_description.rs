use crate::methods::method::*;
use crate::types::ChatId;
use failure::Error;
use serde::Serialize;

/// Change the description of a supergroup or a channel
///
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights
#[derive(Clone, Debug, Serialize)]
pub struct SetChatDescription {
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl SetChatDescription {
    /// Creates a new SetChatDescription
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        SetChatDescription {
            chat_id: chat_id.into(),
            description: None,
        }
    }

    /// New chat description, 0-255 characters
    pub fn description<S: Into<String>>(&mut self, description: S) -> &mut Self {
        self.description = Some(description.into());
        self
    }
}

impl Method for SetChatDescription {
    type Response = bool;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("setChatDescription", &self)
    }
}
