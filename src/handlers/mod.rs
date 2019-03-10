use crate::types::Update;
use std::net::SocketAddr;

mod poll;
mod webhook;

pub use self::poll::UpdatesStream;

pub(crate) use self::webhook::run_server;

/// A webhook update handler
pub trait UpdateHandler {
    /// Handles an update
    fn handle(&mut self, update: Update);
}

/// Defines how to get updates from telegram
pub struct UpdateMethod {
    pub(crate) kind: UpdateMethodKind,
}

impl UpdateMethod {
    /// Get updates using long polling
    pub fn poll() -> Self {
        Self {
            kind: UpdateMethodKind::Poll,
        }
    }

    /// Get updates using webhook
    ///
    /// # Arguments
    ///
    /// - addr - Bind address
    /// - path - URL path for webhook
    pub fn webhook<A, S>(addr: A, path: S) -> Self
    where
        A: Into<SocketAddr>,
        S: Into<String>,
    {
        Self {
            kind: UpdateMethodKind::Webhook {
                addr: addr.into(),
                path: path.into(),
            },
        }
    }
}

pub(crate) enum UpdateMethodKind {
    Poll,
    Webhook { addr: SocketAddr, path: String },
}
