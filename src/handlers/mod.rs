mod poll;
mod webhook;

pub use self::{poll::UpdatesStream, webhook::run_server};

use crate::{types::Update, Api};
use std::net::SocketAddr;

/// A webhook update handler
pub trait UpdateHandler {
    /// Handles an update
    fn handle(&mut self, api: &Api, update: Update);
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
