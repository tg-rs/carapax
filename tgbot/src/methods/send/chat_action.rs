use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatAction, ChatId},
};
use failure::Error;
use serde::Serialize;

/// Tell the user that something is happening on the bot's side
///
/// The status is set for 5 seconds or less
/// (when a message arrives from your bot, Telegram clients clear its typing status)
///
/// Example: The ImageBot needs some time to process a request and upload the image
/// Instead of sending a text message along the lines of “Retrieving image, please wait…”,
/// the bot may use sendChatAction with action = upload_photo
/// The user will see a “sending photo” status for the bot
/// We only recommend using this method when a response from the bot
/// will take a noticeable amount of time to arrive
#[derive(Clone, Debug, Serialize)]
pub struct SendChatAction {
    chat_id: ChatId,
    action: ChatAction,
}

impl SendChatAction {
    /// Creates a new SendChatAction
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identitifer for the target chat
    /// * action - Type of action to broadcast
    pub fn new<C: Into<ChatId>>(chat_id: C, action: ChatAction) -> Self {
        SendChatAction {
            chat_id: chat_id.into(),
            action,
        }
    }
}

impl Method for SendChatAction {
    type Response = bool;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("sendChatAction", &self)
    }
}
