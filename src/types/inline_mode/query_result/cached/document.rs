use crate::types::inline_mode::message_content::InputMessageContent;
use crate::types::primitive::ParseMode;
use crate::types::reply_markup::InlineKeyboardMarkup;

/// Link to a file stored on the Telegram servers
///
/// By default, this file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the file
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultCachedDocument {
    id: String,
    title: String,
    document_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedDocument {
    /// Creates a new InlineQueryResultCachedDocument with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * title - Title for the result
    /// * document_file_id - A valid file identifier for the file
    pub fn new<S: Into<String>>(id: S, title: S, document_file_id: S) -> Self {
        InlineQueryResultCachedDocument {
            id: id.into(),
            title: title.into(),
            document_file_id: document_file_id.into(),
            description: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// Short description of the result
    pub fn description<S: Into<String>>(&mut self, description: S) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    /// Caption of the document to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(&mut self, caption: S) -> &mut Self {
        self.caption = Some(caption.into());
        self
    }

    /// Parse mode
    pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(&mut self, reply_markup: I) -> &mut Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the file
    pub fn input_message_content(
        &mut self,
        input_message_content: InputMessageContent,
    ) -> &mut Self {
        self.input_message_content = Some(input_message_content);
        self
    }
}
