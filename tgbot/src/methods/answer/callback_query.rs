use crate::{methods::Method, request::Request, types::Integer};
use serde::Serialize;

/// Send answer to callback query sent from inline keyboard
///
/// The answer will be displayed to the user as a notification at the top of the chat screen or as an alert
/// Alternatively, the user can be redirected to the specified Game URL
/// For this option to work, you must first create a game for your bot via @Botfather and accept the terms
/// Otherwise, you may use links like t.me/your_bot?start=XXXX that open your bot with a parameter
#[derive(Clone, Debug, Serialize)]
pub struct AnswerCallbackQuery {
    callback_query_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<Integer>,
}

impl AnswerCallbackQuery {
    /// Creates a new AnswerCallbackQuery with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * callback_query_id - Unique identifier for the query to be answered
    pub fn new<S: Into<String>>(callback_query_id: S) -> Self {
        AnswerCallbackQuery {
            callback_query_id: callback_query_id.into(),
            text: None,
            show_alert: None,
            url: None,
            cache_time: None,
        }
    }

    /// Text of the notification
    ///
    /// If not specified, nothing will be shown to the user, 0-200 characters
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    /// An alert will be shown by the client instead of a notification at the top of the chat screen
    ///
    /// Defaults to false
    pub fn show_alert(mut self, show_alert: bool) -> Self {
        self.show_alert = Some(show_alert);
        self
    }

    /// URL that will be opened by the user's client
    ///
    /// If you have created a Game and accepted the conditions via @Botfather,
    /// specify the URL that opens your game – note that this will only work
    /// if the query comes from a callback_game button
    ///
    /// Otherwise, you may use links like t.me/your_bot?start=XXXX that open your bot with a parameter
    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        self.url = Some(url.into());
        self
    }

    /// The maximum amount of time in seconds that the result of the callback query may be cached client-side
    ///
    /// Telegram apps will support caching starting in version 3.14
    /// Defaults to 0
    pub fn cache_time(mut self, cache_time: Integer) -> Self {
        self.cache_time = Some(cache_time);
        self
    }
}

impl Method for AnswerCallbackQuery {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("answerCallbackQuery", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn answer_callback_query() {
        let request = AnswerCallbackQuery::new("q-id")
            .text("text")
            .show_alert(true)
            .url("url")
            .cache_time(10)
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/answerCallbackQuery"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["callback_query_id"], "q-id");
            assert_eq!(data["text"], "text");
            assert_eq!(data["show_alert"], true);
            assert_eq!(data["url"], "url");
            assert_eq!(data["cache_time"], 10);
        } else {
            panic!("Unexpected request body");
        }
    }
}
