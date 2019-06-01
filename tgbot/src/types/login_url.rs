use serde::{Deserialize, Serialize};

/// Represents a parameter of the inline keyboard button used to automatically authorize a user
///
/// Serves as a great replacement for the Telegram Login Widget when the user is coming from Telegram
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginUrl {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bot_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_write_access: Option<bool>,
}

impl LoginUrl {
    /// Creates a new LoginUrl with given URL
    ///
    /// An HTTP URL will be opened with user authorization data added to the query string when the button is pressed
    ///
    /// If the user refuses to provide authorization data, the original URL without information about the user will be opened
    ///
    /// The data added is the same as described in [Receiving authorization data][1]
    ///
    /// NOTE: You **must** always check the hash of the received data to verify the authentication
    /// and the integrity of the data as described in [Checking authorization][2]
    ///
    /// [1]: https://core.telegram.org/widgets/login#receiving-authorization-data
    /// [2]: https://core.telegram.org/widgets/login#checking-authorization
    pub fn new<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            url: url.into(),
            forward_text: None,
            bot_username: None,
            request_write_access: None,
        }
    }

    /// New text of the button in forwarded messages
    pub fn forward_text<S>(mut self, forward_text: S) -> Self
    where
        S: Into<String>,
    {
        self.forward_text = Some(forward_text.into());
        self
    }

    /// Username of a bot, which will be used for user authorization
    ///
    /// See [Setting up a bot][1] for more details
    ///
    /// If not specified, the current bot's username will be assumed
    ///
    /// The url's domain must be the same as the domain linked with the bot
    ///
    /// See [Linking your domain to the bot][2] for more details
    ///
    /// [1]: https://core.telegram.org/widgets/login#setting-up-a-bot
    /// [2]: https://core.telegram.org/widgets/login#linking-your-domain-to-the-bot
    pub fn bot_username<S>(mut self, bot_username: S) -> Self
    where
        S: Into<String>,
    {
        self.bot_username = Some(bot_username.into());
        self
    }

    /// Pass True to request the permission for your bot to send messages to the user
    pub fn request_write_access(mut self, request_write_access: bool) -> Self {
        self.request_write_access = Some(request_write_access);
        self
    }
}

impl<S> From<S> for LoginUrl
where
    S: Into<String>,
{
    fn from(url: S) -> Self {
        Self::new(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let mut url = LoginUrl::from("url");

        let data = serde_json::to_value(&url).unwrap();
        assert_eq!(data, serde_json::json!({"url": "url"}));

        url = url.forward_text("forward text");
        let data = serde_json::to_value(&url).unwrap();
        assert_eq!(data, serde_json::json!({"url": "url", "forward_text": "forward text"}));

        url = url.bot_username("botusername");
        let data = serde_json::to_value(&url).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "url": "url",
                "forward_text": "forward text",
                "bot_username": "botusername"
            })
        );

        url = url.request_write_access(true);
        let data = serde_json::to_value(&url).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "url": "url",
                "forward_text": "forward text",
                "bot_username": "botusername",
                "request_write_access": true
            })
        );
    }
}
