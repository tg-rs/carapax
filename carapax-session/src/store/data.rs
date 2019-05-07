use failure::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub(super) struct Data {
    expires_at: Option<u64>,
    value: JsonValue,
}

impl Data {
    pub(super) fn set_lifetime(&mut self, lifetime: u64) -> Result<(), Error> {
        self.expires_at = Some(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|x| x.as_secs() + lifetime)?,
        );
        Ok(())
    }

    pub(super) fn is_expired(&self) -> Result<bool, Error> {
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

    pub(super) fn parse_value<T: DeserializeOwned>(mut self) -> Result<T, Error> {
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
}
