use serde::Deserialize;

/// Chat photo
#[derive(Clone, Debug, Deserialize)]
pub struct ChatPhoto {
    /// File identifier of small (160x160) chat photo
    ///
    /// This file_id can be used only for photo download
    /// and only for as long as the photo is not changed.
    pub small_file_id: String,
    /// Unique file identifier of small (160x160) chat photo.
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub small_file_unique_id: String,
    /// File identifier of big (640x640) chat photo
    ///
    /// This file_id can be used only for photo download
    /// and only for as long as the photo is not changed.
    pub big_file_id: String,
    /// Unique file identifier of big (640x640) chat photo.
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub big_file_unique_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data: ChatPhoto = serde_json::from_value(serde_json::json!({
            "small_file_id": "small-id",
            "big_file_id": "big-id",
            "small_file_unique_id": "small-unique-id",
            "big_file_unique_id": "big-unique-id"
        }))
        .unwrap();
        assert_eq!(data.small_file_id, "small-id");
        assert_eq!(data.big_file_id, "big-id");
        assert_eq!(data.small_file_unique_id, "small-unique-id");
        assert_eq!(data.big_file_unique_id, "big-unique-id");
    }
}
