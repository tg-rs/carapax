use crate::{session::SessionKey, store::SessionStore};
use failure::{Error, Fail};
use futures::{
    future::{self, Either},
    Future,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::time::SystemTime;
use std::{io::ErrorKind as IoErrorKind, path::PathBuf};
use tokio_fs as fs;

/// File system session store
///
/// Serialization and deserialization of input/output values implemented using serde_json
#[derive(Clone)]
pub struct FsSessionStore {
    root: PathBuf,
}

#[derive(Debug, Fail)]
enum KeyError {
    #[fail(display = "Session key is empty")]
    Empty,
    #[fail(display = "Key path occupied: {:?}", _0)]
    PathOccupied(PathBuf),
}

impl FsSessionStore {
    /// Creates a new store
    ///
    /// # Arguments
    ///
    /// - root - A directory to store session data
    pub fn new<P>(root: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self { root: root.into() }
    }

    fn key_to_path(&self, key: SessionKey) -> Box<Future<Item = PathBuf, Error = Error> + Send> {
        let mut key_root = self.root.clone();
        let parts = key.into_inner();
        let parts_len = parts.len();
        let file_name = match parts_len {
            0 => return Box::new(future::err(KeyError::Empty.into())),
            1 => parts[0].clone(),
            _ => {
                for i in parts.iter().take(parts_len - 2) {
                    key_root = key_root.join(i)
                }
                parts[parts_len - 1].clone()
            }
        };
        let file_path = key_root.join(file_name).with_extension("json");
        let key_root_clone = key_root.clone();
        Box::new(
            fs::metadata(key_root.clone())
                .then(move |result| match result {
                    Ok(metadata) => Either::A(future::ok(metadata)),
                    Err(err) => match err.kind() {
                        IoErrorKind::NotFound => Either::B(
                            fs::create_dir_all(key_root.clone()).and_then(move |()| fs::metadata(key_root.clone())),
                        ),
                        _ => Either::A(future::err(err)),
                    },
                })
                .from_err()
                .and_then(move |metadata| {
                    if metadata.is_dir() {
                        Either::A(future::ok(file_path))
                    } else {
                        Either::B(future::err(Error::from(KeyError::PathOccupied(key_root_clone))))
                    }
                }),
        )
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    expires_at: Option<u64>,
    value: JsonValue,
}

impl Data {
    fn set_lifetime(&mut self, lifetime: u64) -> Result<(), Error> {
        self.expires_at = Some(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|x| x.as_secs() + lifetime)?,
        );
        Ok(())
    }

    fn is_expired(&self) -> Result<bool, Error> {
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

    fn parse_value<T: DeserializeOwned>(mut self) -> Result<T, Error> {
        Ok(serde_json::from_value(self.value.take())?)
    }
}

#[derive(Serialize)]
struct DataRef<'a, T>
where
    T: Serialize + 'a,
{
    expires_at: Option<u64>,
    value: &'a T,
}

impl<'a, T> DataRef<'a, T>
where
    T: Serialize + 'a,
{
    fn new(value: &'a T) -> Self {
        Self {
            value,
            expires_at: None,
        }
    }
}

impl SessionStore for FsSessionStore {
    fn get<O>(&self, key: SessionKey) -> Box<Future<Item = Option<O>, Error = Error> + Send>
    where
        O: DeserializeOwned + Send + 'static,
    {
        Box::new(self.key_to_path(key).and_then(|file_path| {
            fs::read(file_path).then(|result| match result {
                Ok(data) => {
                    let data: Data = serde_json::from_slice(&data)?;
                    if data.is_expired()? {
                        Ok(None)
                    } else {
                        Ok(Some(data.parse_value()?))
                    }
                }
                Err(error) => match error.kind() {
                    IoErrorKind::NotFound => Ok(None),
                    _ => Err(error.into()),
                },
            })
        }))
    }

    fn set<I>(&self, key: SessionKey, val: &I) -> Box<Future<Item = (), Error = Error> + Send>
    where
        I: Serialize,
    {
        match serde_json::to_vec(&DataRef::new(val)) {
            Ok(data) => Box::new(
                self.key_to_path(key)
                    .and_then(|file_path| fs::write(file_path, data).from_err().map(|_| ())),
            ),
            Err(err) => Box::new(future::err(err.into())),
        }
    }

    fn expire(&self, key: SessionKey, seconds: usize) -> Box<Future<Item = (), Error = Error> + Send> {
        Box::new(self.key_to_path(key).and_then(move |file_path| {
            fs::read(file_path.clone())
                .from_err()
                .and_then(move |data| match serde_json::from_slice::<Data>(&data) {
                    Ok(mut data) => match data.set_lifetime(seconds as u64) {
                        Ok(()) => match serde_json::to_vec(&data) {
                            Ok(data) => Either::A(fs::write(file_path, data).from_err().map(|_| ())),
                            Err(err) => Either::B(future::err(err.into())),
                        },
                        Err(err) => Either::B(future::err(err)),
                    },
                    Err(err) => Either::B(future::err(err.into())),
                })
        }))
    }

    fn del(&self, key: SessionKey) -> Box<Future<Item = (), Error = Error> + Send> {
        Box::new(
            self.key_to_path(key)
                .and_then(|file_path| fs::remove_file(file_path).from_err()),
        )
    }
}
