use crate::context::Context;
use std::{marker::PhantomData, ops::Deref};
use tgbot::types::Update;

mod command;
mod future;
mod text;
mod update;

pub use self::{command::*, future::*, text::*, update::*};

/// An update handler
///
/// Allows to handle specific kinds of update or update itself
pub trait Handler {
    /// An object to handle (update, message, inline query, etc...)
    ///
    /// See [FromUpdate](trait.FromUpdate.html) for more information
    type Input: FromUpdate;

    /// A result or future to return
    ///
    /// See [HandlerFuture](struct.HandlerFuture.html) for more information
    type Output: Into<HandlerFuture>;

    /// Handles an object
    ///
    /// # Arguments
    ///
    /// * context - A handler context
    /// * input - An object obtained from update (update itself, message, etc...)
    ///
    /// The context is valid for all handlers that accepts given update
    ///
    /// When processing a subsequent update, the old context will be destroyed and replaced with a new one
    fn handle(&self, context: &mut Context, input: Self::Input) -> Self::Output;
}

pub(crate) type BoxedHandler = Box<Handler<Input = Update, Output = HandlerFuture> + Send + Sync + 'static>;

impl<H, I, R> Handler for Box<H>
where
    H: Handler<Input = I, Output = R> + Sized,
    I: FromUpdate,
    R: Into<HandlerFuture>,
{
    type Input = I;
    type Output = R;

    fn handle(&self, context: &mut Context, input: Self::Input) -> Self::Output {
        self.deref().handle(context, input)
    }
}

pub(crate) struct HandlerWrapper<H> {
    handler: H,
}

impl<H> HandlerWrapper<H> {
    pub fn boxed(handler: H) -> Box<Self> {
        Box::new(Self { handler })
    }
}

impl<H, I, O> Handler for HandlerWrapper<H>
where
    H: Handler<Input = I, Output = O>,
    I: FromUpdate,
    O: Into<HandlerFuture>,
{
    type Input = Update;
    type Output = HandlerFuture;

    fn handle(&self, context: &mut Context, input: Self::Input) -> Self::Output {
        match I::from_update(input) {
            Some(input) => self.handler.handle(context, input).into(),
            _ => HandlerResult::Continue.into(),
        }
    }
}

/// A function handler
///
/// Since we can not implement a handler for Fn directly,
/// we use this struct as a workaround
///
/// # Example
///
/// ```
/// use carapax::prelude::*;
///
/// fn update_handler(_context: &mut Context, _update: Update) {}
///
/// fn main() {
///     FnHandler::from(update_handler);
/// }
/// ```
pub struct FnHandler<F, I, O>
where
    F: Fn(&mut Context, I) -> O,
    I: FromUpdate,
    O: Into<HandlerFuture>,
{
    f: F,
    _input: PhantomData<I>,
}

impl<F, I, O> From<F> for FnHandler<F, I, O>
where
    F: Fn(&mut Context, I) -> O,
    I: FromUpdate,
    O: Into<HandlerFuture>,
{
    fn from(f: F) -> Self {
        Self { f, _input: PhantomData }
    }
}

impl<F, I, O> Handler for FnHandler<F, I, O>
where
    F: Fn(&mut Context, I) -> O,
    I: FromUpdate,
    O: Into<HandlerFuture>,
{
    type Input = I;
    type Output = O;

    fn handle(&self, context: &mut Context, input: Self::Input) -> Self::Output {
        (self.f)(context, input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::Future;
    use tgbot::types::Message;

    fn handle_message(_context: &mut Context, _message: Message) -> HandlerResult {
        HandlerResult::Stop
    }

    #[test]
    fn handler() {
        // message handler accepts message
        let mut context = Context::default();
        let handler = HandlerWrapper::boxed(FnHandler::from(handle_message));
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "message": {
                "message_id": 1111,
                "date": 0,
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "chat": {"id": 1, "type": "private", "first_name": "test"},
                "text": "test message from private chat"
            }
        }))
        .unwrap();
        assert_eq!(
            handler.handle(&mut context, update).wait().unwrap(),
            HandlerResult::Stop
        );

        // message handler does not accept inline query
        let update: Update = serde_json::from_value(serde_json::json!({
            "update_id": 1,
            "inline_query": {
                "id": "id",
                "from": {"id": 1, "is_bot": false, "first_name": "test"},
                "query": "query",
                "offset": "offset"
            }
        }))
        .unwrap();
        assert_eq!(
            handler.handle(&mut context, update).wait().unwrap(),
            HandlerResult::Continue
        );
    }
}
