/// This object represents a voice note.
#[derive(Debug)]
pub struct Voice {
    /// Unique identifier for this file
    file_id: String,
    /// Duration of the audio in seconds as defined by sender
    duration: i64,
    /// MIME type of the file as defined by sender
    mime_type: Option<String>,
    /// File size
    file_size: Option<i64>,
}
