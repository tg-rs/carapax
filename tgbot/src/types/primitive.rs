use serde::Serialize;

/// Telegram Integer type
pub type Integer = i64;

/// Telegram Float type
pub type Float = f32;

/// Send Markdown or HTML,
/// if you want Telegram apps to show
/// bold, italic, fixed-width text or
/// inline URLs in the media caption.
#[derive(Clone, Copy, Debug, Serialize)]
pub enum ParseMode {
    /// HTML
    #[serde(rename = "HTML")]
    Html,
    /// Markdown
    Markdown,
    /// MarkdownV2 style
    MarkdownV2,
}

impl ToString for ParseMode {
    fn to_string(&self) -> String {
        String::from(match self {
            ParseMode::Html => "HTML",
            ParseMode::Markdown => "Markdown",
            ParseMode::MarkdownV2 => "MarkdownV2",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mode() {
        assert_eq!(serde_json::to_string(&ParseMode::Html).unwrap(), r#""HTML""#);
        assert_eq!(serde_json::to_string(&ParseMode::Markdown).unwrap(), r#""Markdown""#);
        assert_eq!(
            serde_json::to_string(&ParseMode::MarkdownV2).unwrap(),
            r#""MarkdownV2""#
        );
        assert_eq!(ParseMode::Html.to_string(), "HTML");
        assert_eq!(ParseMode::Markdown.to_string(), "Markdown");
        assert_eq!(ParseMode::MarkdownV2.to_string(), "MarkdownV2");
    }
}
