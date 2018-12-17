use crate::types::chat::{ChannelChat, GroupChat, PrivateChat, SupergroupChat};
use crate::types::user::User;

/// Contains chat-specific data
#[derive(Debug)]
pub enum MessageKind {
    /// Channel chat
    Channel {
        /// Channel chat
        chat: ChannelChat,
        /// Author signature, if exists
        author_signature: Option<String>,
    },
    /// Group chat
    Group {
        /// Group chat
        chat: GroupChat,
        /// Sender
        from: User,
    },
    /// Private chat
    Private {
        /// Private chat
        chat: PrivateChat,
        /// Sender
        from: User,
    },
    /// Supergroup chat
    Supergroup {
        /// Supergroup chat
        chat: SupergroupChat,
        /// Sender
        from: User,
    },
}
