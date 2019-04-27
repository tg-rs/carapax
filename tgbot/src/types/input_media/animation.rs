use crate::types::{Integer, ParseMode};
use serde::Serialize;

/// Animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent
#[derive(Clone, Default, Debug, Serialize)]
pub struct InputMediaAnimation {
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
}

impl InputMediaAnimation {
    /// Set a thumbnail
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size
    /// A thumbnail‘s width and height should not exceed 320
    /// Ignored if the file is not uploaded using multipart/form-data
    /// Thumbnails can’t be reused and can be only uploaded
    /// as a new file, so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data
    /// under <file_attach_name>
    pub fn thumb<S: Into<String>>(mut self, thumb: S) -> Self {
        self.thumb = Some(thumb.into());
        self
    }

    /// Caption of the animation to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Set width
    pub fn width(mut self, width: Integer) -> Self {
        self.width = Some(width);
        self
    }

    /// Set height
    pub fn height(mut self, height: Integer) -> Self {
        self.height = Some(height);
        self
    }

    /// Set duration
    pub fn duration(mut self, duration: Integer) -> Self {
        self.duration = Some(duration);
        self
    }
}
