use super::data::{Data, DataRef};
use crate::{
    gc::GarbageCollector,
    session::{SessionKey, SessionLifetime},
    store::SessionStore,
};
use carapax::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Error as JsonError;
use snafu::{ResultExt, Snafu};
use std::{
    io::{Error as IoError, ErrorKind as IoErrorKind},
    num::ParseIntError,
    path::{Path, PathBuf},
    string::FromUtf8Error,
    time::{SystemTime, SystemTimeError},
};
use tokio::fs;

/// File system session store
///
/// Serialization and deserialization of input/output values implemented using serde_json
#[derive(Clone)]
pub struct FsSessionStore {
    root: PathBuf,
    session_lifetime: SessionLifetime,
}

impl FsSessionStore {
    /// Creates a new store
    ///
    /// # Arguments
    ///
    /// - root - A directory to store session data
    pub async fn open<P>(root: P) -> Result<Self, FsError>
    where
        P: Into<PathBuf>,
    {
        let root = root.into();
        fs::create_dir_all(root.clone())
            .await
            .with_context(|| CreateRoot { path: root.clone() })?;
        Ok(Self {
            root,
            session_lifetime: SessionLifetime::default(),
        })
    }

    /// Sets session lifetime
    ///
    /// You need to spawn a GC in order to remove old sessions
    /// (see [run_gc](../../fn.run_gc.html))
    pub fn with_lifetime<L>(mut self, lifetime: L) -> Self
    where
        L: Into<SessionLifetime>,
    {
        self.session_lifetime = lifetime.into();
        self
    }

    async fn key_to_path(&self, key: SessionKey) -> Result<PathBuf, FsError> {
        let key_root = self.root.clone().join(key.namespace());
        let file_path = key_root.join(key.name()).with_extension("json");
        let result = fs::metadata(key_root.clone()).await;
        let metadata = match result {
            Ok(metadata) => metadata,
            Err(err) => match err.kind() {
                IoErrorKind::NotFound => {
                    fs::create_dir_all(key_root.clone())
                        .await
                        .context(CreateSessionRoot {})?;
                    TimeMarker::create(key_root.clone()).await?;
                    fs::metadata(key_root.clone())
                        .await
                        .context(GetSessionRootMetadata {})?
                }
                _ => return Err(err).context(KeyToPath { key }),
            },
        };
        if metadata.is_dir() {
            Ok(file_path)
        } else {
            SessionRootOccupied { path: key_root }.fail()
        }
    }

    async fn read_data<P: AsRef<Path>>(key: SessionKey, path: P) -> Result<Option<Data>, FsError> {
        match fs::read(path).await {
            Ok(data) => Ok(Some(
                serde_json::from_slice(&data).with_context(|| ReadDataDeserialize { key: key.clone() })?,
            )),
            Err(error) => match error.kind() {
                IoErrorKind::NotFound => Ok(None),
                _ => Err(error).context(ReadDataIo { key }),
            },
        }
    }
}

#[async_trait]
impl SessionStore for FsSessionStore {
    type Error = FsError;

    async fn get<O>(&mut self, key: SessionKey) -> Result<Option<O>, Self::Error>
    where
        O: DeserializeOwned + Send + Sync,
    {
        let file_path = self.key_to_path(key.clone()).await?;
        match Self::read_data(key.clone(), file_path).await? {
            Some(data) => {
                if data
                    .is_expired()
                    .with_context(|| ReadDataExpirationTime { key: key.clone() })?
                {
                    Ok(None)
                } else {
                    Ok(Some(data.parse_value().context(ReadDataDeserialize { key })?))
                }
            }
            None => Ok(None),
        }
    }

    async fn set<I>(&mut self, key: SessionKey, val: &I) -> Result<(), Self::Error>
    where
        I: Serialize + Send + Sync,
    {
        let file_path = self.key_to_path(key.clone()).await?;
        let mut data = DataRef::new(val);
        if let Some(old_data) = Self::read_data(key.clone(), &file_path).await? {
            if !old_data
                .is_expired()
                .with_context(|| ReadDataExpirationTime { key: key.clone() })?
            {
                if let Some(expires_at) = old_data.get_expires_at() {
                    data = data.expires_at(expires_at);
                }
            }
        }
        let data = serde_json::to_vec(&data).with_context(|| WriteDataSerialize { key: key.clone() })?;
        fs::write(file_path, data).await.context(WriteDataIo { key })?;
        Ok(())
    }

    async fn expire(&mut self, key: SessionKey, seconds: usize) -> Result<(), Self::Error> {
        let file_path = self.key_to_path(key.clone()).await?;
        if let Some(mut data) = Self::read_data(key.clone(), &file_path).await? {
            data.set_lifetime(seconds as u64)
                .with_context(|| ExpireData { key: key.clone() })?;
            let data = serde_json::to_vec(&data).with_context(|| WriteDataSerialize { key: key.clone() })?;
            fs::write(file_path, data).await.context(WriteDataIo { key })?;
        }
        Ok(())
    }

    async fn del(&mut self, key: SessionKey) -> Result<(), Self::Error> {
        let file_path = self.key_to_path(key.clone()).await?;
        match fs::remove_file(file_path).await {
            Ok(()) => Ok(()),
            Err(err) => match err.kind() {
                IoErrorKind::NotFound => Ok(()),
                _ => Err(err).context(DeleteDataIo { key }),
            },
        }
    }
}

#[async_trait]
impl GarbageCollector for FsSessionStore {
    type Error = FsError;

    async fn collect(&mut self) -> Result<(), Self::Error> {
        let lifetime = match self.session_lifetime {
            SessionLifetime::Forever => return Ok(()),
            SessionLifetime::Duration(duration) => duration.as_secs(),
        };
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .context(GcGetCurrentTime)?
            .as_secs();
        let mut entries = fs::read_dir(self.root.clone()).await.context(GcReadDir)?;
        while let Some(session_dir) = entries.next_entry().await.context(GcReadDir)? {
            let session_path = session_dir.path();
            let timestamp = TimeMarker::read(session_path.clone()).await?;
            if now - timestamp >= lifetime {
                let mut session_entires = fs::read_dir(session_path.clone()).await.context(GcReadDir)?;
                while let Some(session_file) = session_entires.next_entry().await.context(GcReadDir)? {
                    fs::remove_file(session_file.path()).await.context(GcRemoveFile)?;
                }
                fs::remove_dir(session_path).await.context(GcRemoveDir)?;
            }
        }
        Ok(())
    }
}

const CREATE_TIME_MARKER: &str = ".__created";

struct TimeMarker;

impl TimeMarker {
    async fn create<P: AsRef<Path>>(root: P) -> Result<(), FsError> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .context(TimeMarkerInitValue)?;
        let now = format!("{}", now.as_secs());
        fs::write(root.as_ref().join(CREATE_TIME_MARKER), now)
            .await
            .context(TimeMarkerCreate)?;
        Ok(())
    }

    async fn read<P: AsRef<Path>>(root: P) -> Result<u64, FsError> {
        let data = fs::read(root.as_ref().join(CREATE_TIME_MARKER))
            .await
            .context(TimeMarkerRead)?;
        let data = String::from_utf8(data).context(TimeMarkerGetString)?;
        let timestamp = data.parse::<u64>().context(TimeMarkerParseValue)?;
        Ok(timestamp)
    }
}

/// An error occurred in store
#[derive(Debug, Snafu)]
pub enum FsError {
    /// Failed to create root directory for store
    #[snafu(display("failed to create root directory '{}' for store: {}", path.display(), source))]
    CreateRoot {
        /// Path to directory
        path: PathBuf,
        /// Source error
        source: IoError,
    },

    /// Failed to create directory for a session
    #[snafu(display("failed to create session root: {}", source))]
    CreateSessionRoot {
        /// Source error
        source: IoError,
    },

    /// Failed to delete data for a key
    #[snafu(display("failed to delete data for key '{}': {}", key, source))]
    DeleteDataIo {
        /// Key to delete
        key: SessionKey,
        /// Source error
        source: IoError,
    },

    /// Unable to set expiration time for a key
    #[snafu(display("unable to set expiration time for data under key '{}': {}", key, source))]
    ExpireData {
        /// Key to expire
        key: SessionKey,
        /// Source error
        source: SystemTimeError,
    },

    /// Unable to get current time when running GC
    #[snafu(display("unable to get current time: {}", source))]
    GcGetCurrentTime {
        /// Source error
        source: SystemTimeError,
    },

    /// Failed to read entries of a directory when running GC
    #[snafu(display("failed to read store entries: {}", source))]
    GcReadDir {
        /// Source error
        source: IoError,
    },

    /// Failed to remove session directory when running GC
    #[snafu(display("GC failed to remove session dir: {}", source))]
    GcRemoveDir {
        /// Source error
        source: IoError,
    },

    /// Failed to remove session file when running GC
    #[snafu(display("GC failed to remove session file: {}", source))]
    GcRemoveFile {
        /// Source error
        source: IoError,
    },

    /// Can not get session metadata for a session directory
    #[snafu(display("can not get session root metadata: {}", source))]
    GetSessionRootMetadata {
        /// Source error
        source: IoError,
    },

    /// Unable to convert a session key to FS path
    #[snafu(display("unable to convert session key '{}' to FS path: {}", key, source))]
    KeyToPath {
        /// Key to convert
        key: SessionKey,
        /// Source error
        source: IoError,
    },

    /// Can not deserialize session data for a key when reading data
    #[snafu(display("can not deserialize data for key '{}': {}", key, source))]
    ReadDataDeserialize {
        /// Key to serialize data for
        key: SessionKey,
        /// Source error
        source: JsonError,
    },

    /// Unable to get expiration time for a key when reading data
    #[snafu(display("unable to get expiration time of data for key '{}': {}", key, source))]
    ReadDataExpirationTime {
        /// Key to read
        key: SessionKey,
        /// Source error
        source: SystemTimeError,
    },

    /// Can not read session data from a file
    #[snafu(display("can not read data for key {}: {}", key, source))]
    ReadDataIo {
        /// Key to read
        key: SessionKey,
        /// Source error
        source: IoError,
    },

    /// Session directory is occupied by a file or a link
    #[snafu(display("session root '{}' is occupied", path.display()))]
    SessionRootOccupied {
        /// Path to a file
        path: PathBuf,
    },

    /// Can not create time marker for a session
    #[snafu(display("failed to create time marker: {}", source))]
    TimeMarkerCreate {
        /// Source error
        source: IoError,
    },

    /// Unable to get current time for time marker
    #[snafu(display("unable to initialize value for time marker: {}", source))]
    TimeMarkerInitValue {
        /// Source error
        source: SystemTimeError,
    },

    /// Failed to read data from time marker
    #[snafu(display("time marker contains non UTF-8 string: {}", source))]
    TimeMarkerGetString {
        /// Source error
        source: FromUtf8Error,
    },

    /// Failed to parse value for a time marker
    #[snafu(display("failed to parse time marker value: {}", source))]
    TimeMarkerParseValue {
        /// Source error
        source: ParseIntError,
    },

    /// Can not read time marker data from a file
    #[snafu(display("can not read time marker data: {}", source))]
    TimeMarkerRead {
        /// Source error
        source: IoError,
    },

    /// Can not write session data for a key
    #[snafu(display("can not write data for key '{}': {}", key, source))]
    WriteDataIo {
        /// Key to write
        key: SessionKey,
        /// Source error
        source: IoError,
    },

    /// Can not serialize session data for a key
    #[snafu(display("can not serialize data for key '{}': {}", key, source))]
    WriteDataSerialize {
        /// Key to write
        key: SessionKey,
        /// Source error
        source: JsonError,
    },
}
