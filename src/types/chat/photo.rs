/// This object represents a chat photo
#[derive(Debug, Deserialize)]
pub struct ChatPhoto {
    /// Unique file identifier of small (160x160) chat photo
    /// This file_id can be used only for photo download
    pub small_file_id: String,
    /// Unique file identifier of big (640x640) chat photo
    /// This file_id can be used only for photo download
    pub big_file_id: String,
}
