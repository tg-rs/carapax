use crate::types::primitive::Integer;

/// This object represents a file ready to be downloaded.
/// The file can be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>.
/// It is guaranteed that the link will be valid for at least 1 hour.
/// When the link expires, a new one can be requested by calling getFile.
/// Maximum file size to download is 20 MB
#[derive(Debug)]
pub struct File {
    /// Unique identifier for this file
    pub file_id: String,
    /// File size, if known
    pub file_size: Option<Integer>,
    /// File path.
    /// Use https://api.telegram.org/file/bot<token>/<file_path> to get the file.
    pub file_path: Option<String>,
}
