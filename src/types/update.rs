use crate::types::inline_mode::{ChosenInlineResult, InlineQuery};
use crate::types::keyboards::CallbackQuery;
use crate::types::message::Message;
use crate::types::payments::{PreCheckoutQuery, ShippingQuery};
use crate::types::primitive::Integer;

/// This object represents an incoming update.
/// At most one of the optional parameters can be present in any given update.
#[derive(Debug)]
pub struct Update {
    /// The update‘s unique identifier.
    /// Update identifiers start from a certain positive number and increase sequentially.
    /// This ID becomes especially handy if you’re using Webhooks,
    /// since it allows you to ignore repeated updates or to restore the correct update sequence,
    /// should they get out of order. If there are no new updates
    /// for at least a week, then identifier
    /// of the next update will be chosen randomly instead of sequentially.
    pub update_id: Integer,
    /// New incoming message of any kind — text, photo, sticker, etc.
    pub message: Option<Message>,
    /// New version of a message that is known to the bot and was edited
    pub edited_message: Option<Message>,
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    pub channel_post: Option<Message>,
    /// New version of a channel post that is known to the bot and was edited
    pub edited_channel_post: Option<Message>,
    /// New incoming inline query
    pub inline_query: Option<InlineQuery>,
    /// The result of an inline query that was chosen by a user and sent to their chat partner.
    /// Please see our documentation on the feedback collecting
    /// for details on how to enable these updates for your bot.
    pub chosen_inline_result: Option<ChosenInlineResult>,
    /// New incoming callback query
    pub callback_query: Option<CallbackQuery>,
    /// New incoming shipping query. Only for invoices with flexible price
    pub shipping_query: Option<ShippingQuery>,
    /// New incoming pre-checkout query. Contains full information about checkout
    pub pre_checkout_query: Option<PreCheckoutQuery>,
}
