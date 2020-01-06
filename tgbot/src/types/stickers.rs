use crate::types::{
    photo_size::PhotoSize,
    primitive::{Float, Integer},
};
use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;
use std::{error::Error as StdError, fmt};

/// The part of the face relative to which the mask should be placed
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
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
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
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

impl MaskPosition {
    pub(crate) fn serialize(&self) -> Result<String, MaskPositionError> {
        serde_json::to_string(self).map_err(MaskPositionError::Serialize)
    }
}

/// An error occurred with mask position
#[derive(Debug)]
pub enum MaskPositionError {
    /// Failed to serialize mask position
    Serialize(JsonError),
}

impl StdError for MaskPositionError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            MaskPositionError::Serialize(err) => Some(err),
        }
    }
}

impl fmt::Display for MaskPositionError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MaskPositionError::Serialize(err) => write!(out, "can not serialize mask position: {}", err),
        }
    }
}

/// Sticker
#[derive(Clone, Debug, Deserialize)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
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
    /// True, if the sticker is animated
    pub is_animated: bool,
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
    /// True, if the sticker set contains animated stickers
    pub is_animated: bool,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]
    use super::*;

    #[test]
    fn deserialize_sticker_full() {
        let data: Sticker = serde_json::from_value(serde_json::json!({
            "file_id": "test file id",
            "file_unique_id": "unique-id",
            "width": 512,
            "height": 512,
            "thumb": {
                "file_id": "AdddddUuUUUUccccUUmm_PPP",
                "file_unique_id": "unique-thumb-id",
                "width": 24,
                "height": 24,
                "file_size": 12324
            },
            "emoji": ":D",
            "set_name": "sticker set name",
            "mask_position": {
                "point": "forehead",
                "x_shift": 3.0,
                "y_shift": 2.0,
                "scale": 3.0,
            },
            "file_size": 1234,
            "is_animated": false
        }))
        .unwrap();

        assert_eq!(data.file_id, "test file id");
        assert_eq!(data.file_unique_id, "unique-id");
        assert_eq!(data.width, 512);
        assert_eq!(data.height, 512);
        assert!(!data.is_animated);

        let thumb = data.thumb.unwrap();
        assert_eq!(thumb.file_id, "AdddddUuUUUUccccUUmm_PPP");
        assert_eq!(thumb.file_unique_id, "unique-thumb-id");
        assert_eq!(thumb.width, 24);
        assert_eq!(thumb.height, 24);
        assert_eq!(thumb.file_size.unwrap(), 12324);

        assert_eq!(data.emoji.unwrap(), ":D");
        assert_eq!(data.set_name.unwrap(), "sticker set name");

        let mask_position = data.mask_position.unwrap();
        assert_eq!(mask_position.point, MaskPositionPoint::Forehead);
        assert_eq!(mask_position.x_shift, 3.0);
        assert_eq!(mask_position.y_shift, 2.0);
        assert_eq!(mask_position.scale, 3.0);

        assert_eq!(data.file_size.unwrap(), 1234);
    }

    #[test]
    fn deserialize_sticker_partial() {
        let data: Sticker = serde_json::from_value(serde_json::json!({
            "file_id": "test file id",
            "file_unique_id": "unique-id",
            "width": 512,
            "height": 512,
            "is_animated": true
        }))
        .unwrap();

        assert_eq!(data.file_id, "test file id");
        assert_eq!(data.file_unique_id, "unique-id");
        assert_eq!(data.width, 512);
        assert_eq!(data.height, 512);
        assert!(data.is_animated);
        assert!(data.thumb.is_none());
        assert!(data.emoji.is_none());
        assert!(data.set_name.is_none());
        assert!(data.file_size.is_none());
    }

    #[test]
    fn mask_position_point() {
        assert_eq!(
            serde_json::to_string(&MaskPositionPoint::Forehead).unwrap(),
            r#""forehead""#
        );
        assert_eq!(serde_json::to_string(&MaskPositionPoint::Eyes).unwrap(), r#""eyes""#);
        assert_eq!(serde_json::to_string(&MaskPositionPoint::Mouth).unwrap(), r#""mouth""#);
        assert_eq!(serde_json::to_string(&MaskPositionPoint::Chin).unwrap(), r#""chin""#);

        assert_eq!(
            serde_json::from_str::<MaskPositionPoint>(r#""forehead""#).unwrap(),
            MaskPositionPoint::Forehead
        );
        assert_eq!(
            serde_json::from_str::<MaskPositionPoint>(r#""eyes""#).unwrap(),
            MaskPositionPoint::Eyes
        );
        assert_eq!(
            serde_json::from_str::<MaskPositionPoint>(r#""mouth""#).unwrap(),
            MaskPositionPoint::Mouth
        );
        assert_eq!(
            serde_json::from_str::<MaskPositionPoint>(r#""chin""#).unwrap(),
            MaskPositionPoint::Chin
        );
    }

    #[test]
    fn deserialize_sticker_set() {
        let data: StickerSet = serde_json::from_value(serde_json::json!({
            "name": "test",
            "title": "test",
            "contains_masks": false,
            "stickers": [],
            "is_animated": false
        }))
        .unwrap();
        assert_eq!(data.name, "test");
        assert_eq!(data.title, "test");
        assert!(!data.is_animated);
        assert!(!data.contains_masks);
        assert!(data.stickers.is_empty());
    }
}
