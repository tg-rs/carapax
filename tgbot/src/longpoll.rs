use crate::{
    api::{Api, ExecuteError},
    handler::UpdateHandler,
    methods::GetUpdates,
    types::{AllowedUpdate, Integer, ResponseError, ResponseParameters},
};
use async_stream::stream;
use futures_util::{pin_mut, stream::StreamExt};
use log::error;
use std::{cmp::max, collections::HashSet, time::Duration};
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    time::delay_for,
};

const DEFAULT_LIMIT: Integer = 100;
const DEFAULT_POLL_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_ERROR_TIMEOUT: Duration = Duration::from_secs(5);

/// Receive incoming updates using long polling
pub struct LongPoll<H> {
    api: Api,
    handler: Box<H>,
    options: LongPollOptions,
    sender: Sender<()>,
    receiver: Receiver<()>,
}

impl<H> LongPoll<H> {
    /// Creates a new LongPoll
    ///
    /// # Arguments
    ///
    /// * api - Telegram Bot API Client
    /// * handler - Updates Handler
    pub fn new(api: Api, handler: H) -> Self {
        let (sender, receiver) = channel(1);
        Self {
            api,
            handler: Box::new(handler),
            options: LongPollOptions::default(),
            sender,
            receiver,
        }
    }

    /// Set poll options
    pub fn options(mut self, options: LongPollOptions) -> Self {
        self.options = options;
        self
    }
}

impl<H> LongPoll<H>
where
    H: UpdateHandler,
{
    /// Returns a long poll handle
    pub fn get_handle(&self) -> LongPollHandle {
        LongPollHandle {
            sender: self.sender.clone(),
        }
    }

    /// Start polling loop
    pub async fn run(mut self) {
        let LongPollOptions {
            mut offset,
            limit,
            poll_timeout,
            error_timeout,
            allowed_updates,
        } = self.options;
        let api = self.api.clone();
        let mut receiver = self.receiver;
        let s = stream! {
            loop {
                if receiver.try_recv().is_ok() {
                    receiver.close();
                    break;
                }
                let method = GetUpdates::default()
                    .offset(offset + 1)
                    .limit(limit)
                    .timeout(poll_timeout)
                    .allowed_updates(allowed_updates.clone());
                let updates = match api.execute(method).await {
                    Ok(updates) => updates,
                    Err(err) => {
                        error!("An error has occurred while getting updates: {}", err);
                        let timeout = get_error_timeout(err, error_timeout);
                        delay_for(error_timeout).await;
                        continue
                    }
                };
                for update in updates {
                    offset = max(offset, update.id);
                    yield update
                }
            }
        };
        pin_mut!(s);
        while let Some(update) = s.next().await {
            if let Err(err) = self.handler.handle(update).await {
                error!("Failed to handle update: {:?}", err);
            }
        }
    }
}

/// Long poll handle
pub struct LongPollHandle {
    sender: Sender<()>,
}

impl LongPollHandle {
    /// Stop polling loop
    pub async fn shutdown(mut self) {
        let _ = self.sender.send(()).await;
    }
}

fn get_error_timeout(err: ExecuteError, default_timeout: Duration) -> Duration {
    if let ExecuteError::Response(ResponseError {
        parameters: Some(ResponseParameters {
            retry_after: Some(retry_after),
            ..
        }),
        ..
    }) = err
    {
        Duration::from_secs(retry_after as u64)
    } else {
        default_timeout
    }
}

/// Options for long polling
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LongPollOptions {
    offset: Integer,
    limit: Integer,
    poll_timeout: Duration,
    error_timeout: Duration,
    allowed_updates: HashSet<AllowedUpdate>,
}

impl LongPollOptions {
    /// Limits the number of updates to be retrieved
    ///
    /// Values between 1—100 are accepted
    ///
    /// Defaults to 100
    pub fn limit(mut self, limit: Integer) -> Self {
        self.limit = limit;
        self
    }

    /// Timeout for long polling
    ///
    /// 0 - usual short polling
    ///
    /// Defaults to 10
    ///
    /// Should be positive, short polling should be used for testing purposes only
    pub fn poll_timeout(mut self, poll_timeout: Duration) -> Self {
        self.poll_timeout = poll_timeout;
        self
    }

    /// Timeout in seconds when an error has occurred
    ///
    /// Defaults to 5
    pub fn error_timeout(mut self, error_timeout: u64) -> Self {
        self.error_timeout = Duration::from_secs(error_timeout);
        self
    }

    /// Adds a type of updates you want your bot to receive
    pub fn allowed_update(mut self, allowed_update: AllowedUpdate) -> Self {
        self.allowed_updates.insert(allowed_update);
        self
    }
}

impl Default for LongPollOptions {
    fn default() -> Self {
        LongPollOptions {
            offset: 0,
            limit: DEFAULT_LIMIT,
            poll_timeout: DEFAULT_POLL_TIMEOUT,
            error_timeout: DEFAULT_ERROR_TIMEOUT,
            allowed_updates: HashSet::new(),
        }
    }
}
