use serde::Deserialize;

/// Chat photo
#[derive(Clone, Debug, Deserialize)]
pub struct ChatPhoto {
    /// Unique file identifier of small (160x160) chat photo
    /// This file_id can be used only for photo download
    pub small_file_id: String,
    /// Unique file identifier of big (640x640) chat photo
    /// This file_id can be used only for photo download
    pub big_file_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data: ChatPhoto = serde_json::from_value(serde_json::json!({
            "small_file_id": "small-id",
            "big_file_id": "big-id"
        }))
        .unwrap();
        assert_eq!(data.small_file_id, "small-id");
        assert_eq!(data.big_file_id, "big-id");
    }
}
