use crate::methods::method::*;
use crate::types::ChatId;
use failure::Error;
use serde::Serialize;

/// Leave a group, supergroup or channel
#[derive(Clone, Debug, Serialize)]
pub struct LeaveChat {
    chat_id: ChatId,
}

impl LeaveChat {
    /// Creates a new LeaveChat
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    pub fn new<C: Into<ChatId>>(chat_id: C) -> Self {
        LeaveChat {
            chat_id: chat_id.into(),
        }
    }
}

impl Method for LeaveChat {
    type Response = bool;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("leaveChat", &self)
    }
}
