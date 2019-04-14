use crate::{
    api::{Api, ApiFuture},
    methods::GetUpdates,
    types::{AllowedUpdate, Integer, ResponseError, Update},
};
use failure::Error;
use futures::{try_ready, Async, Future, Poll, Stream};
use log::error;
use std::{
    cmp::max,
    collections::{HashSet, VecDeque},
    mem,
    time::Duration,
};
use tokio_timer::sleep;

const DEFAULT_LIMIT: Integer = 100;
const DEFAULT_POLL_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_ERROR_TIMEOUT: Duration = Duration::from_secs(5);

enum UpdateState {
    BufferedResults(VecDeque<Update>),
    Running(ApiFuture<Vec<Update>>),
    Idling(tokio_timer::Delay),
}

/// Updates stream used for long polling
#[must_use = "streams must be polled to produce results"]
pub struct UpdatesStream {
    api: Api,
    options: UpdatesStreamOptions,
    state: UpdateState,
}

fn make_request(api: &Api, options: &UpdatesStreamOptions) -> ApiFuture<Vec<Update>> {
    api.execute(
        GetUpdates::default()
            .offset(options.offset + 1)
            .limit(options.limit)
            .timeout(options.poll_timeout)
            .allowed_updates(options.allowed_updates.clone()),
    )
}

impl UpdateState {
    fn switch_to_idle(&mut self, err: Error) {
        error!("An error has occurred while getting updates: {:?}", err);
        let error_timeout = err
            .downcast::<ResponseError>()
            .ok()
            .and_then(|err| {
                err.parameters
                    .and_then(|parameters| parameters.retry_after.map(|count| Duration::from_secs(count as u64)))
            })
            .unwrap_or(DEFAULT_ERROR_TIMEOUT);
        mem::replace(self, UpdateState::Idling(sleep(error_timeout)));
    }

    fn switch_to_request(&mut self, api: &Api, options: &UpdatesStreamOptions) {
        let fut = make_request(api, options);
        mem::replace(self, UpdateState::Running(fut));
    }

    fn switch_to_buffered(&mut self, items: impl IntoIterator<Item = Update>) {
        mem::replace(self, UpdateState::BufferedResults(items.into_iter().collect()));
    }
}

impl Stream for UpdatesStream {
    type Item = Update;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            match &mut self.state {
                UpdateState::BufferedResults(buffered) => {
                    if let Some(update) = buffered.pop_front() {
                        self.options.offset = max(self.options.offset, update.id);
                        return Ok(Async::Ready(Some(update)));
                    } else {
                        self.state.switch_to_request(&self.api, &self.options);
                    }
                }
                UpdateState::Running(request_fut) => match request_fut.poll() {
                    Ok(Async::NotReady) => return Ok(Async::NotReady),
                    Ok(Async::Ready(items)) => self.state.switch_to_buffered(items),
                    Err(err) => self.state.switch_to_idle(err),
                },
                UpdateState::Idling(delay_fut) => {
                    // Timer errors are unrecoverable.
                    try_ready!(delay_fut.poll());
                    self.state.switch_to_request(&self.api, &self.options)
                }
            }
        }
    }
}

impl UpdatesStream {
    /// Creates a new updates stream
    pub fn new(api: Api) -> Self {
        let options = UpdatesStreamOptions::default();
        let state = UpdateState::Running(make_request(&api, &options));
        UpdatesStream { api, options, state }
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

/// Update stream options
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UpdatesStreamOptions {
    offset: Integer,
    limit: Integer,
    poll_timeout: Duration,
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

impl Default for UpdatesStreamOptions {
    fn default() -> Self {
        UpdatesStreamOptions {
            offset: 0,
            limit: DEFAULT_LIMIT,
            poll_timeout: DEFAULT_POLL_TIMEOUT,
            error_timeout: DEFAULT_ERROR_TIMEOUT,
            allowed_updates: HashSet::new(),
        }
    }
}
