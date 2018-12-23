use crate::client::Client;
use crate::methods::GetUpdates;
use crate::types::{AllowedUpdate, Integer, Update};
use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;

const DEFAULT_LIMIT: Integer = 100;
const DEFAULT_POLL_TIMEOUT: Integer = 10;
const DEFAULT_ERROR_TIMEOUT: u64 = 5;

/// Updates iterator used for long polling
pub struct UpdatesIter<'a> {
    client: &'a Client,
    items: Vec<Update>,
    offset: Integer,
    limit: Integer,
    poll_timeout: Integer,
    error_timeout: Duration,
    allowed_updates: HashSet<AllowedUpdate>,
}

impl<'a> UpdatesIter<'a> {
    /// Creates a new updates iterator
    pub fn new(client: &'a Client) -> Self {
        UpdatesIter {
            client,
            items: Vec::new(),
            offset: 0,
            limit: DEFAULT_LIMIT,
            poll_timeout: DEFAULT_POLL_TIMEOUT,
            error_timeout: Duration::from_secs(DEFAULT_ERROR_TIMEOUT),
            allowed_updates: HashSet::new(),
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

impl<'a> Iterator for UpdatesIter<'a> {
    type Item = Update;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.items.is_empty() {
            return self.items.pop();
        }
        loop {
            let updates = self.client.execute(
                GetUpdates::default()
                    .offset(self.offset)
                    .limit(self.limit)
                    .timeout(self.poll_timeout)
                    .allowed_updates(self.allowed_updates.clone()),
            );
            match updates {
                Ok(items) => {
                    self.items = items;
                    if let Some(update) = self.items.pop() {
                        self.offset = update.id + 1;
                        return Some(update);
                    }
                }
                Err(err) => {
                    // TODO: log
                    println!("{:?}", err);
                    sleep(self.error_timeout);
                }
            };
        }
    }
}
