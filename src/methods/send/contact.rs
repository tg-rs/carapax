use crate::methods::method::*;
use crate::types::{ChatId, Integer, Message, ReplyMarkup};
use failure::Error;
use serde::Serialize;

/// Send phone contacts
#[derive(Clone, Debug, Serialize)]
pub struct SendContact {
    chat_id: ChatId,
    phone_number: String,
    first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl SendContact {
    /// Creates a new SendContact with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * phone_number - Contact's phone number
    /// * first_name - Contact's first name
    pub fn new<C: Into<ChatId>, S: Into<String>>(
        chat_id: C,
        phone_number: S,
        first_name: S,
    ) -> Self {
        SendContact {
            chat_id: chat_id.into(),
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Contact's last name
    pub fn last_name<S: Into<String>>(&mut self, last_name: S) -> &mut Self {
        self.last_name = Some(last_name.into());
        self
    }

    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub fn vcard<S: Into<String>>(&mut self, vcard: S) -> &mut Self {
        self.vcard = Some(vcard.into());
        self
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
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

impl Method for SendContact {
    type Response = Message;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("sendContact", &self)
    }
}
