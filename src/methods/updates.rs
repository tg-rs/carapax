use crate::{
    methods::method::*,
    types::{AllowedUpdate, Integer, Update, WebhookInfo},
};
use failure::Error;
use serde::Serialize;
use std::collections::HashSet;

/// Receive incoming updates using long polling
///
/// An Array of Update objects is returned
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<HashSet<AllowedUpdate>>,
}

impl Method for GetUpdates {
    type Response = Vec<Update>;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("getUpdates", &self)
    }
}

impl GetUpdates {
    /// Identifier of the first update to be returned
    ///
    /// Must be greater by one than the highest among the identifiers of previously received updates
    /// By default, updates starting with the earliest unconfirmed update are returned
    /// An update is considered confirmed as soon as getUpdates is called with an offset higher than its update_id
    /// The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue
    /// All previous updates will forgotten
    pub fn offset(mut self, offset: Integer) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Limits the number of updates to be retrieved
    ///
    /// Values between 1—100 are accepted
    /// Defaults to 100
    pub fn limit(mut self, limit: Integer) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Timeout in seconds for long polling
    ///
    /// Defaults to 0, i.e. usual short polling
    /// Should be positive, short polling should be used for testing purposes only
    pub fn timeout(mut self, timeout: Integer) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// List the types of updates you want your bot to receive
    ///
    /// For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types
    /// Specify an empty list to receive all updates regardless of type (default)
    /// If not specified, the previous setting will be used
    /// Please note that this parameter doesn't affect updates created before the call to the getUpdates,
    /// so unwanted updates may be received for a short period of time
    pub fn allowed_updates(mut self, allowed_updates: HashSet<AllowedUpdate>) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }

    /// Adds a type of updates you want your bot to receive
    pub fn add_allowed_update(mut self, allowed_update: AllowedUpdate) -> Self {
        match self.allowed_updates {
            Some(ref mut updates) => {
                updates.insert(allowed_update);
            }
            None => {
                let mut updates = HashSet::new();
                updates.insert(allowed_update);
                self.allowed_updates = Some(updates);
            }
        };
        self
    }
}

/// Specify a url and receive incoming updates via an outgoing webhook
///
/// Whenever there is an update for the bot, we will send an HTTPS POST request
/// to the specified url, containing a JSON-serialized Update
/// In case of an unsuccessful request, we will give up after a reasonable amount of attempts
///
/// If you'd like to make sure that the Webhook request comes from Telegram,
/// we recommend using a secret path in the URL, e.g. https://www.example.com/<token>
/// Since nobody else knows your bot‘s token, you can be pretty sure it’s us
#[derive(Clone, Debug, Serialize)]
pub struct SetWebhook {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<HashSet<AllowedUpdate>>,
}

impl SetWebhook {
    /// Creates a new SetWebhook with given url
    ///
    /// # Arguments
    ///
    /// * url - HTTPS url to send updates to
    ///         Use an empty string to remove webhook integration
    pub fn new<S: Into<String>>(url: S) -> Self {
        SetWebhook {
            url: url.into(),
            certificate: None,
            max_connections: None,
            allowed_updates: None,
        }
    }

    /// Upload your public key certificate so that the root certificate in use can be checked
    pub fn certificate(mut self, certificate: String) -> Self {
        self.certificate = Some(certificate);
        self
    }

    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100
    ///
    /// Defaults to 40
    /// Use lower values to limit the load on your bot‘s server, and higher values to increase your bot’s throughput
    pub fn max_connections(mut self, max_connections: Integer) -> Self {
        self.max_connections = Some(max_connections);
        self
    }

    /// List the types of updates you want your bot to receive
    ///
    /// For example, specify [“message”, “edited_channel_post”, “callback_query”]
    /// to only receive updates of these types
    /// See Update for a complete list of available update types
    /// Specify an empty list to receive all updates regardless of type (default)
    /// If not specified, the previous setting will be used
    /// Please note that this parameter doesn't affect updates created before the call to the setWebhook,
    /// so unwanted updates may be received for a short period of time
    pub fn allowed_updates(mut self, allowed_updates: HashSet<AllowedUpdate>) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }

    /// Adds a type of updates you want your bot to receive
    pub fn add_allowed_update(mut self, allowed_update: AllowedUpdate) -> Self {
        match self.allowed_updates {
            Some(ref mut updates) => {
                updates.insert(allowed_update);
            }
            None => {
                let mut updates = HashSet::new();
                updates.insert(allowed_update);
                self.allowed_updates = Some(updates);
            }
        };
        self
    }
}

impl Method for SetWebhook {
    type Response = bool;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("setWebhook", &self)
    }
}

/// Remove webhook integration if you decide to switch back to getUpdates
///
/// Returns True on success
#[derive(Clone, Copy, Debug)]
pub struct DeleteWebhook;

impl Method for DeleteWebhook {
    type Response = bool;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::empty("deleteWebhook")
    }
}

/// Get current webhook status
#[derive(Clone, Copy, Debug)]
pub struct GetWebhookInfo;

impl Method for GetWebhookInfo {
    type Response = WebhookInfo;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::empty("getWebhookInfo")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_serialize_get_updates() {
        let req = GetUpdates::default().get_request().unwrap().build("token");
        assert_eq!(req.method, RequestMethod::Post);
        assert_eq!(req.url, String::from("https://api.telegram.org/bottoken/getUpdates"));
        match req.body {
            RequestBody::Json(data) => {
                assert_eq!(String::from_utf8(data).unwrap(), String::from(r#"{}"#));
            }
            data => panic!("Unexpected request data: {:?}", data),
        }

        let mut updates = HashSet::new();
        updates.insert(AllowedUpdate::Message);
        updates.insert(AllowedUpdate::Message);
        updates.insert(AllowedUpdate::EditedMessage);
        updates.insert(AllowedUpdate::ChannelPost);
        updates.insert(AllowedUpdate::EditedChannelPost);
        updates.insert(AllowedUpdate::ChosenInlineResult);
        let req = GetUpdates::default()
            .offset(0)
            .limit(10)
            .allowed_updates(updates)
            .add_allowed_update(AllowedUpdate::InlineQuery)
            .add_allowed_update(AllowedUpdate::CallbackQuery)
            .add_allowed_update(AllowedUpdate::PreCheckoutQuery)
            .add_allowed_update(AllowedUpdate::ShippingQuery)
            .get_request()
            .unwrap()
            .build("token");
        match req.body {
            RequestBody::Json(data) => {
                let data: Value = serde_json::from_slice(&data).unwrap();;
                assert_eq!(data["offset"], 0);
                assert_eq!(data["limit"], 10);
                let mut updates: Vec<&str> = data["allowed_updates"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_str().unwrap())
                    .collect();
                updates.sort();
                assert_eq!(
                    updates,
                    vec![
                        "callback_query",
                        "channel_post",
                        "chosen_inline_result",
                        "edited_channel_post",
                        "edited_message",
                        "inline_query",
                        "message",
                        "pre_checkout_query",
                        "shipping_query",
                    ]
                );
            }
            data => panic!("Unexpected request data: {:?}", data),
        }
    }

    #[test]
    fn test_serialize_set_webhook() {
        let req = SetWebhook::new("url").get_request().unwrap().build("token");
        assert_eq!(req.method, RequestMethod::Post);
        assert_eq!(req.url, String::from("https://api.telegram.org/bottoken/setWebhook"));
        match req.body {
            RequestBody::Json(data) => {
                assert_eq!(String::from_utf8(data).unwrap(), String::from(r#"{"url":"url"}"#));
            }
            data => panic!("Unexpected request data: {:?}", data),
        }
    }

    #[test]
    fn test_serialize_delete_webhook() {
        let req = DeleteWebhook.get_request().unwrap().build("token");
        assert_eq!(req.method, RequestMethod::Get);
        assert_eq!(req.url, String::from("https://api.telegram.org/bottoken/deleteWebhook"));
        match req.body {
            RequestBody::Empty => {}
            data => panic!("Unexpected request data: {:?}", data),
        }
    }

    #[test]
    fn test_serialize_get_webhook_info() {
        let req = GetWebhookInfo.get_request().unwrap().build("token");
        assert_eq!(req.method, RequestMethod::Get);
        assert_eq!(
            req.url,
            String::from("https://api.telegram.org/bottoken/getWebhookInfo")
        );
        match req.body {
            RequestBody::Empty => {}
            data => panic!("Unexpected request data: {:?}", data),
        }
    }
}
