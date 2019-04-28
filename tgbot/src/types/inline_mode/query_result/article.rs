use crate::types::{
    inline_mode::message_content::InputMessageContent, primitive::Integer, reply_markup::InlineKeyboardMarkup,
};
use serde::Serialize;

/// Link to an article or web page
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultArticle {
    id: String,
    title: String,
    input_message_content: InputMessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hide_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_height: Option<Integer>,
}

impl InlineQueryResultArticle {
    /// Creates a new InlineQueryResultArticle with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 Bytes
    /// * title - Title of the result
    /// * input_message_content - Content of the message to be sent
    pub fn new<I, T, C>(id: I, title: T, input_message_content: C) -> Self
    where
        I: Into<String>,
        T: Into<String>,
        C: Into<InputMessageContent>,
    {
        InlineQueryResultArticle {
            id: id.into(),
            title: title.into(),
            input_message_content: input_message_content.into(),
            reply_markup: None,
            url: None,
            hide_url: None,
            description: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// URL of the result
    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Pass True, if you don't want the URL to be shown in the message
    pub fn hide_url(mut self, hide_url: bool) -> Self {
        self.hide_url = Some(hide_url);
        self
    }

    /// Short description of the result
    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Url of the thumbnail for the result
    pub fn thumb_url<S: Into<String>>(mut self, thumb_url: S) -> Self {
        self.thumb_url = Some(thumb_url.into());
        self
    }

    /// Thumbnail width
    pub fn thumb_width(mut self, thumb_width: Integer) -> Self {
        self.thumb_width = Some(thumb_width);
        self
    }

    /// Thumbnail height
    pub fn thumb_height(mut self, thumb_height: Integer) -> Self {
        self.thumb_height = Some(thumb_height);
        self
    }
}
