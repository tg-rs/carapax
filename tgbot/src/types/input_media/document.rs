use crate::types::ParseMode;
use serde::Serialize;

/// General file to be sent
#[derive(Clone, Default, Debug, Serialize)]
pub struct InputMediaDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
}

impl InputMediaDocument {
    /// Caption of the document to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
}
