use crate::types::primitive::Integer;
use mime::Mime;
use serde::Deserialize;
use std::{fmt, io::Read, path::PathBuf};

/// File ready to be downloaded
///
/// The file can be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>
/// It is guaranteed that the link will be valid for at least 1 hour
/// When the link expires, a new one can be requested by calling getFile
/// Maximum file size to download is 20 MB
#[derive(Clone, Debug, Deserialize)]
pub struct File {
    /// Unique identifier for this file
    pub file_id: String,
    /// File size, if known
    pub file_size: Option<Integer>,
    /// File path
    /// Use https://api.telegram.org/file/bot<token>/<file_path> to get the file
    pub file_path: Option<String>,
}

/// Information about a file for reader
#[derive(Clone, Debug)]
pub struct InputFileInfo {
    pub(crate) name: String,
    pub(crate) mime_type: Option<Mime>,
}

impl InputFileInfo {
    /// Creates a new info object with given file name
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            mime_type: None,
        }
    }

    /// Sets mime type of a file
    pub fn mime_type(mut self, mime_type: Mime) -> Self {
        self.mime_type = Some(mime_type);
        self
    }
}

impl From<&str> for InputFileInfo {
    fn from(name: &str) -> Self {
        InputFileInfo::new(name)
    }
}

impl From<(&str, Mime)> for InputFileInfo {
    fn from((name, mime_type): (&str, Mime)) -> Self {
        InputFileInfo::new(name).mime_type(mime_type)
    }
}

impl From<String> for InputFileInfo {
    fn from(name: String) -> Self {
        InputFileInfo::new(name)
    }
}

impl From<(String, Mime)> for InputFileInfo {
    fn from((name, mime_type): (String, Mime)) -> Self {
        InputFileInfo::new(name).mime_type(mime_type)
    }
}

/// File reader to upload
pub struct InputFileReader {
    pub(crate) reader: Box<Read + Send>,
    pub(crate) info: Option<InputFileInfo>,
}

impl InputFileReader {
    /// Creates a new file reader
    pub fn new<R>(reader: R) -> Self
    where
        R: Read + Send + 'static,
    {
        InputFileReader {
            reader: Box::new(reader),
            info: None,
        }
    }

    /// Sets a file info
    pub fn info<I: Into<InputFileInfo>>(mut self, info: I) -> Self {
        self.info = Some(info.into());
        self
    }
}

impl<R> From<R> for InputFileReader
where
    R: Read + Send + 'static,
{
    fn from(reader: R) -> Self {
        InputFileReader::new(reader)
    }
}

/// File to upload
#[derive(Debug)]
pub struct InputFile {
    pub(crate) kind: InputFileKind,
}

impl InputFile {
    /// Send a file_id that exists on the Telegram servers
    pub fn file_id<S: Into<String>>(file_id: S) -> Self {
        Self {
            kind: InputFileKind::Id(file_id.into()),
        }
    }

    /// Send an HTTP URL to get a file from the Internet
    ///
    /// Telegram will download a file from that URL
    pub fn url<S: Into<String>>(url: S) -> Self {
        Self {
            kind: InputFileKind::Url(url.into()),
        }
    }

    /// Path to file in FS (will be uploaded using multipart/form-data)
    pub fn path<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            kind: InputFileKind::Path(path.into()),
        }
    }

    /// A reader (file will be uploaded using multipart/form-data)
    pub fn reader<R: Into<InputFileReader>>(reader: R) -> Self {
        Self {
            kind: InputFileKind::Reader(reader.into()),
        }
    }
}

pub(crate) enum InputFileKind {
    Id(String),
    Url(String),
    Path(PathBuf),
    Reader(InputFileReader),
}

impl fmt::Debug for InputFileKind {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputFileKind::Id(ref s) => write!(out, "InputFile::Id({:?})", s),
            InputFileKind::Url(ref s) => write!(out, "InputFile::Url({:?})", s),
            InputFileKind::Path(ref s) => write!(out, "InputFile::Path({:?})", s),
            InputFileKind::Reader(_) => write!(out, "InputFile::Reader(...)"),
        }
    }
}

impl From<InputFileReader> for InputFile {
    fn from(reader: InputFileReader) -> Self {
        Self::reader(reader)
    }
}
