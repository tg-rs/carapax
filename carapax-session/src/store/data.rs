use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Error as JsonError, Value as JsonValue};
use std::time::{SystemTime, SystemTimeError};

#[derive(Serialize, Deserialize)]
pub(super) struct Data {
    expires_at: Option<u64>,
    value: JsonValue,
}

impl Data {
    pub(super) fn set_lifetime(&mut self, lifetime: u64) -> Result<(), SystemTimeError> {
        self.expires_at = Some(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|x| x.as_secs() + lifetime)?,
        );
        Ok(())
    }

    pub(super) fn get_expires_at(&self) -> Option<u64> {
        self.expires_at
    }

    pub(super) fn is_expired(&self) -> Result<bool, SystemTimeError> {
        Ok(match self.expires_at {
            Some(expires_at) => {
                let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .map(|x| x.as_secs())?;
                expires_at < now
            }
            None => false,
        })
    }

    pub(super) fn parse_value<T: DeserializeOwned>(mut self) -> Result<T, JsonError> {
        Ok(serde_json::from_value(self.value.take())?)
    }
}

#[derive(Serialize)]
pub(super) struct DataRef<'a, T>
where
    T: Serialize,
{
    expires_at: Option<u64>,
    value: &'a T,
}

impl<'a, T> DataRef<'a, T>
where
    T: Serialize + 'a,
{
    pub(super) fn new(value: &'a T) -> Self {
        Self {
            value,
            expires_at: None,
        }
    }

    pub(super) fn expires_at(mut self, expires_at: u64) -> Self {
        self.expires_at = Some(expires_at);
        self
    }
}
