use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, Integer, Message, ParseMode, ReplyMarkup},
};
use failure::Error;
use serde::Serialize;

/// Send video file
///
/// Telegram clients support mp4 videos (other formats may be sent as Document)
/// Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future
#[derive(Clone, Debug, Serialize)]
pub struct SendVideo {
    chat_id: ChatId,
    video: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_streaming: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl SendVideo {
    /// Creates a new SendVideo with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * video - Video to send
    ///           Pass a file_id as String to send a video that exists on the Telegram servers (recommended),
    ///           pass an HTTP URL as a String for Telegram to get a video from the Internet,
    ///           or upload a new video using multipart/form-data
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, video: S) -> Self {
        SendVideo {
            chat_id: chat_id.into(),
            video: video.into(),
            duration: None,
            width: None,
            height: None,
            thumb: None,
            caption: None,
            parse_mode: None,
            supports_streaming: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Duration of sent video in seconds
    pub fn duration(mut self, duration: Integer) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Video width
    pub fn width(mut self, width: Integer) -> Self {
        self.width = Some(width);
        self
    }

    /// Video height
    pub fn height(mut self, height: Integer) -> Self {
        self.height = Some(height);
        self
    }

    /// Thumbnail of the file sent
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size
    /// A thumbnail‘s width and height should not exceed 90
    /// Ignored if the file is not uploaded using multipart/form-data
    /// Thumbnails can’t be reused and can be only uploaded as a new file,
    /// so you can pass “attach://<file_attach_name>” if the thumbnail was
    /// uploaded using multipart/form-data under <file_attach_name>
    pub fn thumb<S: Into<String>>(mut self, thumb: S) -> Self {
        self.thumb = Some(thumb.into());
        self
    }

    /// Video caption
    ///
    /// May also be used when resending videos by file_id
    /// 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Sets a parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Pass True, if the uploaded video is suitable for streaming
    pub fn supports_streaming(mut self, supports_streaming: bool) -> Self {
        self.supports_streaming = Some(supports_streaming);
        self
    }

    // Sends the message silently
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(mut self, reply_markup: R) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendVideo {
    type Response = Message;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("sendVideo", &self)
    }
}
