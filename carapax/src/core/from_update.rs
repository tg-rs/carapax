use crate::core::dispatcher::DispatcherData;
use crate::types::{
    CallbackQuery, ChosenInlineResult, Command as RawCommand, CommandError as RawCommandError, InlineQuery, Message,
    Poll, PollAnswer, PreCheckoutQuery, ShippingQuery, Update, UpdateKind,
};
use crate::Api;
use std::convert::{Infallible, TryFrom};
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServiceUpdate {
    pub update: Update,
    pub api: Api,
    pub data: Arc<DispatcherData>,
}

/// Allows to create an input for a handler from given update
pub trait FromUpdate: Sized {
    /// An error when converting update
    type Error: fmt::Debug + Send + Sync;

    /// Returns a handler input
    ///
    /// Handler will not run if None or Error returned
    fn from_update(service_update: ServiceUpdate) -> Result<Option<Self>, Self::Error>;
}

impl FromUpdate for Update {
    type Error = Infallible;

    fn from_update(service_update: ServiceUpdate) -> Result<Option<Self>, Self::Error> {
        Ok(Some(service_update.update))
    }
}

impl FromUpdate for Message {
    type Error = Infallible;

    fn from_update(service_update: ServiceUpdate) -> Result<Option<Self>, Self::Error> {
        Ok(match service_update.update.kind {
            UpdateKind::Message(msg)
            | UpdateKind::EditedMessage(msg)
            | UpdateKind::ChannelPost(msg)
            | UpdateKind::EditedChannelPost(msg) => Some(msg),
            _ => None,
        })
    }
}

impl FromUpdate for RawCommand {
    type Error = RawCommandError;

    fn from_update(service_update: ServiceUpdate) -> Result<Option<Self>, Self::Error> {
        // Should never panic as the error type is Infallible
        let message: Option<Message> =
            FromUpdate::from_update(service_update).expect("Could not convert update to message");
        if let Some(message) = message {
            match RawCommand::try_from(message) {
                Ok(command) => Ok(Some(command)),
                Err(RawCommandError::NotFound) => Ok(None),
                Err(err) => Err(err),
            }
        } else {
            Ok(None)
        }
    }
}

impl FromUpdate for InlineQuery {
    type Error = Infallible;

    fn from_update(service_update: ServiceUpdate) -> Result<Option<Self>, Self::Error> {
        Ok(match service_update.update.kind {
            UpdateKind::InlineQuery(query) => Some(query),
            _ => None,
        })
    }
}

impl FromUpdate for ChosenInlineResult {
    type Error = Infallible;

    fn from_update(service_update: ServiceUpdate) -> Result<Option<Self>, Self::Error> {
        Ok(match service_update.update.kind {
            UpdateKind::ChosenInlineResult(result) => Some(result),
            _ => None,
        })
    }
}

impl FromUpdate for CallbackQuery {
    type Error = Infallible;

    fn from_update(service_update: ServiceUpdate) -> Result<Option<Self>, Self::Error> {
        Ok(match service_update.update.kind {
            UpdateKind::CallbackQuery(query) => Some(query),
            _ => None,
        })
    }
}

impl FromUpdate for ShippingQuery {
    type Error = Infallible;

    fn from_update(service_update: ServiceUpdate) -> Result<Option<Self>, Self::Error> {
        Ok(match service_update.update.kind {
            UpdateKind::ShippingQuery(query) => Some(query),
            _ => None,
        })
    }
}

impl FromUpdate for PreCheckoutQuery {
    type Error = Infallible;

    fn from_update(service_update: ServiceUpdate) -> Result<Option<Self>, Self::Error> {
        Ok(match service_update.update.kind {
            UpdateKind::PreCheckoutQuery(query) => Some(query),
            _ => None,
        })
    }
}

impl FromUpdate for Poll {
    type Error = Infallible;

    fn from_update(service_update: ServiceUpdate) -> Result<Option<Self>, Self::Error> {
        Ok(match service_update.update.kind {
            UpdateKind::Poll(poll) => Some(poll),
            _ => None,
        })
    }
}

impl FromUpdate for PollAnswer {
    type Error = Infallible;

    fn from_update(service_update: ServiceUpdate) -> Result<Option<Self>, Self::Error> {
        Ok(match service_update.update.kind {
            UpdateKind::PollAnswer(poll_answer) => Some(poll_answer),
            _ => None,
        })
    }
}

impl FromUpdate for Api {
    type Error = Infallible;

    fn from_update(service_update: ServiceUpdate) -> Result<Option<Self>, Self::Error> {
        Ok(Some(service_update.api))
    }
}

macro_rules! impl_from_update_for_tuple {
    ($($T:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($T),+> FromUpdate for ($($T,)+)
        where
            $(
                $T: FromUpdate,
                $T::Error: fmt::Display + 'static,
            )+
        {
            // FIXME: The size for values of type `(dyn std::error::Error + 'static)` cannot be known at compilation time
            type Error = anyhow::Error;

            fn from_update(service_update: ServiceUpdate) -> Result<Option<Self>, Self::Error> {
                $( let $T: Option<$T> = FromUpdate::from_update(service_update.clone()).map_err(anyhow::Error::msg)?; )+

                // same as
                // ```
                // A.zip(B).zip(C) ...
                // ```
                // which produce `((A, B), C)`
                // but with 1 nesting level of tuples - `(A, B, C)`
                let f = move || {
                    Some(($($T?,)+))
                };

                Ok(f())
            }
        }
    };
}

impl_from_update_for_tuple!(A);
impl_from_update_for_tuple!(A, B);
impl_from_update_for_tuple!(A, B, C);

pub struct Data<T>(Arc<T>);

impl<T> Data<T> {
    pub fn from_arc(data: Arc<T>) -> Self {
        Self(data)
    }
}

impl<T> Clone for Data<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<T> for Data<T> {
    fn from(t: T) -> Self {
        Self(Arc::new(t))
    }
}

impl<T: 'static> FromUpdate for Data<T> {
    type Error = DataError;

    fn from_update(service_update: ServiceUpdate) -> Result<Option<Self>, Self::Error> {
        let data = service_update.data.get::<Self>().cloned().ok_or(DataError)?;
        Ok(Some(data))
    }
}

impl<T> Deref for Data<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<T> fmt::Debug for Data<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct DataError;

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Data is not available")
    }
}

impl std::error::Error for DataError {}

#[derive(Debug)]
pub struct Command<T>(pub T);

impl<T> FromUpdate for Command<T>
where
    T: CommandMeta,
    T::Error: fmt::Debug + Send + Sync,
{
    type Error = CommandError<T::Error>;

    fn from_update(service_update: ServiceUpdate) -> Result<Option<Self>, Self::Error> {
        Ok(RawCommand::from_update(service_update)?
            .filter(|command| command.get_name() == T::NAME)
            .map(T::try_from)
            .transpose()
            .map_err(CommandError::Meta)?
            .map(Command))
    }
}

pub trait CommandMeta: TryFrom<RawCommand> {
    const NAME: &'static str;
}

#[derive(Debug)]
pub enum CommandError<T> {
    Meta(T),
    Raw(RawCommandError),
}

impl<T> From<RawCommandError> for CommandError<T> {
    fn from(err: RawCommandError) -> Self {
        Self::Raw(err)
    }
}

impl<T> fmt::Display for CommandError<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandError::Meta(meta) => meta.fmt(f),
            CommandError::Raw(err) => err.fmt(f),
        }
    }
}

impl<T: fmt::Debug + fmt::Display> std::error::Error for CommandError<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message() {
        for data in vec![
            serde_json::json!({
                "update_id": 1,
                "message": {
                    "message_id": 1111,
                    "date": 0,
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "private", "first_name": "test"},
                    "text": "test message from private chat"
                }
            }),
            serde_json::json!({
                "update_id": 1,
                "edited_message": {
                    "message_id": 1111,
                    "date": 0,
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "private", "first_name": "test"},
                    "text": "test edited message from private chat",
                    "edit_date": 1213
                }
            }),
            serde_json::json!({
                "update_id": 1,
                "channel_post": {
                    "message_id": 1111,
                    "date": 0,
                    "author_signature": "test",
                    "chat": {"id": 1, "type": "channel", "title": "channeltitle", "username": "channelusername"},
                    "text": "test message from channel"
                }
            }),
            serde_json::json!({
                "update_id": 1,
                "edited_channel_post": {
                    "message_id": 1111,
                    "date": 0,
                    "chat": {"id": 1, "type": "channel", "title": "channeltitle", "username": "channelusername"},
                    "text": "test edited message from channel",
                    "edit_date": 1213
                }
            }),
        ] {
            let update: Update = serde_json::from_value(data).unwrap();
            assert!(Update::from_update(update.clone()).unwrap().is_some());
            assert!(Message::from_update(update).unwrap().is_some());
        }
    }

    #[test]
    fn command() {
        let update: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "message": {
                    "message_id": 1111,
                    "date": 0,
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "chat": {"id": 1, "type": "private", "first_name": "test"},
                    "text": "/test",
                    "entities": [
                        {"type": "bot_command", "offset": 0, "length": 5}
                    ]
                }
            }
        ))
        .unwrap();
        assert!(Update::from_update(update.clone()).unwrap().is_some());
        assert!(Command::from_update(update).unwrap().is_some());
    }

    #[test]
    fn inline_query() {
        let update: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "inline_query": {
                    "id": "id",
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "query": "query",
                    "offset": "offset"
                }
            }
        ))
        .unwrap();
        assert!(Update::from_update(update.clone()).unwrap().is_some());
        assert!(InlineQuery::from_update(update).unwrap().is_some());
    }

    #[test]
    fn chosen_inline_result() {
        let update: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "chosen_inline_result": {
                    "result_id": "id",
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "query": "query"
                }
            }
        ))
        .unwrap();
        assert!(Update::from_update(update.clone()).unwrap().is_some());
        assert!(ChosenInlineResult::from_update(update).unwrap().is_some());
    }

    #[test]
    fn callback_query() {
        let update: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "callback_query": {
                    "id": "id",
                    "from": {"id": 1, "is_bot": false, "first_name": "test"}
                }
            }
        ))
        .unwrap();
        assert!(Update::from_update(update.clone()).unwrap().is_some());
        assert!(CallbackQuery::from_update(update).unwrap().is_some());
    }

    #[test]
    fn shipping_query() {
        let update: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "shipping_query": {
                    "id": "id",
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "invoice_payload": "payload",
                    "shipping_address": {
                        "country_code": "RU",
                        "state": "State",
                        "city": "City",
                        "street_line1": "Line 1",
                        "street_line2": "Line 2",
                        "post_code": "Post Code"
                    }
                }
            }
        ))
        .unwrap();
        assert!(Update::from_update(update.clone()).unwrap().is_some());
        assert!(ShippingQuery::from_update(update).unwrap().is_some());
    }

    #[test]
    fn pre_checkout_query() {
        let update: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "pre_checkout_query": {
                    "id": "id",
                    "from": {"id": 1, "is_bot": false, "first_name": "test"},
                    "currency": "RUB",
                    "total_amount": 145,
                    "invoice_payload": "payload"
                }
            }
        ))
        .unwrap();
        assert!(Update::from_update(update.clone()).unwrap().is_some());
        assert!(PreCheckoutQuery::from_update(update).unwrap().is_some());
    }

    #[test]
    fn poll() {
        let update: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "poll": {
                    "id": "id",
                    "question": "test poll",
                    "options": [
                        {"text": "opt 1", "voter_count": 1},
                        {"text": "opt 2", "voter_count": 2}
                    ],
                    "is_closed": false,
                    "total_voter_count": 3,
                    "is_anonymous": true,
                    "type": "regular",
                    "allows_multiple_answers": false
                }
            }
        ))
        .unwrap();
        assert!(Update::from_update(update.clone()).unwrap().is_some());
        assert!(Poll::from_update(update).unwrap().is_some());
    }

    #[test]
    fn poll_answer() {
        let update: Update = serde_json::from_value(serde_json::json!(
            {
                "update_id": 1,
                "poll_answer": {
                    "poll_id": "poll-id",
                    "user": {
                        "id": 1,
                        "first_name": "Jamie",
                        "is_bot": false
                    },
                    "option_ids": [0],
                }
            }
        ))
        .unwrap();
        assert!(Update::from_update(update.clone()).unwrap().is_some());
        assert!(PollAnswer::from_update(update).unwrap().is_some());
    }
}
