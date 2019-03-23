use crate::types::{
    inline_mode::message_content::InputMessageContent,
    primitive::{Integer, ParseMode},
    reply_markup::InlineKeyboardMarkup,
};
use serde::Serialize;

/// Link to a page containing an embedded video player or a video file
///
/// By default, this video file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content to send a message with
/// the specified content instead of the video
/// If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube),
/// you must replace its content using input_message_content
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultVideo {
    id: String,
    video_url: String,
    mime_type: String,
    thumb_url: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultVideo {
    /// Creates a new InlineQueryResultVideo with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * video_url - A valid URL for the embedded video player or video file
    /// * mime_type - Mime type of the content of video url, “text/html” or “video/mp4”
    /// * thumb_url - URL of the thumbnail (jpeg only) for the video
    /// * title - Title for the result
    pub fn new<S: Into<String>>(id: S, video_url: S, mime_type: S, thumb_url: S, title: S) -> Self {
        InlineQueryResultVideo {
            id: id.into(),
            video_url: video_url.into(),
            mime_type: mime_type.into(),
            thumb_url: thumb_url.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            video_width: None,
            video_height: None,
            video_duration: None,
            description: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// Caption of the video to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Video width
    pub fn video_width(mut self, video_width: Integer) -> Self {
        self.video_width = Some(video_width);
        self
    }

    /// Video height
    pub fn video_height(mut self, video_height: Integer) -> Self {
        self.video_height = Some(video_height);
        self
    }

    /// Video duration in seconds
    pub fn video_duration(mut self, video_duration: Integer) -> Self {
        self.video_duration = Some(video_duration);
        self
    }

    /// Short description of the result
    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the video
    ///
    /// This field is required if InlineQueryResultVideo is used
    /// to send an HTML-page as a result (e.g., a YouTube video)
    pub fn input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }
}
