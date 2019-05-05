use crate::types::Update;
use futures::{future::Either, Future, Stream};
use hyper::Server;
use std::net::SocketAddr;

mod poll;
mod webhook;

pub use self::{poll::*, webhook::*};

/// An update handler
///
/// When update is received from telegram it will come here
pub trait UpdateHandler {
    /// Handles an update
    ///
    /// # Arguments
    ///
    /// * update - A received update
    fn handle(&mut self, update: Update);
}

/// Defines how to get updates from Telegram
pub struct UpdateMethod {
    kind: UpdateMethodKind,
}

impl UpdateMethod {
    /// Get updates using long polling
    pub fn poll<S: Into<UpdatesStream>>(stream: S) -> Self {
        Self {
            kind: UpdateMethodKind::Poll(stream.into()),
        }
    }

    /// Get updates using webhook
    ///
    /// # Arguments
    ///
    /// * addr - Bind address
    /// * path - URL path for webhook
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

enum UpdateMethodKind {
    Poll(UpdatesStream),
    Webhook { addr: SocketAddr, path: String },
}

/// Allows to get updates from Telegram
///
/// # Arguments
///
/// * update_method - How to receive updates: via webhook or long polling
/// * handler - Updates handler (see [UpdateHandler](trait.UpdateHandler.html) trait)
pub fn handle_updates<H>(update_method: UpdateMethod, mut handler: H) -> impl Future<Item = (), Error = ()>
where
    H: UpdateHandler + Send + Sync + 'static,
{
    match update_method.kind {
        UpdateMethodKind::Poll(stream) => Either::A(
            stream
                .for_each(move |update| {
                    handler.handle(update);
                    Ok(())
                })
                .then(|_| Ok(())),
        ),
        UpdateMethodKind::Webhook { addr, path } => Either::B(
            Server::bind(&addr)
                .serve(WebhookServiceFactory::new(path, handler))
                .map_err(|e| log::error!("Server error: {}", e)),
        ),
    }
}
