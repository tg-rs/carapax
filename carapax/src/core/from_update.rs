use crate::{
    core::dispatcher::DispatcherData,
    types::{
        CallbackQuery, ChosenInlineResult, Command, CommandError, InlineQuery, Message, Poll, PollAnswer,
        PreCheckoutQuery, ShippingQuery, Update, UpdateKind,
    },
    Api,
};
use futures_util::future::{ok, ready, BoxFuture, Ready};
use std::{
    convert::{Infallible, TryFrom},
    fmt,
    future::Future,
    ops::Deref,
    sync::Arc,
};

/// An update with additional data to be used in [`FromUpdate`]
///
/// [`FromUpdate`]: FromUpdate
#[derive(Clone)]
#[allow(missing_docs)]
pub struct ServiceUpdate {
    pub update: Update,
    pub api: Api,
    pub data: Arc<DispatcherData>,
}

/// Allows to create an input for a handler from given update
pub trait FromUpdate: Sized {
    /// An error when converting update
    type Error: fmt::Debug + Send + Sync;

    #[allow(missing_docs)]
    type Future: Future<Output = Result<Option<Self>, Self::Error>>;

    /// Returns a handler input
    ///
    /// Handler will not run if None or Error returned
    fn from_update(service_update: ServiceUpdate) -> Self::Future;
}

impl FromUpdate for () {
    type Error = Infallible;
    type Future = Ready<Result<Option<Self>, Self::Error>>;

    fn from_update(_service_update: ServiceUpdate) -> Self::Future {
        ok(Some(()))
    }
}

impl FromUpdate for Update {
    type Error = Infallible;
    type Future = Ready<Result<Option<Self>, Self::Error>>;

    fn from_update(service_update: ServiceUpdate) -> Self::Future {
        ok(Some(service_update.update))
    }
}

impl FromUpdate for Message {
    type Error = Infallible;
    type Future = Ready<Result<Option<Self>, Self::Error>>;

    fn from_update(service_update: ServiceUpdate) -> Self::Future {
        ok(match service_update.update.kind {
            UpdateKind::Message(msg)
            | UpdateKind::EditedMessage(msg)
            | UpdateKind::ChannelPost(msg)
            | UpdateKind::EditedChannelPost(msg) => Some(msg),
            _ => None,
        })
    }
}

impl FromUpdate for Command {
    type Error = CommandError;
    type Future = BoxFuture<'static, Result<Option<Self>, Self::Error>>;

    fn from_update(service_update: ServiceUpdate) -> Self::Future {
        Box::pin(async move {
            // Should never panic as the error type is Infallible
            let message = Message::from_update(service_update)
                .await
                .expect("Could not convert update to message");
            if let Some(message) = message {
                match Command::try_from(message) {
                    Ok(command) => Ok(Some(command)),
                    Err(CommandError::NotFound) => Ok(None),
                    Err(err) => Err(err),
                }
            } else {
                Ok(None)
            }
        })
    }
}

impl FromUpdate for InlineQuery {
    type Error = Infallible;
    type Future = Ready<Result<Option<Self>, Self::Error>>;

    fn from_update(service_update: ServiceUpdate) -> Self::Future {
        ok(match service_update.update.kind {
            UpdateKind::InlineQuery(query) => Some(query),
            _ => None,
        })
    }
}

impl FromUpdate for ChosenInlineResult {
    type Error = Infallible;
    type Future = Ready<Result<Option<Self>, Self::Error>>;

    fn from_update(service_update: ServiceUpdate) -> Self::Future {
        ok(match service_update.update.kind {
            UpdateKind::ChosenInlineResult(result) => Some(result),
            _ => None,
        })
    }
}

impl FromUpdate for CallbackQuery {
    type Error = Infallible;
    type Future = Ready<Result<Option<Self>, Self::Error>>;

    fn from_update(service_update: ServiceUpdate) -> Self::Future {
        ok(match service_update.update.kind {
            UpdateKind::CallbackQuery(query) => Some(query),
            _ => None,
        })
    }
}

impl FromUpdate for ShippingQuery {
    type Error = Infallible;
    type Future = Ready<Result<Option<Self>, Self::Error>>;

    fn from_update(service_update: ServiceUpdate) -> Self::Future {
        ok(match service_update.update.kind {
            UpdateKind::ShippingQuery(query) => Some(query),
            _ => None,
        })
    }
}

impl FromUpdate for PreCheckoutQuery {
    type Error = Infallible;
    type Future = Ready<Result<Option<Self>, Self::Error>>;

    fn from_update(service_update: ServiceUpdate) -> Self::Future {
        ok(match service_update.update.kind {
            UpdateKind::PreCheckoutQuery(query) => Some(query),
            _ => None,
        })
    }
}

impl FromUpdate for Poll {
    type Error = Infallible;
    type Future = Ready<Result<Option<Self>, Self::Error>>;

    fn from_update(service_update: ServiceUpdate) -> Self::Future {
        ok(match service_update.update.kind {
            UpdateKind::Poll(poll) => Some(poll),
            _ => None,
        })
    }
}

impl FromUpdate for PollAnswer {
    type Error = Infallible;
    type Future = Ready<Result<Option<Self>, Self::Error>>;

    fn from_update(service_update: ServiceUpdate) -> Self::Future {
        ok(match service_update.update.kind {
            UpdateKind::PollAnswer(poll_answer) => Some(poll_answer),
            _ => None,
        })
    }
}

impl FromUpdate for Api {
    type Error = Infallible;
    type Future = Ready<Result<Option<Self>, Self::Error>>;

    fn from_update(service_update: ServiceUpdate) -> Self::Future {
        ok(Some(service_update.api))
    }
}

// actually it exists to avoid strange error:
// the size for values of type `(dyn std::error::Error + Send + Sync + 'static)` cannot be known at compilation time
// when using boxed error in FromUpdate implementation with tuples like `((Api,), (Message,))`
#[doc(hidden)]
pub struct TupleError(Box<dyn std::error::Error + Send + Sync>);

impl TupleError {
    fn from_err<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        Self(Box::new(err))
    }
}

impl std::error::Error for TupleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl fmt::Debug for TupleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for TupleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

macro_rules! impl_from_update_for_tuple {
    ($($T:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($T),+> FromUpdate for ($($T,)+)
        where
            $(
                $T: FromUpdate + Send,
                $T::Error: std::error::Error + Send + Sync + 'static,
                $T::Future: Send,
            )+
        {
            type Error = TupleError;
            type Future = BoxFuture<'static, Result<Option<Self>, Self::Error>>;

            fn from_update(service_update: ServiceUpdate) -> Self::Future {
                Box::pin(async move {
                    $( let $T = <$T>::from_update(service_update.clone()).await.map_err(TupleError::from_err)?; )+

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
                })
            }
        }
    };
}

impl_from_update_for_tuple!(A);
impl_from_update_for_tuple!(A, B);
impl_from_update_for_tuple!(A, B, C);
impl_from_update_for_tuple!(A, B, C, D);
impl_from_update_for_tuple!(A, B, C, D, E);
impl_from_update_for_tuple!(A, B, C, D, E, F);
impl_from_update_for_tuple!(A, B, C, D, E, F, G);
impl_from_update_for_tuple!(A, B, C, D, E, F, G, H);
impl_from_update_for_tuple!(A, B, C, D, E, F, G, H, I); // 9 arguments

/// A user data wrapped in [`Arc`] that can be used in handler
///
/// Data is added in [`Dispatcher::data`]
///
/// ```rust
/// use carapax::Data;
///
/// struct State {
///     a: i32,
/// }
///
/// async fn handler(my_data: Data<State>) {
///     log::info!("a is {}", my_data.a);
/// }
/// ```
///
/// [`Dispatcher::data`]: crate::Dispatcher::data
#[derive(Debug)]
pub struct Data<T>(Arc<T>);

impl<T> Data<T> {
    /// Creates Data with value that already wrapped in [`Arc`]
    /// because [`Data::from`](Data#impl-From<T>)
    pub fn from_arc(arc: Arc<T>) -> Self {
        Self(arc)
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
    type Future = Ready<Result<Option<Self>, Self::Error>>;

    fn from_update(service_update: ServiceUpdate) -> Self::Future {
        ready(service_update.data.get::<Self>().cloned().ok_or(DataError).map(Some))
    }
}

impl<T> Deref for Data<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

/// This error is returned when user requested data, but it's not available
#[derive(Debug)]
pub struct DataError;

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Data is not available")
    }
}

impl std::error::Error for DataError {}

/// Can be used to either get one or another function argument that implements [`FromUpdate`]
///
/// because [`FromUpdate::from_update`] can return `None` then handler is skipped
///
/// ```rust
/// use carapax::Either;
/// use carapax::types::{Poll, Message};
///
/// async fn handler(poll_or_msg: Either<Poll, Message>) {
///     match poll_or_msg {
///         Either::Left(poll) => log::info!("Poll received: {:?}", poll),
///         Either::Right(msg) => log::info!("Message received: {:?}", msg),
///     }
/// }
/// ```
pub enum Either<A, B> {
    #[allow(missing_docs)]
    Left(A),
    #[allow(missing_docs)]
    Right(B),
}

impl<A, B> FromUpdate for Either<A, B>
where
    A: FromUpdate + Send,
    A::Error: std::error::Error + Send + Sync + 'static,
    A::Future: Send,
    B: FromUpdate + Send,
    B::Error: std::error::Error + Send + Sync + 'static,
    B::Future: Send,
{
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = BoxFuture<'static, Result<Option<Self>, Self::Error>>;

    fn from_update(service_update: ServiceUpdate) -> Self::Future {
        Box::pin(async move {
            let a = A::from_update(service_update.clone()).await?;
            if let Some(a) = a {
                return Ok(Some(Either::Left(a)));
            }

            let b = B::from_update(service_update).await?;
            if let Some(b) = b {
                return Ok(Some(Either::Right(b)));
            }

            Ok(None)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_service_update(update: Update) -> ServiceUpdate {
        ServiceUpdate {
            update,
            api: Api::new("123").unwrap(),
            data: Arc::new(DispatcherData::default()),
        }
    }

    #[tokio::test]
    async fn message() {
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
            let update = create_service_update(update);
            assert!(Update::from_update(update.clone()).await.unwrap().is_some());
            assert!(Message::from_update(update).await.unwrap().is_some());
        }
    }

    #[tokio::test]
    async fn command() {
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
        let update = create_service_update(update);
        assert!(Update::from_update(update.clone()).await.unwrap().is_some());
        assert!(Command::from_update(update).await.unwrap().is_some());
    }

    #[tokio::test]
    async fn inline_query() {
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
        let update = create_service_update(update);
        assert!(Update::from_update(update.clone()).await.unwrap().is_some());
        assert!(InlineQuery::from_update(update).await.unwrap().is_some());
    }

    #[tokio::test]
    async fn chosen_inline_result() {
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
        let update = create_service_update(update);
        assert!(Update::from_update(update.clone()).await.unwrap().is_some());
        assert!(ChosenInlineResult::from_update(update).await.unwrap().is_some());
    }

    #[tokio::test]
    async fn callback_query() {
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
        let update = create_service_update(update);
        assert!(Update::from_update(update.clone()).await.unwrap().is_some());
        assert!(CallbackQuery::from_update(update).await.unwrap().is_some());
    }

    #[tokio::test]
    async fn shipping_query() {
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
        let update = create_service_update(update);
        assert!(Update::from_update(update.clone()).await.unwrap().is_some());
        assert!(ShippingQuery::from_update(update).await.unwrap().is_some());
    }

    #[tokio::test]
    async fn pre_checkout_query() {
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
        let update = create_service_update(update);
        assert!(Update::from_update(update.clone()).await.unwrap().is_some());
        assert!(PreCheckoutQuery::from_update(update).await.unwrap().is_some());
    }

    #[tokio::test]
    async fn poll() {
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
        let update = create_service_update(update);
        assert!(Update::from_update(update.clone()).await.unwrap().is_some());
        assert!(Poll::from_update(update).await.unwrap().is_some());
    }

    #[tokio::test]
    async fn poll_answer() {
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
        let update = create_service_update(update);
        assert!(Update::from_update(update.clone()).await.unwrap().is_some());
        assert!(PollAnswer::from_update(update).await.unwrap().is_some());
    }
}
