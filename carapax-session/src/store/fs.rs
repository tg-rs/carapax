use crate::{
    gc::GarbageCollector,
    session::{SessionKey, SessionLifetime},
    store::{
        data::{Data, DataRef},
        SessionStore,
    },
};
use failure::{Error, Fail};
use futures::{
    future::{self, Either},
    Future, Stream,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{io::ErrorKind as IoErrorKind, path::PathBuf, time::SystemTime};
use tokio_fs as fs;

const CREATE_TIME_MARKER: &str = ".__created";

/// File system session store
///
/// Serialization and deserialization of input/output values implemented using serde_json
#[derive(Clone)]
pub struct FsSessionStore {
    root: PathBuf,
    session_lifetime: SessionLifetime,
}

#[derive(Debug, Fail)]
enum KeyError {
    #[fail(display = "Key path occupied: {:?}", _0)]
    PathOccupied(PathBuf),
}

impl FsSessionStore {
    /// Creates a new store
    ///
    /// # Arguments
    ///
    /// - root - A directory to store session data
    pub fn open<P>(root: P) -> Box<dyn Future<Item = Self, Error = Error> + Send>
    where
        P: Into<PathBuf>,
    {
        let root = root.into();
        Box::new(fs::create_dir_all(root.clone()).from_err().and_then(move |_| {
            Ok(Self {
                root,
                session_lifetime: SessionLifetime::default(),
            })
        }))
    }

    /// Sets session lifetime
    ///
    /// You need to spawn a GC in order to remove old sessions
    /// (see [spawn_gc](../../fn.spawn_gc.html))
    pub fn with_lifetime<L>(mut self, lifetime: L) -> Self
    where
        L: Into<SessionLifetime>,
    {
        self.session_lifetime = lifetime.into();
        self
    }

    fn key_to_path(&self, key: SessionKey) -> Box<dyn Future<Item = PathBuf, Error = Error> + Send> {
        let key_root = self.root.clone().join(key.namespace());
        let file_path = key_root.join(key.name()).with_extension("json");
        let key_root_clone = key_root.clone();
        Box::new(
            fs::metadata(key_root.clone())
                .then(move |result| match result {
                    Ok(metadata) => Either::A(future::ok(metadata)),
                    Err(err) => match err.kind() {
                        IoErrorKind::NotFound => Either::B(
                            fs::create_dir_all(key_root.clone())
                                .from_err()
                                .and_then(move |()| {
                                    SystemTime::now()
                                        .duration_since(SystemTime::UNIX_EPOCH)
                                        .map_err(Error::from)
                                })
                                .and_then(|now| {
                                    fs::write(key_root.join(CREATE_TIME_MARKER), format!("{}", now.as_secs()))
                                        .and_then(move |_| fs::metadata(key_root.clone()))
                                        .from_err()
                                }),
                        ),
                        _ => Either::A(future::err(err.into())),
                    },
                })
                .and_then(move |metadata| {
                    if metadata.is_dir() {
                        Ok(file_path)
                    } else {
                        Err(Error::from(KeyError::PathOccupied(key_root_clone)))
                    }
                }),
        )
    }
}

impl SessionStore for FsSessionStore {
    fn get<O>(&self, key: SessionKey) -> Box<dyn Future<Item = Option<O>, Error = Error> + Send>
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

    fn set<I>(&self, key: SessionKey, val: &I) -> Box<dyn Future<Item = (), Error = Error> + Send>
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

    fn expire(&self, key: SessionKey, seconds: usize) -> Box<dyn Future<Item = (), Error = Error> + Send> {
        Box::new(self.key_to_path(key).and_then(move |file_path| {
            fs::read(file_path.clone())
                .from_err()
                .and_then(|data| serde_json::from_slice::<Data>(&data).map_err(Error::from))
                .and_then(move |mut data| data.set_lifetime(seconds as u64).map(|()| data))
                .and_then(|data| serde_json::to_vec(&data).map_err(Error::from))
                .and_then(|data| fs::write(file_path, data).from_err().map(|_| ()))
        }))
    }

    fn del(&self, key: SessionKey) -> Box<dyn Future<Item = (), Error = Error> + Send> {
        Box::new(self.key_to_path(key).and_then(|file_path| {
            fs::remove_file(file_path).then(|r| match r {
                Ok(()) => Ok(()),
                Err(err) => match err.kind() {
                    IoErrorKind::NotFound => Ok(()),
                    _ => Err(err.into()),
                },
            })
        }))
    }
}

impl GarbageCollector for FsSessionStore {
    fn collect(&self) -> Box<dyn Future<Item = (), Error = Error> + Send> {
        let lifetime = match self.session_lifetime {
            SessionLifetime::Forever => return Box::new(future::ok(())),
            SessionLifetime::Duration(duration) => duration.as_secs(),
        };
        let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(err) => return Box::new(future::err(err.into())),
        };
        Box::new(fs::read_dir(self.root.clone()).from_err().and_then(move |stream| {
            stream.from_err().for_each(move |session_dir| {
                let session_path = session_dir.path();
                fs::read(session_path.clone().join(CREATE_TIME_MARKER))
                    .from_err()
                    .and_then(move |data| String::from_utf8(data).map_err(Error::from))
                    .and_then(move |timestamp| timestamp.parse::<u64>().map_err(Error::from))
                    .and_then(move |timestamp| {
                        if now - timestamp >= lifetime {
                            Either::A(
                                fs::read_dir(session_path.clone())
                                    .and_then(|stream| {
                                        stream.for_each(|session_file| fs::remove_file(session_file.path()))
                                    })
                                    .and_then(|_| fs::remove_dir(session_path))
                                    .from_err(),
                            )
                        } else {
                            Either::B(future::ok(()))
                        }
                    })
            })
        }))
    }
}
