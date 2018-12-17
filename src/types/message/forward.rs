use crate::types::chat::ChannelChat;
use crate::types::primitive::Integer;
use crate::types::user::User;

/// Contains information about original message
#[derive(Debug)]
pub struct Forward {
    /// Sender of the original message
    pub from: ForwardFrom,
    /// Date the original message was sent in Unix time
    pub date: Integer,
}

/// Sender of the original message
#[derive(Debug)]
pub enum ForwardFrom {
    /// Information about user
    User(User),
    /// Information about channel
    Channel {
        /// Information about the original chat
        chat: ChannelChat,
        /// Identifier of the original message in the channel
        message_id: Integer,
        /// Signature of the post author if present
        signature: Option<String>,
    },
}
