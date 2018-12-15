use crate::types::location::Location;
use crate::types::primitive::{Float, Integer};

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Debug, Deserialize, Serialize)]
pub struct Animation {
    /// Unique file identifier
    pub file_id: String,
    /// Animation width as defined by sender
    pub width: Integer,
    /// Animation height as defined by sender
    pub height: Integer,
    /// Duration of the video in seconds as defined by sender
    pub duration: Integer,
    /// Animation thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Original animation filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<Integer>,
}

/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Debug, Deserialize, Serialize)]
pub struct Audio {
    /// Unique identifier for this file
    pub file_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: Integer,
    /// Performer of the audio as defined by sender or by audio tags
    pub performer: Option<String>,
    /// Title of the audio as defined by sender or by audio tags
    pub title: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<Integer>,
    /// Thumbnail of the album cover to which the music file belongs
    pub thumb: Option<PhotoSize>,
}

/// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Debug, Deserialize, Serialize)]
pub struct Document {
    /// Unique file identifier
    pub file_id: String,
    /// Document thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Original filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<Integer>,
}

/// This object represents a file ready to be downloaded.
/// The file can be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>.
/// It is guaranteed that the link will be valid for at least 1 hour.
/// When the link expires, a new one can be requested by calling getFile.
/// Maximum file size to download is 20 MB
#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    /// Unique identifier for this file
    pub file_id: String,
    /// File size, if known
    pub file_size: Option<Integer>,
    /// File path.
    /// Use https://api.telegram.org/file/bot<token>/<file_path> to get the file.
    pub file_path: Option<String>,
}

/// This object describes the position on faces where a mask should be placed by default.
#[derive(Debug, Deserialize, Serialize)]
pub struct MaskPosition {
    /// The part of the face relative
    /// to which the mask should be placed.
    pub point: MaskPositionPoint,
    /// Shift by X-axis measured in widths
    /// of the mask scaled to the face size,
    /// from left to right.
    /// For example, choosing -1.0
    /// will place mask just
    /// to the left of the default mask position.
    pub x_shift: Float,
    /// Shift by Y-axis measured
    /// in heights of the mask scaled to the face size,
    /// from top to bottom.
    /// For example, 1.0 will place
    /// the mask just below the default mask position.
    pub y_shift: Float,
    /// Mask scaling coefficient.
    /// For example, 2.0 means double size.
    pub scale: Float,
}

/// The part of the face relative
/// to which the mask should be placed.
#[derive(Debug, Deserialize, Serialize)]
pub enum MaskPositionPoint {
    /// “forehead”
    Forehead,
    /// “eyes”
    Eyes,
    /// “mouth”
    Mouth,
    /// “chin”
    Chin,
}

/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Debug, Deserialize, Serialize)]
pub struct PhotoSize {
    /// Unique identifier for this file
    pub file_id: String,
    /// Photo width
    pub width: Integer,
    /// Photo height
    pub height: Integer,
    /// File size
    pub file_size: Option<Integer>,
}

/// This object represents a sticker.
#[derive(Debug, Deserialize, Serialize)]
pub struct Sticker {
    /// Unique identifier for this file
    pub file_id: String,
    /// Sticker width
    pub width: Integer,
    /// Sticker height
    pub height: Integer,
    /// Sticker thumbnail in the .webp or .jpg format
    pub thumb: Option<PhotoSize>,
    /// Emoji associated with the sticker
    pub emoji: Option<String>,
    /// Name of the sticker set to which the sticker belongs
    pub set_name: Option<String>,
    /// For mask stickers, the position where the mask should be placed
    pub mask_position: Option<MaskPosition>,
    /// File size
    pub file_size: Option<Integer>,
}

/// This object represents a sticker set.
#[derive(Debug, Deserialize, Serialize)]
pub struct StickerSet {
    /// Sticker set name
    pub name: String,
    /// Sticker set title
    pub title: String,
    /// True, if the sticker set contains masks
    pub contains_masks: bool,
    /// List of all set stickers
    pub stickers: Vec<Sticker>,
}

/// This object represents a venue.
#[derive(Debug, Deserialize, Serialize)]
pub struct Venue {
    /// Venue location
    pub location: Location,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue.
    /// For example: “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”.
    pub foursquare_type: Option<String>,
}

/// This object represents a video file.
#[derive(Debug, Deserialize, Serialize)]
pub struct Video {
    /// Unique identifier for this file
    pub file_id: String,
    /// Video width as defined by sender
    pub width: Integer,
    /// Video height as defined by sender
    pub height: Integer,
    /// Duration of the video in seconds as defined by sender
    pub duration: Integer,
    /// Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Mime type of a file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<Integer>,
}

/// This object represents a video message
#[derive(Debug, Deserialize, Serialize)]
pub struct VideoNote {
    /// Unique identifier for this file
    pub file_id: String,
    /// Video width and height
    pub length: Integer,
    ///  Duration of the video in seconds
    pub duration: Integer,
    /// Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// File size
    pub file_size: Option<Integer>,
}

/// This object represents a voice note.
#[derive(Debug, Deserialize, Serialize)]
pub struct Voice {
    /// Unique identifier for this file
    file_id: String,
    /// Duration of the audio in seconds as defined by sender
    duration: Integer,
    /// MIME type of the file as defined by sender
    mime_type: Option<String>,
    /// File size
    file_size: Option<Integer>,
}
