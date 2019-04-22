use crate::types::{
    photo_size::PhotoSize,
    primitive::{Float, Integer},
};
use serde::{Deserialize, Serialize};

/// The part of the face relative to which the mask should be placed
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Message, MessageData, Update, UpdateKind};

    #[test]
    fn parse_full() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 10000,
            "message": {
                "date": 1441645532,
                "chat": {
                    "last_name": "Test Lastname",
                    "type": "private",
                    "id": 1111111,
                    "first_name": "Test Firstname",
                    "username": "Testusername"
                },
                "message_id": 1365,
                "from": {
                    "last_name": "Test Lastname",
                    "id": 1111111,
                    "first_name": "Test Firstname",
                    "username": "Testusername",
                    "is_bot": false
                },
                "caption": "test caption",
                "sticker": {
                    "file_id": "test file id",
                    "width": 512,
                    "height": 512,
                    "thumb": {
                        "file_id": "AdddddUuUUUUccccUUmm_PPP",
                        "width": 24,
                        "height": 24,
                        "file_size": 12324
                    },
                    "emoji": ":D",
                    "set_name": "sticker set name",
                    "mask_position": {
                        "point": "forehead",
                        "x_shift": 1.0,
                        "y_shift": 2.0,
                        "scale": 3.0,
                    },
                    "file_size": 1234,
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Sticker(data),
            ..
        }) = update.kind
        {
            assert_eq!(data.file_id, String::from("test file id"));
            assert_eq!(data.width, 512);
            assert_eq!(data.height, 512);

            let thumb = data.thumb.unwrap();
            assert_eq!(thumb.file_id, String::from("AdddddUuUUUUccccUUmm_PPP"));
            assert_eq!(thumb.width, 24);
            assert_eq!(thumb.height, 24);
            assert_eq!(thumb.file_size.unwrap(), 12324);

            assert_eq!(data.emoji.unwrap(), String::from(":D"));
            assert_eq!(data.set_name.unwrap(), String::from("sticker set name"));

            let mask_position = data.mask_position.unwrap();
            assert_eq!(mask_position.point, MaskPositionPoint::Forehead);
            assert_eq!(mask_position.x_shift, 1.0);
            assert_eq!(mask_position.y_shift, 2.0);
            assert_eq!(mask_position.scale, 3.0);

            assert_eq!(data.file_size.unwrap(), 1234);
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }

    #[test]
    fn parse_partial() {
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 10000,
            "message": {
                "date": 1441645532,
                "chat": {
                    "last_name": "Test Lastname",
                    "type": "private",
                    "id": 1111111,
                    "first_name": "Test Firstname",
                    "username": "Testusername"
                },
                "message_id": 1365,
                "from": {
                    "last_name": "Test Lastname",
                    "id": 1111111,
                    "first_name": "Test Firstname",
                    "username": "Testusername",
                    "is_bot": false
                },
                "sticker": {
                    "file_id": "test file id",
                    "width": 512,
                    "height": 512,
                }
            }
        }))
        .unwrap();
        if let UpdateKind::Message(Message {
            data: MessageData::Sticker(data),
            ..
        }) = update.kind
        {
            assert_eq!(data.file_id, String::from("test file id"));
            assert_eq!(data.width, 512);
            assert_eq!(data.height, 512);
        } else {
            panic!("Unexpected update {:?}", update);
        }
    }
}
