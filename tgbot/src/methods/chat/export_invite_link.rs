use crate::{methods::Method, request::RequestBuilder, types::ChatId};
use failure::Error;
use serde::Serialize;

/// Generate a new invite link for a chat
///
/// Any previously generated link is revoked
/// The bot must be an administrator in the chat for this to work and must have the appropriate admin rights
/// Returns the new invite link as String on success
#[derive(Clone, Debug, Serialize)]
pub struct ExportChatInviteLink {
    chat_id: ChatId,
}

impl ExportChatInviteLink {
    /// Creates a new ExportChatInviteLink
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        ExportChatInviteLink {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for ExportChatInviteLink {
    type Response = String;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("exportChatInviteLink", &self)
    }
}
