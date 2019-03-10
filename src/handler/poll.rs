use crate::{
    api::Api,
    methods::GetUpdates,
    types::{AllowedUpdate, Integer, ResponseError, Update},
};
use failure::Error;
use futures::{task, Async, Future, Poll, Stream};
use log::error;
use std::{
    cmp::max,
    collections::{HashSet, VecDeque},
    time::Duration,
};
use tokio_timer::sleep;

const DEFAULT_LIMIT: Integer = 100;
const DEFAULT_POLL_TIMEOUT: Integer = 10;
const DEFAULT_ERROR_TIMEOUT: u64 = 5;

/// Updates stream used for long polling
pub struct UpdatesStream {
    api: Api,
    options: UpdatesStreamOptions,
    items: VecDeque<Update>,
    request: Option<Box<Future<Item = Option<Vec<Update>>, Error = Error> + Send>>,
}

impl UpdatesStream {
    /// Creates a new updates stream
    pub fn new(api: Api) -> Self {
        UpdatesStream {
            api,
            options: UpdatesStreamOptions::default(),
            items: VecDeque::new(),
            request: None,
        }
    }

    /// Set options
    pub fn options(mut self, options: UpdatesStreamOptions) -> Self {
        self.options = options;
        self
    }
}

impl From<Api> for UpdatesStream {
    fn from(api: Api) -> UpdatesStream {
        UpdatesStream::new(api)
    }
}

impl Stream for UpdatesStream {
    type Item = Update;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(update) = self.items.pop_front() {
            return Ok(Async::Ready(Some(update)));
        }

        let options = &mut self.options;

        let should_request = match self.request {
            Some(ref mut request) => match request.poll() {
                Ok(Async::Ready(Some(items))) => {
                    for i in items {
                        options.offset = max(options.offset, i.id);
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
                            &GetUpdates::default()
                                .offset(options.offset + 1)
                                .limit(options.limit)
                                .timeout(options.poll_timeout)
                                .allowed_updates(options.allowed_updates.clone()),
                        )
                        .map(Some),
                ));
            }
            Err(err) => {
                error!("An error has occurred while getting updates: {:?}", err);

                options.error_timeout = Duration::from_secs(
                    err.downcast::<ResponseError>()
                        .ok()
                        .map(|err| {
                            err.parameters
                                .map(|parameters| {
                                    parameters
                                        .retry_after
                                        .map(|count| count as u64)
                                        .unwrap_or(DEFAULT_ERROR_TIMEOUT)
                                })
                                .unwrap_or(DEFAULT_ERROR_TIMEOUT)
                        })
                        .unwrap_or(DEFAULT_ERROR_TIMEOUT),
                );

                self.request = Some(Box::new(sleep(options.error_timeout).from_err().map(|()| None)));
            }
        };

        task::current().notify();

        Ok(Async::NotReady)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct UpdatesStreamOptions {
    offset: Integer,
    limit: Integer,
    poll_timeout: Integer,
    error_timeout: Duration,
    allowed_updates: HashSet<AllowedUpdate>,
}

impl UpdatesStreamOptions {
    /// Limits the number of updates to be retrieved
    ///
    /// Values between 1—100 are accepted
    /// Defaults to 100
    pub fn limit(mut self, limit: Integer) -> Self {
        self.limit = limit;
        self
    }

    /// Timeout in seconds for long polling
    ///
    /// 0 - usual short polling
    /// Defaults to 10
    /// Should be positive, short polling should be used for testing purposes only
    pub fn poll_timeout(mut self, poll_timeout: Integer) -> Self {
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

impl Default for UpdatesStreamOptions {
    fn default() -> Self {
        UpdatesStreamOptions {
            offset: 0,
            limit: DEFAULT_LIMIT,
            poll_timeout: DEFAULT_POLL_TIMEOUT,
            error_timeout: Duration::from_secs(DEFAULT_ERROR_TIMEOUT),
            allowed_updates: HashSet::new(),
        }
    }
}
