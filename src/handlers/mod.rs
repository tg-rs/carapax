mod poll;
mod webhook;

pub use self::poll::UpdatesStream;

pub(crate) use self::webhook::run_server;

use crate::types::Update;
use std::net::SocketAddr;

/// A webhook update handler
pub trait UpdateHandler {
    /// Handles an update
    fn handle(&mut self, update: Update);
}

/// An update method
pub enum UpdateMethod {
    /// Get updates using "getUpdates" method
    Polling,
    /// Get updates using webhook
    Webhook {
        /// Bind address
        addr: SocketAddr,
        /// URL path for webhook
        path: String,
    },
}

impl UpdateMethod {
    /// An easier way to create [`UpdateMethod::Webhook`]
    ///
    /// [`UpdateMethod::Webhook`]: enum.UpdateMethod.html#variant.Webhook
    pub fn webhook<A, S>(addr: A, path: S) -> UpdateMethod
    where
        A: Into<SocketAddr>,
        S: Into<String>,
    {
        UpdateMethod::Webhook {
            addr: addr.into(),
            path: path.into(),
        }
    }
}
