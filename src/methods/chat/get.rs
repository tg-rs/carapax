use crate::methods::method::*;
use crate::types::{Chat, ChatId};
use failure::Error;
use serde::Serialize;

/// Get up to date information about the chat
#[derive(Clone, Debug, Serialize)]
pub struct GetChat {
    chat_id: ChatId,
}

impl GetChat {
    /// Creates a new GetChat
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        GetChat {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for GetChat {
    type Response = Chat;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("getChat", &self)
    }
}
