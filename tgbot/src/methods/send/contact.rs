use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Integer, Message, ReplyMarkup},
};
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
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, phone_number: S, first_name: S) -> Self {
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
    pub fn last_name<S: Into<String>>(mut self, last_name: S) -> Self {
        self.last_name = Some(last_name.into());
        self
    }

    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub fn vcard<S: Into<String>>(mut self, vcard: S) -> Self {
        self.vcard = Some(vcard.into());
        self
    }

    /// Sends the message silently
    ///
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

impl Method for SendContact {
    type Response = Message;

    fn into_request(self) -> Request {
        Request::json("sendContact", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::ForceReply,
    };
    use serde_json::Value;

    #[test]
    fn send_contact() {
        let request = SendContact::new(1, "phone", "first name")
            .last_name("last name")
            .vcard("vcard")
            .disable_notification(true)
            .reply_to_message_id(1)
            .reply_markup(ForceReply::new(true))
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendContact");
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["phone_number"], "phone");
            assert_eq!(data["first_name"], "first name");
            assert_eq!(data["last_name"], "last name");
            assert_eq!(data["vcard"], "vcard");
            assert_eq!(data["disable_notification"], true);
            assert_eq!(data["reply_to_message_id"], 1);
            assert_eq!(data["reply_markup"]["force_reply"], true);
        } else {
            panic!("Unexpected request body");
        }
    }
}
