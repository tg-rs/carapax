use crate::types::{
    inline_mode::message_content::InputMessageContent, primitive::Integer, reply_markup::InlineKeyboardMarkup,
};
use serde::Serialize;

/// Contact with a phone number
///
/// By default, this contact will be sent by the user
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the contact
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultContact {
    id: String,
    phone_number: String,
    first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_height: Option<Integer>,
}

impl InlineQueryResultContact {
    /// Creates a new InlineQueryResultContact with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * phone_number - Contact's phone number
    /// * first_name - Contact's first name
    pub fn new<I, P, N>(id: I, phone_number: P, first_name: N) -> Self
    where
        I: Into<String>,
        P: Into<String>,
        N: Into<String>,
    {
        InlineQueryResultContact {
            id: id.into(),
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }

    /// Contact's last name
    pub fn last_name<S: Into<String>>(mut self, last_name: S) -> Self {
        self.last_name = Some(last_name.into());
        self
    }

    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub fn vcard<S: Into<String>>(mut self, vcard: S) -> Self {
        self.vcard = Some(vcard.into());
        self
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the contact
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
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
