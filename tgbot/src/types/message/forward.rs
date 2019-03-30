use crate::types::{chat::ChannelChat, primitive::Integer, user::User};

/// Contains information about original message
#[derive(Clone, Debug)]
pub struct Forward {
    /// Sender of the original message
    pub from: ForwardFrom,
    /// Date the original message was sent in Unix time
    pub date: Integer,
}

/// Sender of the original message
#[derive(Clone, Debug)]
pub enum ForwardFrom {
    /// Information about user
    User(User),
    /// Information about channel
    Channel {
        /// Information about the original chat
        chat: ChannelChat,
        /// Identifier of the original message in the channel
        message_id: Option<Integer>,
        /// Signature of the post author if present
        signature: Option<String>,
    },
}
