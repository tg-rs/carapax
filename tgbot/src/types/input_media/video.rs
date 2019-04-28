use crate::types::{Integer, ParseMode};
use serde::Serialize;

/// Video to be sent
#[derive(Clone, Default, Debug, Serialize)]
pub struct InputMediaVideo {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_streaming: Option<bool>,
}

impl InputMediaVideo {
    /// Caption of the video to be sent, 0-1024 characters
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

    /// Pass True, if the uploaded video is suitable for streaming
    pub fn supports_streaming(mut self, supports_streaming: bool) -> Self {
        self.supports_streaming = Some(supports_streaming);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_value(
                InputMediaVideo::default()
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
                    .width(200)
                    .height(200)
                    .duration(100)
                    .supports_streaming(true)
            )
            .unwrap(),
            serde_json::json!({
                "caption": "caption",
                "parse_mode": "Markdown",
                "width": 200,
                "height": 200,
                "duration": 100,
                "supports_streaming": true
            })
        );

        assert_eq!(
            serde_json::to_value(InputMediaVideo::default()).unwrap(),
            serde_json::json!({})
        );
    }
}
