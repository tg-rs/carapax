use crate::methods::method::*;
use crate::types::{ChatId, Integer};

/// Restrict a user in a supergroup
///
/// The bot must be an administrator in the supergroup
/// for this to work and must have the appropriate admin rights.
///
/// Pass True for all boolean parameters to lift restrictions from a user
#[derive(Clone, Debug, Serialize)]
pub struct RestrictChatMember {
    chat_id: ChatId,
    user_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_send_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_send_media_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_send_other_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_add_web_page_previews: Option<bool>,
}

impl RestrictChatMember {
    /// Creates a new RestrictChatMember with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
        RestrictChatMember {
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
            can_send_messages: None,
            can_send_media_messages: None,
            can_send_other_messages: None,
            can_add_web_page_previews: None,
        }
    }

    /// Restrict everything
    pub fn restrict_all(&mut self) -> &mut Self {
        self.can_send_messages = Some(false);
        self.can_send_media_messages = Some(false);
        self.can_send_other_messages = Some(false);
        self.can_add_web_page_previews = Some(false);
        self
    }

    /// Allow everything
    pub fn allow_all(&mut self) -> &mut Self {
        self.can_send_messages = Some(true);
        self.can_send_media_messages = Some(true);
        self.can_send_other_messages = Some(true);
        self.can_add_web_page_previews = Some(true);
        self
    }

    /// Date when restrictions will be lifted for the user, unix time
    ///
    /// If user is restricted for more than 366 days or less than 30 seconds
    /// from the current time, they are considered to be restricted forever
    pub fn until_date(&mut self, until_date: Integer) -> &mut Self {
        self.until_date = Some(until_date);
        self
    }

    /// Pass True, if the user can send text messages, contacts, locations and venues
    pub fn can_send_messages(&mut self, can_send_messages: bool) -> &mut Self {
        self.can_send_messages = Some(can_send_messages);
        self
    }

    /// Pass True, if the user can send audios, documents, photos,
    /// videos, video notes and voice notes, implies can_send_messages
    pub fn can_send_media_messages(&mut self, can_send_media_messages: bool) -> &mut Self {
        self.can_send_media_messages = Some(can_send_media_messages);
        self
    }

    /// Pass True, if the user can send animations, games, stickers and
    /// use inline bots, implies can_send_media_messages
    pub fn can_send_other_messages(&mut self, can_send_other_messages: bool) -> &mut Self {
        self.can_send_other_messages = Some(can_send_other_messages);
        self
    }

    /// Pass True, if the user may add web page previews to their messages,
    /// implies can_send_media_messages
    pub fn can_add_web_page_previews(&mut self, can_add_web_page_previews: bool) -> &mut Self {
        self.can_add_web_page_previews = Some(can_add_web_page_previews);
        self
    }
}

impl Method for RestrictChatMember {
    type Response = bool;

    fn get_request(&self) -> Result<Request, RequestError> {
        Ok(Request {
            method: RequestMethod::Post,
            url: RequestUrl::new("restrictChatMember"),
            body: RequestBody::json(&self)?,
        })
    }
}
