use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, EditMessageResult, InlineKeyboardMarkup, Integer, ParseMode},
};
use failure::Error;
use serde::Serialize;

/// Edit text and game messages sent by the bot or via the bot (for inline bots)
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageText {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageText {
    /// Creates a new EditMessageText
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    /// * text - New text of the message
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, message_id: Integer, text: S) -> Self {
        EditMessageText {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            text: text.into(),
            parse_mode: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    /// Creates a new EditMessageText
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    /// * text - New text of the message
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S, text: S) -> Self {
        EditMessageText {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            text: text.into(),
            parse_mode: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    /// Parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Disables link previews for links in this message
    pub fn disable_web_page_preview(mut self, disable_web_page_preview: bool) -> Self {
        self.disable_web_page_preview = Some(disable_web_page_preview);
        self
    }

    /// Inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for EditMessageText {
    type Response = EditMessageResult;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("editMessageText", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::InlineKeyboardButton,
    };
    use serde_json::Value;

    #[test]
    fn edit_message_text() {
        let request = EditMessageText::new(1, 2, "text")
            .parse_mode(ParseMode::Markdown)
            .disable_web_page_preview(true)
            .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/editMessageText");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["message_id"], 2);
            assert_eq!(data["text"], "text");
            assert_eq!(data["parse_mode"], "Markdown");
            assert_eq!(data["disable_web_page_preview"], true);
            assert_eq!(data["reply_markup"]["inline_keyboard"][0][0]["text"], "text");
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }

        let request = EditMessageText::with_inline_message_id("msg-id", "text")
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/editMessageText");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["inline_message_id"], "msg-id");
            assert_eq!(data["text"], "text");
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
