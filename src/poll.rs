use crate::api::Api;
use crate::methods::GetUpdates;
use crate::types::{AllowedUpdate, Integer, Update};
use failure::Error;
use futures::{Async, Future, Poll, Stream};
use log::error;
use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use std::time::Duration;
use tokio_timer::sleep;

const DEFAULT_LIMIT: Integer = 100;
const DEFAULT_POLL_TIMEOUT: Integer = 10;
const DEFAULT_ERROR_TIMEOUT: u64 = 5;

/// Updates stream used for long polling
pub struct UpdatesStream {
    api: Api,
    offset: Integer,
    limit: Integer,
    poll_timeout: Integer,
    error_timeout: Duration,
    allowed_updates: HashSet<AllowedUpdate>,
    items: VecDeque<Update>,
    request: Option<Box<Future<Item = Option<Vec<Update>>, Error = Error>>>,
}

impl UpdatesStream {
    /// Creates a new updates stream
    pub fn new(api: Api) -> Self {
        UpdatesStream {
            api,
            offset: 0,
            limit: DEFAULT_LIMIT,
            poll_timeout: DEFAULT_POLL_TIMEOUT,
            error_timeout: Duration::from_secs(DEFAULT_ERROR_TIMEOUT),
            allowed_updates: HashSet::new(),
            items: VecDeque::new(),
            request: None,
        }
    }

    /// Limits the number of updates to be retrieved
    ///
    /// Values between 1—100 are accepted
    /// Defaults to 100
    pub fn limit(&mut self, limit: Integer) -> &mut Self {
        self.limit = limit;
        self
    }

    /// Timeout in seconds for long polling
    ///
    /// 0 - usual short polling
    /// Defaults to 10
    /// Should be positive, short polling should be used for testing purposes only
    pub fn poll_timeout(&mut self, poll_timeout: Integer) -> &mut Self {
        self.poll_timeout = poll_timeout;
        self
    }

    /// Timeout in seconds when an error has occurred
    ///
    /// Defaults to 5
    pub fn error_timeout(&mut self, error_timeout: u64) -> &mut Self {
        self.error_timeout = Duration::from_secs(error_timeout);
        self
    }

    /// Adds a type of updates you want your bot to receive
    pub fn allowed_update(&mut self, allowed_update: AllowedUpdate) -> &mut Self {
        self.allowed_updates.insert(allowed_update);
        self
    }
}

impl Stream for UpdatesStream {
    type Item = Update;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(update) = self.items.pop_front() {
            return Ok(Async::Ready(Some(update)));
        }

        let should_request = match self.request {
            Some(ref mut request) => match request.poll() {
                Ok(Async::Ready(Some(items))) => {
                    for i in items {
                        self.offset = max(self.offset, i.id);
                        self.items.push_back(i);
                    }
                    Ok(())
                }
                Ok(Async::Ready(None)) => Ok(()),
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(err) => Err(err),
            },
            None => Ok(()),
        };

        match should_request {
            Ok(()) => {
                self.request = Some(Box::new(
                    self.api
                        .execute(
                            GetUpdates::default()
                                .offset(self.offset + 1)
                                .limit(self.limit)
                                .timeout(self.poll_timeout)
                                .allowed_updates(self.allowed_updates.clone()),
                        )
                        .map(Some),
                ));
            }
            Err(err) => {
                error!("An error has occurred while getting updates: {:?}", err);
                self.request = Some(Box::new(
                    sleep(self.error_timeout).from_err().map(|()| None),
                ));
            }
        };

        Ok(Async::NotReady)
    }
}
