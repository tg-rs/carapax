use crate::{
    methods::Method,
    request::Request,
    types::{ChatAction, ChatId},
};
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

    fn into_request(self) -> Request {
        Request::json("sendChatAction", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn send_chat_action() {
        let request = SendChatAction::new(1, ChatAction::Typing).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/sendChatAction"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["action"], "typing");
        } else {
            panic!("Unexpected request body");
        }
    }
}
