use crate::types::photo_size::PhotoSize;
use crate::types::primitive::{Float, Integer};
use serde::{Deserialize, Serialize};

/// The part of the face relative to which the mask should be placed
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MaskPositionPoint {
    /// “forehead”
    #[serde(rename = "forehead")]
    Forehead,
    /// “eyes”
    #[serde(rename = "eyes")]
    Eyes,
    /// “mouth”
    #[serde(rename = "mouth")]
    Mouth,
    /// “chin”
    #[serde(rename = "chin")]
    Chin,
}

/// Position on faces where a mask should be placed by default
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaskPosition {
    /// The part of the face relative
    /// to which the mask should be placed
    pub point: MaskPositionPoint,
    /// Shift by X-axis measured in widths
    /// of the mask scaled to the face size,
    /// from left to right.
    /// For example, choosing -1.0
    /// will place mask just
    /// to the left of the default mask position
    pub x_shift: Float,
    /// Shift by Y-axis measured
    /// in heights of the mask scaled to the face size,
    /// from top to bottom.
    /// For example, 1.0 will place
    /// the mask just below the default mask position
    pub y_shift: Float,
    /// Mask scaling coefficient.
    /// For example, 2.0 means double size
    pub scale: Float,
}

/// Sticker
#[derive(Clone, Debug, Deserialize)]
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

/// Sticker set
#[derive(Clone, Debug, Deserialize)]
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
