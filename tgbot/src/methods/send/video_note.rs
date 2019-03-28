use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, Integer, Message, ReplyMarkup},
};
use failure::Error;
use serde::Serialize;

/// Send video message
///
/// As of v.4.0, Telegram clients support rounded square mp4 videos of up to 1 minute long
#[derive(Clone, Debug, Serialize)]
pub struct SendVideoNote {
    chat_id: ChatId,
    video_note: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    length: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl SendVideoNote {
    /// Creates a new SendVideoNote with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * video_note - Video note to send
    ///                Pass a file_id as String to send a video note that exists on the Telegram servers (recommended)
    ///                or upload a new video using multipart/form-data
    ///                Sending video notes by a URL is currently unsupported
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, video_note: S) -> Self {
        SendVideoNote {
            chat_id: chat_id.into(),
            video_note: video_note.into(),
            duration: None,
            length: None,
            thumb: None,
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

    /// Video width and height, i.e. diameter of the video message
    pub fn length(mut self, length: Integer) -> Self {
        self.length = Some(length);
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

impl Method for SendVideoNote {
    type Response = Message;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("sendVideoNote", &self)
    }
}
