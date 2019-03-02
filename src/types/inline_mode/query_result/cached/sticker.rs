use crate::types::inline_mode::message_content::InputMessageContent;
use crate::types::reply_markup::InlineKeyboardMarkup;
use serde::Serialize;

/// Link to a sticker stored on the Telegram servers
///
/// By default, this sticker will be sent by the user
/// Alternatively, you can use input_message_content to
/// send a message with the specified content instead of the sticker
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultCachedSticker {
    id: String,
    sticker_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedSticker {
    /// Creates a new InlineQueryResultCachedSticker with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * sticker_file_id - A valid file identifier of the sticker
    pub fn new<S: Into<String>>(id: S, sticker_file_id: S) -> Self {
        InlineQueryResultCachedSticker {
            id: id.into(),
            sticker_file_id: sticker_file_id.into(),
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the photo
    pub fn input_message_content(mut self, input_message_content: InputMessageContent) -> Self {
        self.input_message_content = Some(input_message_content);
        self
    }
}
