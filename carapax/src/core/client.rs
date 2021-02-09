use crate::methods::SendMessage;
use crate::types::{Integer, Message, ParseMode, ReplyMarkup, TextEntity};
use crate::{Api, ExecuteError, FromUpdate, ServiceUpdate};
use std::convert::Infallible;
use std::fmt;
use std::ops::Deref;

#[derive(Clone)]
pub struct Client {
    api: Api,
    message: Message,
}

impl Client {
    pub fn message(&self) -> &Message {
        &self.message
    }

    pub fn send_message<T: Into<String>>(&self, message: T) -> ClientSendMessage {
        ClientSendMessage {
            client: self,
            send_message: SendMessage::new(self.message.get_chat_id(), message),
        }
    }
}

// TODO: derive Debug when Api implement it too
impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Client")
            .field("api", &format_args!("Api"))
            .field("message", &self.message)
            .finish()
    }
}

impl Deref for Client {
    type Target = Api;

    fn deref(&self) -> &Self::Target {
        &self.api
    }
}

impl FromUpdate for Client {
    type Error = Infallible;

    fn from_update(service_update: ServiceUpdate) -> Result<Option<Self>, Self::Error> {
        let message: Message = FromUpdate::from_update(service_update.clone())?.unwrap();
        let api = FromUpdate::from_update(service_update)?.unwrap();
        Ok(Some(Self { api, message }))
    }
}

pub struct ClientSendMessage<'a> {
    client: &'a Client,
    send_message: SendMessage,
}

impl ClientSendMessage<'_> {
    /// Sets parse mode
    ///
    /// Entities will be set to None when this method is called
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.send_message = self.send_message.parse_mode(parse_mode);
        self
    }

    /// List of special entities that appear in message text
    ///
    /// Parse mode will be set to None when this method is called
    pub fn entities(mut self, entities: Vec<TextEntity>) -> Self {
        self.send_message = self.send_message.entities(entities);
        self
    }

    /// Disables link previews for links in this message
    pub fn disable_web_page_preview(mut self, disable_web_page_preview: bool) -> Self {
        self.send_message = self.send_message.disable_web_page_preview(disable_web_page_preview);
        self
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.send_message = self.send_message.disable_notification(disable_notification);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.send_message = self.send_message.reply_to_message_id(reply_to_message_id);
        self
    }

    /// Pass True, if the message should be sent even
    /// if the specified replied-to message is not found
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.send_message = self
            .send_message
            .allow_sending_without_reply(allow_sending_without_reply);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(mut self, reply_markup: R) -> Self {
        self.send_message = self.send_message.reply_markup(reply_markup);
        self
    }
}

impl ClientSendMessage<'_> {
    pub fn reply_to_user(self) -> Self {
        let message_id = self.client.message.id;
        self.reply_to_message_id(message_id)
    }

    pub async fn execute(self) -> Result<Message, ExecuteError> {
        self.client.api.execute(self.send_message).await
    }
}
