use crate::methods::method::*;
use crate::types::{ChatId, Integer, Message, ParseMode, ReplyMarkup};
use serde::Serialize;

/// Send audio files
///
/// Your audio must be in the .mp3 format
/// Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future
///
/// For sending voice messages, use the sendVoice method instead
#[derive(Clone, Debug, Serialize)]
pub struct SendAudio {
    chat_id: ChatId,
    audio: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl SendAudio {
    /// Creates a new SendAudio with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * audio - Audio file to send
    ///           Pass a file_id as String to send an audio file that exists on the Telegram servers (recommended),
    ///           pass an HTTP URL as a String for Telegram to get an audio file from the Internet,
    ///           or upload a new one using multipart/form-data
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, audio: S) -> Self {
        SendAudio {
            chat_id: chat_id.into(),
            audio: audio.into(),
            caption: None,
            parse_mode: None,
            duration: None,
            performer: None,
            title: None,
            thumb: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Sets audio caption (0-1024 characters)
    pub fn caption<S: Into<String>>(&mut self, caption: S) -> &mut Self {
        self.caption = Some(caption.into());
        self
    }

    /// Sets parse mode
    pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Sets duration of the audio in seconds
    pub fn duration(&mut self, duration: Integer) -> &mut Self {
        self.duration = Some(duration);
        self
    }

    /// Sets performer
    pub fn performer<S: Into<String>>(&mut self, performer: S) -> &mut Self {
        self.performer = Some(performer.into());
        self
    }

    /// Sets track name
    pub fn title<S: Into<String>>(&mut self, title: S) -> &mut Self {
        self.title = Some(title.into());
        self
    }

    /// Sets thumbnail of the file
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size
    /// A thumbnail‘s width and height should not exceed 90
    ///
    /// Ignored if the file is not uploaded using multipart/form-data
    /// Thumbnails can’t be reused and can be only uploaded as a new file,
    /// so you can pass “attach://<file_attach_name>” if the thumbnail
    /// was uploaded using multipart/form-data under <file_attach_name>
    pub fn thumb<S: Into<String>>(&mut self, thumb: S) -> &mut Self {
        self.thumb = Some(thumb.into());
        self
    }

    /// Sends the message silently
    ///
    ///Users will receive a notification with no sound
    pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(&mut self, reply_to_message_id: Integer) -> &mut Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(&mut self, reply_markup: R) -> &mut Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendAudio {
    type Response = Message;

    fn get_request(&self) -> Result<RequestBuilder, RequestError> {
        RequestBuilder::json("sendAudio", &self)
    }
}
