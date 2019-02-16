use crate::methods::method::*;
use crate::types::{ChatId, EditMessageResult, Float, InlineKeyboardMarkup, Integer};
use failure::Error;
use serde::Serialize;

/// Edit live location messages sent by the bot or via the bot (for inline bots)
///
/// A location can be edited until its live_period expires or editing
/// is explicitly disabled by a call to stopMessageLiveLocation
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageLiveLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    latitude: Float,
    longitude: Float,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageLiveLocation {
    /// Creates a new EditMessageLiveLocation
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    /// * latitude - Latitude of new location
    /// * longitude Longitude of new location
    pub fn new<C: Into<ChatId>>(
        chat_id: C,
        message_id: Integer,
        latitude: Float,
        longitude: Float,
    ) -> Self {
        EditMessageLiveLocation {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            latitude,
            longitude,
            reply_markup: None,
        }
    }

    /// Creates a new EditMessageLiveLocation
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    /// * latitude - Latitude of new location
    /// * longitude Longitude of new location
    pub fn with_inline_message_id<S: Into<String>>(
        inline_message_id: S,
        latitude: Float,
        longitude: Float,
    ) -> Self {
        EditMessageLiveLocation {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            latitude,
            longitude,
            reply_markup: None,
        }
    }

    /// New inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(&mut self, reply_markup: I) -> &mut Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for EditMessageLiveLocation {
    type Response = EditMessageResult;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("editMessageLiveLocation", &self)
    }
}

/// Stop updating a live location message
/// sent by the bot or via the bot (for inline bots)
/// before live_period expires
#[derive(Clone, Debug, Serialize)]
pub struct StopMessageLiveLocation {
    chat_id: Option<ChatId>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl StopMessageLiveLocation {
    /// Creates a new StopMessageLiveLocation
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer) -> Self {
        StopMessageLiveLocation {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            reply_markup: None,
        }
    }

    /// Creates a new StopMessageLiveLocation
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S) -> Self {
        StopMessageLiveLocation {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            reply_markup: None,
        }
    }

    /// New inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(&mut self, reply_markup: I) -> &mut Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for StopMessageLiveLocation {
    type Response = EditMessageResult;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("stopMessageLiveLocation", &self)
    }
}
